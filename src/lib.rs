pub struct CountMinSketch {
    width: usize,
    depth: usize,
    count: Vec<Vec<u32>>,
}

impl CountMinSketch {
    pub fn new(width: usize, depth: usize) -> Self {
        Self {
            width,
            depth,
            count: vec![vec![0; width]; depth],
        }
    }

    pub fn add(&mut self, item: &[u8]) {
        for i in 0..self.depth {
            *self.at_mut(i, item) += 1;
        }
    }

    pub fn query(&self, item: &[u8]) -> u32 {
        (0..self.depth).map(|i| self.at(i, item)).min().unwrap()
    }

    fn at(&self, i: usize, item: &[u8]) -> u32 {
        self.count[i][self.hash_index(item, i)]
    }

    fn at_mut(&mut self, i: usize, item: &[u8]) -> &mut u32 {
        let j = self.hash_index(item, i);
        &mut self.count[i][j]
    }

    fn hash_index(&self, item: &[u8], seed: usize) -> usize {
        (murmurhash3::murmurhash3_x86_32(item, seed as u32) as usize) % self.width
    }
}
