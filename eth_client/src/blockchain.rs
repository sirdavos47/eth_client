use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, String::from("0x0"), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.number + 1,
            last_block.hash.clone(),
            data,
        );
        self.chain.push(new_block);
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];
            let curr = &self.chain[i];
            if curr.previous_hash != prev.hash {
                return false;
            }
        }
        true
    }
}
