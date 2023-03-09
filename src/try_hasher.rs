
// fn extract_bits_at_indexes(indexes: &[u8], input: u128) -> u64 {
//     let mut result: u64 = 0;
//     for (i, &index) in indexes.iter().enumerate() {
//         if input & (1u128 << index) != 0 {
//             result |= 1 << i;
//         }
//     }
//     result
// }

pub struct IndexHasher<T>{
    index_list: Vec<u8>,
    map: nohash_hasher::IntMap<u64, T>
}

impl<T> IndexHasher<T> {
    pub fn new(index_list: Vec<u8>) -> IndexHasher<T> {
        IndexHasher {
            index_list,
            map: nohash_hasher::IntMap::default(),
        }
    }

    fn extract_bits_at_indexes(&self, input: u128) -> u64{
        let mut result: u64 = 0;
        for (i, &index) in self.index_list.iter().enumerate() {
            if input & (1u128 << index) != 0 {
                result |= 1 << i;
            }
        }
        result
    }

    pub fn insert(&mut self, input: u128, value: T) {
        let key = self.extract_bits_at_indexes(input);
        self.map.insert(key,value);
        //let index = self.map.entry(key).or_default();
        //index.push(value);
    }

    pub fn get(&self, input: u128) -> Option<&T> {
        let key = self.extract_bits_at_indexes(input);
        self.map.get(&key)
    }
    
   
}
