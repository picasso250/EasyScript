//! gc.rs
//! This module implements a simple Mark-and-Sweep Stop-the-World garbage collector for EasyScript.

use std::alloc::{self, Layout};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::ptr::NonNull; // For safe raw pointers
use std::rc::Rc; // For NativeFunction's internal Rc // For custom allocation

use crate::ast::Block; // For FunctionObjectInner
use crate::environment::EnvironmentRef; // For FunctionObjectInner
                                        // NOTE: This recursive use is fine, as self::Value refers to the Value struct defined below.
                                        // It's used within Object::List and Object::Map

// --- Type Aliases for Function Objects ---
/// Defines the signature for a native Rust function that can be called from EasyScript.
pub type NativeFunction =
    Rc<dyn for<'a> Fn(&'a mut Heap, &EnvironmentRef, Vec<Value>) -> Result<Value, String>>;

/// Represents a user-defined or native function in EasyScript.
#[derive(Clone)]
pub enum FunctionObjectInner {
    /// A native Rust function (e.g., `print`).
    Native(NativeFunction),
    /// A user-defined function written in EasyScript.
    User {
        params: Vec<String>,
        body: Rc<Block>,             // Function body is an AST Block
        defined_env: EnvironmentRef, // Closure environment
    },
}

// Manual Debug implementation for FunctionObjectInner because dyn Fn does not implement Debug.
impl fmt::Debug for FunctionObjectInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionObjectInner::Native(_) => write!(f, "NativeFunction"),
            FunctionObjectInner::User {
                params,
                body: _,
                defined_env: _,
            } => {
                write!(f, "UserFunction {{ params: {:?} }}", params)
            }
        }
    }
}

/// Represents a method bound to a specific receiver object.
#[derive(Debug, Clone)]
pub struct BoundMethodInner {
    pub receiver: Value, // The object (self) to which the method is bound
    pub method_name: String, // The name of the method (e.g., "push", "len")
                         // Note: The actual NativeFunction is looked up at call time based on method_name
}

// --- 1. GcRef Handle ---
/// A smart pointer representing a handle to a garbage-collected object on the heap.
///
/// This type replaces `Rc<T>` for types that need to be managed by the GC.
/// It wraps a raw pointer for efficiency and `Clone` semantics.
/// `GcRef` specifically points to `GcObjectHeader` which is then followed by `Object`.
#[derive(Debug, Clone, Copy)] // Copy is fine as it's just a pointer.
pub struct GcRef {
    ptr: NonNull<GcObjectHeader>,
}

// GcRef handles are equal if they point to the same object
impl PartialEq for GcRef {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
impl Eq for GcRef {}

impl std::hash::Hash for GcRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl GcRef {
    /// Internal method to create a GcRef handle from a raw pointer to a GcObjectHeader.
    /// This is `unsafe` because the caller must guarantee the pointer is valid.
    pub unsafe fn from_raw(ptr: NonNull<GcObjectHeader>) -> Self {
        GcRef { ptr }
    }

    /// Dereferences the GcRef handle to get an immutable reference to the managed `Object`.
    pub fn deref(&self) -> &Object {
        unsafe {
            let header_ptr = self.ptr.as_ptr();

            let payload_layout = Layout::new::<Object>();
            let (_, data_offset) = Layout::new::<GcObjectHeader>()
                .extend(payload_layout)
                .unwrap();

            let obj_data_ptr = (header_ptr as *mut u8).add(data_offset) as *mut Object;
            obj_data_ptr.as_ref().unwrap()
        }
    }

    /// Dereferences the GcRef handle to get a mutable reference to the managed `Object`.
    /// This is `unsafe` because the caller must guarantee no other mutable references exist
    /// to this object (GC ensures this during Stop-the-World phases).
    pub fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            let header_ptr = self.ptr.as_ptr();

            let payload_layout = Layout::new::<Object>();
            let (_, data_offset) = Layout::new::<GcObjectHeader>()
                .extend(payload_layout)
                .unwrap();

            let obj_data_ptr = (header_ptr as *mut u8).add(data_offset) as *mut Object;
            obj_data_ptr.as_mut().unwrap()
        }
    }
}

// --- 2. GcObjectHeader and Trait ---
/// Header for every object allocated on the GC heap.
/// This structure ensures proper alignment for the Object that follows it.
#[repr(C)] // Force specific memory layout for header followed by payload
pub struct GcObjectHeader {
    pub marked: RefCell<bool>, // The 'color' field: Whether the object is marked as reachable during GC cycle
    pub obj_type: GcObjectType, // Type of the Object (Number, List, Map, etc.)
                               // Data for the actual object (Object) follows this header in memory
}

