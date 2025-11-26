import os
import re

def replace_comments_in_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    modified_lines = []
    changed = False
    for line in lines:
        # Regex to match '//' comments, but not if they are inside a double-quoted string
        # This regex is a simplified one, assuming no escaped quotes or complex scenarios
        # It looks for '//' not preceded by an odd number of double quotes on the line
        # For simplicity in this context, we will directly replace // outside of quotes.
        # Given the .es files are simple, a direct replace is likely safe for single-line comments.

        # Heuristic: Replace // only if it's not inside "..."
        # This is a bit tricky with regex, simpler to assume // marks comments at the end of line or start
        
        # Priority 1: // expect:
        # If it's an expect comment, convert // to #
        if line.strip().startswith('// expect:'):
            new_line = line.replace('// expect:', '# expect:', 1)
            if new_line != line:
                changed = True
            modified_lines.append(new_line)
        # Priority 2: // expect_stdout:
        # If it's an expect_stdout comment, convert // to #
        elif line.strip().startswith('// expect_stdout:'):
            new_line = line.replace('// expect_stdout:', '# expect_stdout:', 1)
            if new_line != line:
                changed = True
            modified_lines.append(new_line)
        # Priority 3: other single line comments
        # This is the tricky part. For now, assuming simple cases where // is not inside strings.
        # A more robust solution would require a simple lexer or more complex regex.
        # For the given .es test files, simple replacement should be enough.
        # Let's replace // with # for general comments not starting with # expect:
        elif '//' in line and not line.strip().startswith('#'): # Ensure it's not already processed # expect
            # Check if // is inside a string literal "..."
            # This is a very basic check and might fail for complex cases like "str // str"
            parts = line.split('"')
            new_line_parts = []
            for i, part in enumerate(parts):
                if i % 2 == 0: # Outside a string
                    new_line_parts.append(part.replace('//', '#', 1)) # Replace only the first occurrence
                else: # Inside a string
                    new_line_parts.append(part)
            new_line = '"'.join(new_line_parts)

            if new_line != line:
                changed = True
            modified_lines.append(new_line)
        else:
            modified_lines.append(line)

    if changed:
        print(f"Modifying: {filepath}")
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(modified_lines)
    return changed

def main():
    e2e_dir = 'tests/e2e'
    if not os.path.isdir(e2e_dir):
        print(f"Error: Directory '{e2e_dir}' not found.")
        return

    total_files_changed = 0
    for root, _, files in os.walk(e2e_dir):
        for filename in files:
            if filename.endswith('.es'):
                filepath = os.path.join(root, filename)
                if replace_comments_in_file(filepath):
                    total_files_changed += 1

    print(f"\nProcessed {total_files_changed} EasyScript test files.")

if __name__ == '__main__':
    main()
