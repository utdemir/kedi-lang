use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KediValue {
    KediNum(num_bigint::BigInt),
}

impl KediValue {
    pub fn num<T: Into<num_bigint::BigInt>>(n: T) -> Self {
        KediValue::KediNum(n.into())
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            KediValue::KediNum(n) => *n != BigInt::ZERO,
        }
    }
}

impl<T: Into<num_bigint::BigInt>> From<T> for KediValue {
    fn from(n: T) -> Self {
        KediValue::num(n)
    }
}
