use crate::{expression::Expr, Variable, Function};
use utils::{SharedString};


impl Field {
    pub fn new_dynamic(dynamic_property: SharedString) -> Self {
        Self {
            dynamic_property,
            static_property: None,
        }
    }

    pub fn name(&self) -> SharedString {
        self.dynamic_property.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StaticProperty {
    // known name, type inferred, accessed with fixed offset
    InlineProperty(u16),
    // unknown name, type inferred, accessed with hiddenclass offset
    FastProperty(u16),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedProperty(Expr);