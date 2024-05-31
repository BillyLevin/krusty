pub struct TranspositionTable {
    entries: Vec<TranspositionTableEntry>,
    size: usize,
}

#[derive(Debug, Default, Clone)]
pub struct TranspositionTableEntry {
    pub hash: u64,
    pub node_count: u64,
    pub depth: u8,
}

impl TranspositionTableEntry {
    pub fn new(hash: u64, node_count: u64, depth: u8) -> Self {
        Self {
            hash,
            node_count,
            depth,
        }
    }
}

const MEGABYTE: usize = 1024 * 1024;

impl TranspositionTable {
    pub fn new(size_in_mb: usize) -> Self {
        let size = (size_in_mb * MEGABYTE) / std::mem::size_of::<TranspositionTableEntry>();

        Self {
            entries: vec![TranspositionTableEntry::default(); size],
            size,
        }
    }

    pub fn store(&mut self, entry: TranspositionTableEntry) {
        let index = self.get_index(entry.hash);
        self.entries[index] = entry;
    }

    pub fn probe(&self, hash: u64) -> &TranspositionTableEntry {
        let index = self.get_index(hash);
        &self.entries[index]
    }

    fn get_index(&self, hash: u64) -> usize {
        (hash as usize) % self.size
    }
}
