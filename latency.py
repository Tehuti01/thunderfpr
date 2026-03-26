with open("thunderfpr/crates/thunderforge-plugin/src/lib.rs", "r") as f:
    content = f.read()

impl_tail = """
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Process audio through the signal chain
        self.processor.process(buffer, &self.params);
        ProcessStatus::Normal
    }
}
"""

replacement = """
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Report latency (< 5ms at 128 samples is typically around 200 samples)
        // Adjust according to the IR partition size if any, or algorithmic delay
        context.set_latency_samples(128); // Placeholder latency report

        // Process audio through the signal chain
        self.processor.process(buffer, &self.params);
        ProcessStatus::Normal
    }
}
"""

content = content.replace(impl_tail, replacement)

with open("thunderfpr/crates/thunderforge-plugin/src/lib.rs", "w") as f:
    f.write(content)