/// Enum to identify the type of Object.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GcObjectType {
    Number,
    Boolean,
    String,
    Nil,
    List,
    Map,
    Function,
    BoundMethod,
    // ... potentially other GC'd types
}

/// The actual data for all EasyScript runtime values.
/// Instances of this enum are always allocated on the GC heap.
/// This struct must be `Sized` and have a known layout for `Heap::allocate`.
#[derive(Debug, Clone)] // Clone for deep copying, though GC manages lifetimes
pub enum Object {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    List(Vec<Value>),              // Contains Value handles
    Map(HashMap<Value, Value>),    // Keys and values are Value handles
    Function(FunctionObjectInner), // User-defined or native functions
    BoundMethod(BoundMethodInner), // Method bound to a receiver
}

// Convert Object variant to GcObjectType for the header
impl From<Object> for GcObjectType {
    fn from(payload: Object) -> Self {
        match payload {
            Object::Number(_) => GcObjectType::Number,
            Object::Boolean(_) => GcObjectType::Boolean,
            Object::String(_) => GcObjectType::String,
            Object::Nil => GcObjectType::Nil,
            Object::List(_) => GcObjectType::List,
            Object::Map(_) => GcObjectType::Map,
            Object::Function(_) => GcObjectType::Function,
            Object::BoundMethod(_) => GcObjectType::BoundMethod,
        }
    }
}

/// Trait for types that can be managed by the GC and can be traced.
///
/// Every type that can live on the GC heap, or contain `GcRef` references,
/// must implement this trait so the GC knows how to traverse the object graph.
pub trait GcTrace {
    /// Marks this object as reachable and recursively marks all objects it refers to.
    fn trace(&self, heap: &Heap);
}

// Implement Display for Object for printing EasyScript values
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Nil => write!(f, "nil"),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Number(n) => {
                // Handle integer display without .0
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Object::String(s) => write!(f, "{}", s), // No quotes
            Object::List(list) => {
                write!(f, "[")?;
                let mut first = true;
                for item in list.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    if let Object::String(s) = item.0.deref() {
                        write!(f, "{:?}", s)?; // String elements within lists should be quoted
                    } else {
                        write!(f, "{}", item)?; // Other types use their normal Display
                    }
                    first = false;
                }
                write!(f, "]")
            }
            Object::Map(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, val) in map.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    if let Object::String(s) = key.0.deref() {
                        write!(f, "{:?}", s)?; // String keys within maps should be quoted
                    } else {
                        write!(f, "{}", key)?; // Other key types use their normal Display
                    }
                    write!(f, ": ")?;
                    if let Object::String(s) = val.0.deref() {
                        write!(f, "{:?}", s)?; // String values within maps should be quoted
                    } else {
                        write!(f, "{}", val)?; // Other value types use their normal Display
                    }
                    first = false;
                }
                write!(f, "}}")
            }
            Object::Function(_) => write!(f, "<function>"),
            Object::BoundMethod(_) => write!(f, "<bound method>"),
        }
    }
}

// Implement GcTrace for Object
impl GcTrace for Object {
    fn trace(&self, heap: &Heap) {
        // This method only needs to recursively trace its children.
        match self {
            Object::List(list) => {
                for item in list {
                    item.trace(heap); // Recursively trace the Value handles
                }
            }
            Object::Map(map) => {
                for (key, val) in map {
                    key.trace(heap); // Trace keys
                    val.trace(heap); // Trace values
                }
            }
            Object::Function(func_inner) => {
                if let FunctionObjectInner::User { defined_env, .. } = func_inner {
                    // A closure roots all values in its captured environment. We must trace them.
                    let mut current_env = Some(Rc::clone(defined_env));
                    while let Some(env_ref) = current_env {
                        let env_borrow = env_ref.borrow();
                        for value in env_borrow.values.values() {
                            value.trace(heap);
                        }
                        current_env = env_borrow.parent.as_ref().map(Rc::clone);
                    }
                }
            }
            Object::BoundMethod(bound_method_inner) => {
                bound_method_inner.receiver.trace(heap); // Trace the receiver
            }
            _ => { /* Primitives (Number, Boolean, String, Nil) do not contain GcRef */ }
        }
    }
}

