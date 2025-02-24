use std::ops::Deref;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Ax<Attachment, Value> {
    pub a: Attachment,
    pub v: Value,
}

impl<_Attachment, Value> Deref for Ax<_Attachment, Value> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T: PartialEq> PartialEq for Ax<(), T> {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl<T: Eq> Eq for Ax<(), T> {}

pub fn ax<A, V>(a: A, v: V) -> Ax<A, V> {
    Ax { a, v }
}

impl<Attachment, Value> Ax<Attachment, Value> {
    pub fn new(a: Attachment, v: Value) -> Self {
        ax(a, v)
    }

    pub fn map<NewVal, F: FnOnce(Value) -> NewVal>(self, f: F) -> Ax<Attachment, NewVal> {
        ax(self.a, f(self.v))
    }

    pub fn map_a<NewAx, F: FnOnce(Attachment) -> NewAx>(self, f: F) -> Ax<NewAx, Value> {
        ax(f(self.a), self.v)
    }

    pub fn as_ref(&self) -> Ax<&Attachment, &Value> {
        ax(&self.a, &self.v)
    }
}

impl<Attachment: Clone, Value> Ax<&Attachment, Value> {
    pub fn clone_a(self) -> Ax<Attachment, Value> {
        ax(self.a.clone(), self.v)
    }
}

impl<Attachment, Value, Err> Ax<Attachment, Result<Value, Err>> {
    pub fn transpose(self) -> Result<Ax<Attachment, Value>, Err> {
        self.v.map(|v| ax(self.a, v))
    }
}

impl<Attachment, Value> Ax<Attachment, Option<Value>> {
    pub fn transpose(self) -> Option<Ax<Attachment, Value>> {
        self.v.map(|v| ax(self.a, v))
    }
}

trait HasAx<Attachment> {
    fn ax(&self) -> Attachment;
}
