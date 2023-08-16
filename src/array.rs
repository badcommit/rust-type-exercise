mod iterator;
mod primitive_array;
mod string_array;

use iterator::ArrayIterator;
pub trait Array: Send + Sync + Sized + 'static{
    type Builder: ArrayBuilder<Array = Self>;

    type OwnedItem: 'static + std::fmt::Debug;

    type RefItem<'a>: Copy + std::fmt::Debug;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool { self.len() == 0}

    fn iter(&self) -> ArrayIterator<Self>;

}

pub trait ArrayBuilder {
    type Array: Array<Builder = Self>;

    fn with_capacity(size: usize) -> Self;

    fn push(&mut self, value : Option<<Self::Array as Array>::RefItem<'_>>);

    fn finish(self) -> Self::Array;
}