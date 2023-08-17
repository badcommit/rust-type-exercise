mod iterator;
mod primitive_array;
mod string_array;

use iterator::ArrayIterator;

pub use string_array::*;
pub use primitive_array::*;

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

#[cfg(test)]
mod tests{
    use super::*;
    fn build_array_from_vec<A: Array>(items: &[Option<A::RefItem<'_>>]) -> A {
        let mut builder = A::Builder::with_capacity(items.len());
        for item in items {
            builder.push(*item);
        }
        builder.finish()
    }

    fn check_array_eq<'a, A:Array>(array: &'a A, vec: &[Option<A::RefItem<'a>>]) where A::RefItem<'a>: PartialEq {
        assert_eq!(array.len(), vec.len());
        for (i, item) in vec.iter().enumerate() {
            assert_eq!(array.get(i), *item);
        }
    }

    #[test]
    fn test_build_int32_array() {
        let array = build_array_from_vec::<I32Array>(&[Some(1), None, Some(3)]);
        check_array_eq(&array, &[Some(1), None, Some(3)]);
    }

    #[test]
    fn test_build_string_array(){
        let data = vec![Some("1"), Some("2"), Some("3"), None, Some("5"), Some("")];
        let array = build_array_from_vec::<StringArray>(&data[..]);
        check_array_eq(&array, &data[..]);
    }
}