// Helper methods for Object to safely access internal data
impl Object {
    pub fn as_number(&self) -> Option<&f64> {
        if let Object::Number(n) = self {
            Some(n)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        if let Object::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        if let Object::Boolean(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        if let Object::List(l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<Value, Value>> {
        if let Object::Map(m) = self {
            Some(m)
        } else {
            None
        }
    }

    pub fn as_bound_method(&self) -> Option<&BoundMethodInner> {
        if let Object::BoundMethod(bm) = self {
            Some(bm)
        } else {
            None
        }
    }
}

// Implement PartialEq for Object for Map keys
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Nil, Object::Nil) => true,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::List(a), Object::List(b)) => a == b, // Structural comparison
            (Object::Map(a), Object::Map(b)) => a == b,   // Structural comparison
            (Object::Function(_a), Object::Function(_b)) => {
                // For function equality, we can compare their internal representation
                // or simply return false for now if not identical native functions.
                // For user functions, comparing params, body (Rc<Block> might be tricky for Rc) and defined_env
                // For simplicity, let's assume functions are equal only if they are the exact same Rc.
                // However, GcRef are compared by pointer equality.
                // So, for FunctionObjectInner, we need to compare their content if user function.
                // For native function, Rc<dyn Fn> cannot be compared.
                // Let's implement this as identity for now, or false if not the exact same.
                // For the first version, returning false is a safe default for complex types
                // that aren't strictly comparable by value.
                false // Or based on identity if we store a unique ID
            }
            (Object::BoundMethod(a), Object::BoundMethod(b)) => {
                // Compare receiver and method_name
                a.receiver == b.receiver && a.method_name == b.method_name
            }
            _ => false, // Different enum variants are not equal
        }
    }
}

impl Eq for Object {}

// Implement Hash for Object for Map keys
impl std::hash::Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Object::Nil => 0.hash(state),
            Object::Boolean(b) => b.hash(state),
            Object::Number(n) => n.to_bits().hash(state),
            Object::String(s) => s.hash(state),
            // List, Map, Function, BoundMethod can't be used as HashMap keys (by Rust's default Hash)
            // or require more complex structural hashing that can lead to cycles.
            // For now, these will panic if used as keys. A robust solution needs identity hashing
            // for these types as map keys.
            Object::List(_) => panic!("List values cannot be used as HashMap keys"),
            Object::Map(_) => panic!("Map values cannot be used as HashMap keys"),
            Object::Function(_) => panic!("Function values cannot be used as HashMap keys"),
            Object::BoundMethod(_) => {
                panic!("BoundMethod values cannot be used as HashMap keys")
            }
        }
    }
}

// --- 3. Heap Manager ---
/// The global Garbage Collector heap.
///
/// This manages memory allocation and deallocation for `GcRef` objects.
/// For the first version, this will be a simple `Vec` of `NonNull<GcObjectHeader>`
/// storing raw pointers to allocated memory blocks.
pub struct Heap {
    // Stores raw pointers to the GcObjectHeader of all allocated objects.
    objects: Vec<NonNull<GcObjectHeader>>,
    // We also need to keep track of the roots for the GC cycle.
    // This will be provided to the `collect` method for now.
}

impl Heap {
    /// Creates a new, empty GC heap.
    pub fn new() -> Self {
        Heap {
            objects: Vec::new(),
        }
    }

    /// Allocates a new Object on the GC heap.
    /// This is `unsafe` because it involves raw memory allocation and pointer casting.
    pub unsafe fn allocate(&mut self, payload: Object) -> GcRef {
        let obj_type: GcObjectType = payload.clone().into(); // Get GcObjectType from payload (needs Clone for payload)
        let payload_layout = Layout::new::<Object>();

        // Calculate layout for Header + Payload
        let (layout, data_offset) = Layout::new::<GcObjectHeader>()
            .extend(payload_layout)
            .unwrap();

        // Allocate raw memory
        let ptr = alloc::alloc(layout) as *mut GcObjectHeader;
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        // Write header
        ptr.write(GcObjectHeader {
            marked: RefCell::new(false),
            obj_type,
        });

        // Write object data (payload) after the header
        let obj_data_ptr = (ptr as *mut u8).add(data_offset) as *mut Object;
        obj_data_ptr.write(payload);

        let gc_handle = GcRef::from_raw(NonNull::new_unchecked(ptr));
        self.objects.push(NonNull::new_unchecked(ptr)); // Keep track of all allocated objects
        gc_handle
    }

