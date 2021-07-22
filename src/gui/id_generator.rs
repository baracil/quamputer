#[derive(Default)]
pub struct IdGenerator {
    id:u32
}

impl IdGenerator {
    pub fn get_and_increment(&mut self) -> u32 {
        let result = self.id;
        self.id += 1;
        result
    }
}
