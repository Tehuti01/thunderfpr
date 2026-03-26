with open('thunderfpr/crates/thunderforge-plugin/src/processor.rs', 'r') as f:
    content = f.read()

content = content.replace("let mut fallback_r = &mut [];", "let mut fallback_r: &mut [f32] = &mut [];")

with open('thunderfpr/crates/thunderforge-plugin/src/processor.rs', 'w') as f:
    f.write(content)
