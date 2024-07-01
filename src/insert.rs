use crate::Caravela;
use crate::Encode;

pub trait Insert {
    fn _insert(&mut self, point: Vec<f32>);
}

impl Insert for Caravela {
    fn _insert(&mut self, point: Vec<f32>) {
        let rank = self._encode(&point);
        self.data.push(point); //push point to data
                               // Find the position to insert the new key
        match self.rank_array.binary_search(&rank) {
            Ok(_) => (),
            // Insert the key into the array if its not there already
            Err(pos) => {
                self.rank_array.insert(pos, rank);
            }
        };
        // Update the map
        self.index_map
            .entry(rank)
            .and_modify(|v| v.push(self.index))
            .or_insert_with(|| vec![self.index]);
        self.index += 1;
    }
}
