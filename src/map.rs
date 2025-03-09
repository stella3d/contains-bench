use smallvec::SmallVec;

pub struct SmallSortedMap<K: PartialEq + Eq + Ord + Copy, V: Copy> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> SmallSortedMap<K, V>
where
    K: PartialEq + Eq + Ord + Copy,
    V: Copy,
{
    pub fn with_capacity(capacity: u8) -> Self {
        Self {
            keys: Vec::with_capacity(capacity as usize),
            values: Vec::with_capacity(capacity as usize),
        }
    }

    pub fn from_vecs(keys: Vec<K>, values: Vec<V>) -> Self {
        let mut s = Self {
            keys,
            values,
        };

        s.sort();
        s
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(index) = self.keys.iter().position(|k| *k == key) {
            self.values[index] = value;
        } else {
            self.keys.push(key);
            self.values.push(value);
        }
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<V> {
        let index = self.keys.binary_search(key).ok()?;
        Some(self.values[index])
    }

    /// this must be called before any read operations
    pub fn sort(&mut self) {
        let mut indices: Vec<usize> = (0..self.keys.len()).collect();
        indices.sort_by(|&a, &b| self.keys[a].cmp(&self.keys[b]));

        let mut sorted_keys = Vec::with_capacity(self.keys.len());
        let mut sorted_values = Vec::with_capacity(self.values.len());

        for &index in &indices {
            sorted_keys.push(self.keys[index]);
            sorted_values.push(self.values[index]);
        }

        self.keys = sorted_keys;
        self.values = sorted_values;
    }
}


pub struct PicoSortedMap<K: PartialEq + Eq + Ord + Copy, V: Copy, const N: usize> {
    keys: SmallVec<[K; N]>,
    values: SmallVec<[V; N]>,
}

impl <K, V, const N: usize> PicoSortedMap<K, V, N>
where
    K: PartialEq + Eq + Ord + Copy,
    V: Copy,
{
    pub fn with_capacity() -> Self {
        Self {
            keys: SmallVec::new(),
            values: SmallVec::new(),
        }
    }

    pub fn from_vecs(keys: Vec<K>, values: Vec<V>) -> Self {
        let mut s = Self {
            keys: SmallVec::from_vec(keys),
            values: SmallVec::from_vec(values),
        };

        s.sort();
        s
    }

    pub fn sort(&mut self) {
        let mut indices: Vec<usize> = (0..self.keys.len()).collect();
        indices.sort_by(|&a, &b| self.keys[a].cmp(&self.keys[b]));

        let mut sorted_keys = SmallVec::<[K; N]>::new();
        let mut sorted_values = SmallVec::<[V; N]>::new();

        for &index in &indices {
            sorted_keys.push(self.keys[index]);
            sorted_values.push(self.values[index]);
        }

        self.keys = sorted_keys;
        self.values = sorted_values;
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<V> {
        let index = self.keys.binary_search(key).ok()?;
        Some(self.values[index])
    }
}