use crate::{expression::Expr, VariableCell, VariablePointer};
use utils::{SharedString};

#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct Record(pub Vec<PropDef>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum PropDef {
    KeyValue(Box<Field>, Expr),
    Shorthand(Box<Field>, Box<VariableCell>),
    Spread(Expr),
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