    /// Allocates a Nil object on the GC heap.
    pub fn allocate_nil(&mut self) -> Value {
        Value(unsafe { self.allocate(Object::Nil) })
    }

    /// Allocates a Boolean object on the GC heap.
    pub fn allocate_boolean(&mut self, b: bool) -> Value {
        Value(unsafe { self.allocate(Object::Boolean(b)) })
    }

    /// Allocates a Number object on the GC heap.
    pub fn allocate_number(&mut self, n: f64) -> Value {
        Value(unsafe { self.allocate(Object::Number(n)) })
    }

    /// Allocates a String object on the GC heap.
    pub fn allocate_string(&mut self, s: String) -> Value {
        Value(unsafe { self.allocate(Object::String(s)) })
    }

    /// Allocates a List object on the GC heap.
    pub fn allocate_list(&mut self, l: Vec<Value>) -> Value {
        Value(unsafe { self.allocate(Object::List(l)) })
    }

    /// Allocates a Map object on the GC heap.
    pub fn allocate_map(&mut self, m: HashMap<Value, Value>) -> Value {
        Value(unsafe { self.allocate(Object::Map(m)) })
    }

    /// Allocates a Function object on the GC heap.
    pub fn allocate_function(&mut self, f: FunctionObjectInner) -> Value {
        Value(unsafe { self.allocate(Object::Function(f)) })
    }

    /// Allocates a BoundMethod object on the GC heap.
    pub fn allocate_bound_method(&mut self, bm: BoundMethodInner) -> Value {
        Value(unsafe { self.allocate(Object::BoundMethod(bm)) })
    }

    /// Triggers a garbage collection cycle. (Stop-the-World Mark-and-Sweep)
    /// `roots` are the starting points for tracing reachable objects.
    pub fn collect(&mut self, roots: &[Value]) {
        eprintln!(
            "[GC] Starting collection phase. {} objects on heap.",
            self.objects.len()
        );
        // 1. Mark Phase:
        //    Reset all mark bits to false for the current sweep cycle.
        self.unmark_all();
        //    Trace from roots. Each Value is a GcRef<Object>, so we trace its payload.
        for root in roots {
            root.trace(self); // Call the GcTrace for Value
        }

        // 2. Sweep Phase:
        self.sweep();
    }

    /// Resets the mark bit for all objects on the heap to `false`.
    fn unmark_all(&self) {
        for &ptr in &self.objects {
            unsafe {
                ptr.as_ref().marked.replace(false);
            }
        }
    }

    /// Sweeps through the heap, freeing unmarked objects.
    fn sweep(&mut self) {
        let before_count = self.objects.len();

        self.objects.retain(|&ptr| {
            unsafe {
                // Get a reference to the header
                let header = ptr.as_ref();
                if *header.marked.borrow() == false {
                    // Object is not marked, so it's garbage. Deallocate.
                    let payload_layout = Self::layout_for_type(header.obj_type); // Get payload layout

                    let (layout, data_offset) = Layout::new::<GcObjectHeader>()
                        .extend(payload_layout)
                        .unwrap();

                    // Call Drop for the payload before deallocating memory
                    let obj_data_ptr = (ptr.as_ptr() as *mut u8).add(data_offset) as *mut Object;
                    std::ptr::drop_in_place(obj_data_ptr); // Explicitly call drop

                    alloc::dealloc(ptr.as_ptr() as *mut u8, layout);
                    false // Remove from objects vector
                } else {
                    true // Keep this object
                }
            }
        });

        let after_count = self.objects.len();
        if before_count > after_count {
            eprintln!(
                "[GC] Swept and freed {} objects. {} remaining.",
                before_count - after_count,
                after_count
            );
        }
    }

