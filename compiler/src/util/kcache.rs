use lasso::{Rodeo, Spur};

pub struct KCache {
    strcache: Rodeo,
}

impl KCache {
    pub fn new() -> KCache {
        KCache {
            strcache: Rodeo::new(),
        }
    }

    pub fn intern(&mut self, input: &str) -> Spur {
        self.strcache.get_or_intern(input)
    }

    pub fn get(&self, key: Spur) -> &str {
        self.strcache.resolve(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kcache() {
        let mut cache = KCache::new();
        let s1 = cache.intern("hello");
        let s2 = cache.intern("world");
        let s3 = cache.intern("hello");
        assert_eq!(s1, s3);
        assert_ne!(s1, s2);
    }
}
