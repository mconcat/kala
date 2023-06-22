use crate::{expression::Expr, VariableCell, VariablePointer};
use utils::{SharedString, OwnedSlice};

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct Record(pub OwnedSlice<PropDef>);

pub enum PropDefDiscriminant {
    KeyValue = 0, 
    Shorthand = 1,
    Spread = 2,
    // Getter = 3,
    // Setter = 4,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum PropDef {
    KeyValue(Box<Field>, Expr) = PropDefDiscriminant::KeyValue as u8,
    Shorthand(Box<Field>, VariableCell) = PropDefDiscriminant::Shorthand as u8,
    Spread(Expr) = PropDefDiscriminant::Spread as u8,
    // Getter(Function) = PropDefDiscriminant::Getter as u8,
    // Setter(Function) = PropDefDiscriminant::Setter as u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub dynamic_property: SharedString,
    pub static_property: Option<StaticProperty>,    
}

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