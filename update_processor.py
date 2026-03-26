import re

with open('thunderfpr/crates/thunderforge-plugin/src/processor.rs', 'r') as f:
    content = f.read()

# Replace allocations with slice mutations
content = content.replace("let mut work_l = input_left.to_vec();", "let mut work_l = input_left;")
content = content.replace("let mut work_r = input_right.to_vec();", "let mut work_r = input_right;")

with open('thunderfpr/crates/thunderforge-plugin/src/processor.rs', 'w') as f:
    f.write(content)

print("Updated processor.rs")
