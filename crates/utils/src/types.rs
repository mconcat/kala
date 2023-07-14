use std::{rc::Rc, ops::{Deref, DerefMut}, hash::Hash};
/* 
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct OwnedSlice<T>(pub Box<[T]>);

impl<T> OwnedSlice<T> {
    pub fn empty() -> Self {
        Self(vec![].into_boxed_slice())
    }

    // TODO: into_boxed_slice deep copies if the capacity is different, do something
    // we definitely don't want to deep copy everytime we call this
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self(vec.into_boxed_slice())
    }

    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        Self::from_vec(slice.to_vec())
    }
}

impl<T> Deref for OwnedSlice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0
    }
}

impl<T> DerefMut for OwnedSlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}*/
/* 
pub struct OwnedSliceIter<T>{
    cursor: *const T,
    end: *const T,
}

impl<T> Iterator for OwnedSliceIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{:?}, {:?}", self.cursor, self.end);
        if self.cursor == self.end {
            None
        } else {
            let item = unsafe { self.cursor.read() };
            self.cursor = unsafe { self.cursor.add(1) };
            Some(item)
        }
    }
}
*/

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct SharedSlice<T>(pub Rc<[T]>);

impl<T> SharedSlice<T> {
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self(Rc::from(vec))
    }
}

impl<T> Deref for SharedSlice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct SharedString(pub Rc<str>);

impl From<String> for SharedString {
    fn from(value: String) -> Self {
        SharedString::from_string(value)
    }
}

impl Deref for SharedString {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl SharedString {
    pub fn empty() -> Self {
        Self("".into())
    }
    
    pub fn from_string(string: String) -> Self {
        Self(Rc::from(string))
    }

    pub fn from_str(string: &str) -> Self {
        Self(string.into())
    }
}

impl Hash for SharedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
/* 
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct OwnedString(pub Box<str>);

impl OwnedString {
    pub fn empty() -> Self {
        Self("".into())
    }
    
    pub fn from_string(string: String) -> Self {
        Self(string.into_boxed_str())
    }

    pub fn from_str(string: &str) -> Self {
        Self(string.into())
    }
}

impl Hash for OwnedString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
*/
