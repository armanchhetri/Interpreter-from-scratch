pub trait SuperIterator: Iterator {
    // type Item;
    // fn next(&mut self) -> Option<Self::Item>;
    fn prev(&mut self) -> Option<Self::Item>;
    fn peek(&self) -> Option<Self::Item>;
    fn peek_next(&self) -> Option<Self::Item>;
}
