extern crate fxhash;

use crate::SharedString;
use fxhash::FxHashMap;

pub type Map<V> = FxHashMap<SharedString, V>;