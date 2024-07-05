use crate::encode::Encode;
use crate::Caravela;

pub trait Insert {
    fn _insert(&mut self, point: Vec<f32>);
}

impl Insert for Caravela {
    fn _insert(&mut self, point: Vec<f32>) {
        let code = self._encode(&point);
        self.codex.push(code);
        self.data.push(point);
    }
}
