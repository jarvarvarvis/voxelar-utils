use std::marker::PhantomData;

/// A utility type that keeps a value of `T` alive for the lifetime of a reference
/// to the type `Other`.
pub struct KeepAlive<'other, Other, T> {
    value: T,
    phantom: PhantomData<&'other Other>,
}

impl<'other, Other, T> KeepAlive<'other, Other, T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<'other, Other, T> std::ops::Deref for KeepAlive<'other, Other, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

