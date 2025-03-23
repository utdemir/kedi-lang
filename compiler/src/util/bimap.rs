use bimap::BiHashMap;
use sexpr::SExpr;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bimap<K, V>
where
    K: Eq + std::hash::Hash,
    V: Eq + std::hash::Hash,
{
    bimap: BiHashMap<K, V>,
}

impl<K, V> Bimap<K, V>
where
    K: Eq + std::hash::Hash,
    V: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            bimap: BiHashMap::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.bimap.insert(k, v);
    }

    pub fn get_by_left(&self, k: &K) -> Option<&V> {
        self.bimap.get_by_left(k)
    }

    pub fn get_by_right(&self, v: &V) -> Option<&K> {
        self.bimap.get_by_right(v)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.bimap.iter()
    }
}

impl<K, V> SExpr for Bimap<K, V>
where
    K: SExpr + Eq + std::hash::Hash,
    V: SExpr + Eq + std::hash::Hash,
{
    fn to_sexpr(&self) -> sexpr::SExprTerm {
        sexpr::SExprTerm::List(
            self.bimap
                .iter()
                .map(|(k, v)| sexpr::list(&[k.to_sexpr(), v.to_sexpr()]))
                .collect::<Vec<_>>(),
        )
    }
}

impl<K, V> FromIterator<(K, V)> for Bimap<K, V>
where
    K: Eq + std::hash::Hash,
    V: Eq + std::hash::Hash,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut bimap = BiHashMap::new();
        for (k, v) in iter {
            bimap.insert(k, v);
        }
        Bimap { bimap }
    }
}