    /// Helper to get layout for Object variants (needed for deallocation)
    unsafe fn layout_for_type(obj_type: GcObjectType) -> Layout {
        match obj_type {
            GcObjectType::Number => Layout::new::<f64>(),
            GcObjectType::Boolean => Layout::new::<bool>(),
            GcObjectType::String => Layout::new::<String>(),
            GcObjectType::Nil => Layout::new::<()>(), // Nil has no data in Object variant
            GcObjectType::List => Layout::new::<Vec<Value>>(),
            GcObjectType::Map => Layout::new::<HashMap<Value, Value>>(),
            GcObjectType::Function => Layout::new::<FunctionObjectInner>(),
            GcObjectType::BoundMethod => Layout::new::<BoundMethodInner>(),
        }
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        // Ensure all allocated memory is freed when the Heap is dropped.
        for &ptr in &self.objects {
            unsafe {
                let header = ptr.as_ref();
                let payload_layout = Self::layout_for_type(header.obj_type);

                let (layout, data_offset) = Layout::new::<GcObjectHeader>()
                    .extend(payload_layout)
                    .unwrap();

                // Call Drop for the payload before deallocating memory
                let obj_data_ptr = (ptr.as_ptr() as *mut u8).add(data_offset) as *mut Object;
                std::ptr::drop_in_place(obj_data_ptr); // Explicitly call drop

                alloc::dealloc(ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

// EasyScript 核心运行时值类型，一切皆 Value。
// Value 现在是一个轻量级的结构体，包装了一个指向 GC 堆上 Object 的句柄。
#[derive(Debug, Clone)] // GcRef 句柄需要实现这些 Trait
pub struct Value(pub GcRef);

// Implement GcTrace for Value
impl GcTrace for Value {
    fn trace(&self, heap: &Heap) {
        unsafe {
            let header = self.0.ptr.as_ref(); // Get immutable ref to header
            if *header.marked.borrow() {
                // Check if already marked
                return;
            }
            header.marked.replace(true); // Mark as reachable
            self.0.deref().trace(heap); // Delegate tracing to the payload
        }
    }
}

// Helper for nil, true, false, etc.
impl Value {
    pub fn nil(heap: &mut Heap) -> Value {
        heap.allocate_nil()
    }

    pub fn boolean(heap: &mut Heap, b: bool) -> Value {
        heap.allocate_boolean(b)
    }

    pub fn number(heap: &mut Heap, n: f64) -> Value {
        heap.allocate_number(n)
    }

    pub fn string(heap: &mut Heap, s: String) -> Value {
        heap.allocate_string(s)
    }

    pub fn list(heap: &mut Heap, l: Vec<Value>) -> Value {
        heap.allocate_list(l)
    }

    pub fn map(heap: &mut Heap, m: HashMap<Value, Value>) -> Value {
        heap.allocate_map(m)
    }

    pub fn function(heap: &mut Heap, f: FunctionObjectInner) -> Value {
        heap.allocate_function(f)
    }

    pub fn bound_method(heap: &mut Heap, bm: BoundMethodInner) -> Value {
        heap.allocate_bound_method(bm)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the Object's Display implementation
        // This will need Object to implement Display
        self.0.deref().fmt(f)
    }
}

impl Value {
    pub fn type_of(&self) -> &'static str {
        match self.0.deref() {
            Object::Nil => "nil",
            Object::Boolean(_) => "boolean",
            Object::Number(_) => "number",
            Object::String(_) => "string",
            Object::List(_) => "list",
            Object::Map(_) => "map",
            Object::Function(_) => "function",
            Object::BoundMethod(_) => "method",
        }
    }

    /// Determines the truthiness of a value based on EasyScript's rules.
    /// Falsy values are: nil, false, 0, "", [], and {}.
    /// All other values are truthy.
    pub fn is_truthy(&self) -> bool {
        match self.0.deref() {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            Object::Number(n) => *n != 0.0,
            Object::String(s) => !s.is_empty(),
            Object::List(l) => !l.is_empty(),
            Object::Map(m) => !m.is_empty(),
            Object::Function(_) => true,
            Object::BoundMethod(_) => true, // Bound methods are always truthy
        }
    }

    /// Returns a Python-like developer-friendly representation of the value (repr).
    pub fn repr_string(&self) -> String {
        match self.0.deref() {
            Object::Nil => "nil".to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Number(n) => {
                if n.fract() == 0.0 {
                    (*n as i64).to_string()
                } else {
                    n.to_string()
                }
            }
            Object::String(s) => format!("{:?}", s), // Explicitly quote strings for repr
            Object::List(list) => {
                let elements: Vec<String> = list.iter().map(|item| item.repr_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Object::Map(map) => {
                let pairs: Vec<String> = map
                    .iter()
                    .map(|(key, val)| format!("{}: {}", key.repr_string(), val.repr_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
            Object::Function(_) => "<function>".to_string(),
            Object::BoundMethod(_) => "<bound method>".to_string(),
        }
    }
}

// GcRef 句柄已经实现了 PartialEq, Eq, Hash，因此 Value 只需要派生这些特性。
// 不需要再手动实现。

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        // Delegate to the Object's PartialEq implementation
        self.0.deref() == other.0.deref()
    }
}

impl Eq for Value {} // Manually implement Eq

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}
