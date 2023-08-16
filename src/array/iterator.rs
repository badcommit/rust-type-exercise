use super::Array;

pub struct ArrayIterator<'a, T: Array> {
    array: &'a T,
    pos: usize
}


impl <'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.array.len() {
            None
        } else {
            let item = self.array.get(self.pos);
            self.pos += 1;
            Some(item)
        }
    }


}

impl <'a, A: Array> ArrayIterator<'a, A> {
    pub fn new(array:&'a A) -> Self {
        Self {
            array,
            pos: 0
        }
    }
}
