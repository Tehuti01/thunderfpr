import re

with open('thunderfpr/crates/thunderforge-plugin/src/processor.rs', 'r') as f:
    content = f.read()

content = content.replace("let mut work_l = input_left;", "let work_l = &mut channels[0];")
content = content.replace("let mut work_r = input_right;", "let mut fallback_r = &mut [];\n        let work_r = if num_channels > 1 { &mut channels[1] } else { &mut fallback_r };")
content = content.replace("let input_left = &channels[0];", "")
content = content.replace("let input_right = if num_channels > 1 { &channels[1] } else { &channels[0] };", "")
content = content.replace("for sample in &mut work_l {", "for sample in work_l.iter_mut() {")
content = content.replace("for sample in &mut work_r {", "for sample in work_r.iter_mut() {")
content = content.replace("out_channels[0].copy_from_slice(&work_l);", "")
content = content.replace("out_channels[1].copy_from_slice(&work_r);", "")
content = content.replace("let out_channels = buffer.as_slice();", "")

with open('thunderfpr/crates/thunderforge-plugin/src/processor.rs', 'w') as f:
    f.write(content)

print("Updated processor.rs iterators")
