with open("thunderfpr/crates/thunderforge-plugin/src/lib.rs", "r") as f:
    content = f.read()

content = content.replace("use std::sync::{Arc, Mutex};", "use std::sync::Arc;")
content = content.replace("processor: Arc<Mutex<ThunderforgeProcessor>>,", "processor: ThunderforgeProcessor,")
content = content.replace("let processor = Arc::new(Mutex::new(ThunderforgeProcessor::new(params.clone())));", "let processor = ThunderforgeProcessor::new(params.clone());")
content = content.replace("if let Ok(mut proc) = self.processor.lock() {\n            proc.prepare(\n                buffer_config.sample_rate,\n                buffer_config.max_buffer_size as usize,\n            );\n        }", "self.processor.prepare(buffer_config.sample_rate, buffer_config.max_buffer_size as usize);")
content = content.replace("if let Ok(mut proc) = self.processor.lock() {\n            proc.reset();\n        }", "self.processor.reset();")
content = content.replace("if let Ok(mut proc) = self.processor.lock() {\n            proc.process(buffer, &self.params);\n        }", "self.processor.process(buffer, &self.params);")

with open("thunderfpr/crates/thunderforge-plugin/src/lib.rs", "w") as f:
    f.write(content)
