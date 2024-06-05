pub trait TableEntry {
    fn hash(&self) -> u64;
}

pub struct TranspositionTable<Entry>
where
    Entry: TableEntry + Default + Clone,
{
    entries: Vec<Entry>,
    size: usize,
}

#[derive(Debug, Default, Clone)]
pub struct PerftTableEntry {
    pub hash: u64,
    pub node_count: u64,
    pub depth: u8,
}

#[derive(Debug, Default, Clone)]
pub struct SearchTableEntry {
    pub hash: u64,
    pub depth: u8,
    pub score: i32,
}

impl PerftTableEntry {
    pub fn new(hash: u64, node_count: u64, depth: u8) -> Self {
        Self {
            hash,
            node_count,
            depth,
        }
    }
}

impl TableEntry for PerftTableEntry {
    fn hash(&self) -> u64 {
        self.hash
    }
}

impl SearchTableEntry {
    pub fn new(hash: u64, depth: u8, score: i32) -> Self {
        Self { hash, depth, score }
    }
}

impl TableEntry for SearchTableEntry {
    fn hash(&self) -> u64 {
        self.hash
    }
}

const MEGABYTE: usize = 1024 * 1024;

impl<Entry> TranspositionTable<Entry>
where
    Entry: TableEntry + Default + Clone,
{
    pub fn new(size_in_mb: usize) -> Self {
        let size = (size_in_mb * MEGABYTE) / std::mem::size_of::<Entry>();

        Self {
            entries: vec![Entry::default(); size],
            size,
        }
    }

    pub fn store(&mut self, entry: Entry) {
        let index = self.get_index(entry.hash());
        self.entries[index] = entry;
    }

    pub fn probe(&self, hash: u64) -> &Entry {
        let index = self.get_index(hash);
        &self.entries[index]
    }

    fn get_index(&self, hash: u64) -> usize {
        (hash as usize) % self.size
    }
}
