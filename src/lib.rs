
pub enum ProcessResult<T> {
    /// Represents a value that is either mapped or the original value.
    Value(T),
    Values(Box<dyn Iterator<Item = T>>),
    Skip(usize),
}

pub struct Process<I, T, F> {
    #[doc(hidden)]
    __iter: Option<Box<dyn Iterator<Item = I>>>,
    iter: T,
    predicate: F
}

impl<I, T, F> Iterator for Process<I, T, F> where T: Iterator<Item = I>, F: FnMut(I) -> ProcessResult<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut it) = self.__iter {
            it.next()
        } else {
            match self.iter.next() {
                None => None,
                Some(v) => {
                    match (self.predicate)(v) {
                        ProcessResult::Value(it) => {
                            Some(it)
                        }
                        ProcessResult::Values(mut iter) => {
                            let item = iter.next();
                            self.__iter = Some(iter);
                            item
                        }
                        ProcessResult::Skip(times) => {
                            self.iter.nth(times);
                            self.next()
                        }
                    }
                }
            }
        }
    }
}

pub trait MoreIter: Iterator {
    fn process<F>(self, f: F) -> Process<Self::Item, Self, F> where Self: Sized, F : FnMut(Self::Item) -> ProcessResult<Self::Item> {
        Process { __iter: None, iter: self, predicate: f }
    }
}

#[doc(hidden)]
impl<T: Iterator> MoreIter for T {}