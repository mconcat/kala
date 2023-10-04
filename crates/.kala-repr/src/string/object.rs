use std::{ops::Deref, str::from_utf8, mem::{transmute, size_of}};

use utils::SharedString;

use crate::{slot::{Slot}};

#[repr(C)]
pub struct StringObject(pub(crate) SharedString); // UTF-8, not ECMAScript string

impl StringObject {
	pub fn new(s: SharedString) -> Self {
		unimplemented!("new")	
	}
}

impl Deref for StringObject {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		from_utf8(&*self.0).unwrap()
	}
}

impl PartialEq for StringObject {
	fn eq(&self, other: &Self) -> bool {
		self.deref() == other.deref()
	}
}

impl ToString for StringObject {
	fn to_string(&self) -> String {
		format!("\"{}\"", self.deref())
	}
}