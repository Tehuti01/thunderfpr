import re

with open('thunderfpr/crates/thunderforge-plugin/src/processor.rs', 'r') as f:
    content = f.read()

content = content.replace("let work_l = &mut channels[0];", "let (work_l, rest) = channels.split_at_mut(1);\n        let mut work_l = &mut work_l[0];")
content = content.replace("let work_r = if num_channels > 1 { &mut channels[1] } else { &mut fallback_r };", "let mut work_r = if num_channels > 1 { &mut rest[0] } else { fallback_r };")

content = content.replace("self.noise_gate.process_buffer(&mut work_l)", "self.noise_gate.process_buffer(work_l)")
content = content.replace("self.noise_gate.process_buffer(&mut work_r)", "self.noise_gate.process_buffer(work_r)")

content = content.replace("self.overdrive.process_buffer(&mut work_l)", "self.overdrive.process_buffer(work_l)")
content = content.replace("self.overdrive.process_buffer(&mut work_r)", "self.overdrive.process_buffer(work_r)")

content = content.replace("self.compressor.process_buffer(&mut work_l)", "self.compressor.process_buffer(work_l)")
content = content.replace("self.compressor.process_buffer(&mut work_r)", "self.compressor.process_buffer(work_r)")

content = content.replace("self.tone_stack.process_buffer(&mut work_l)", "self.tone_stack.process_buffer(work_l)")
content = content.replace("self.tone_stack.process_buffer(&mut work_r)", "self.tone_stack.process_buffer(work_r)")

with open('thunderfpr/crates/thunderforge-plugin/src/processor.rs', 'w') as f:
    f.write(content)
