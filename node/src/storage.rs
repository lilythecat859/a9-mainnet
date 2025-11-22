use rocksdb::{DB, Options, ColumnFamilyDescriptor};

pub struct Store {
    db: DB,
}

impl Store {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_max_open_files(10_000);
        let db = DB::open(&opts, path).expect("rocksdb open");
        Store { db }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), rocksdb::Error> {
        self.db.put(key, value)
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, rocksdb::Error> {
        self.db.get(key)
    }

    /// Prune expired state (Mandelbrot interior)
    pub fn prune(&self, cutoff_epoch: u64) -> Result<usize, rocksdb::Error> {
        let mut count = 0;
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        for item in iter {
            let (k, _) = item?;
            let epoch = u64::from_be_bytes(k[0..8].try_into().unwrap());
            if epoch < cutoff_epoch {
                self.db.delete(&k)?;
                count += 1;
            }
        }
        Ok(count)
    }
}
