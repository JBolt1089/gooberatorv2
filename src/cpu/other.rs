use super::*;

impl Cpu {
    pub fn end(&mut self) {
        self.stopped = true;
    }
}
