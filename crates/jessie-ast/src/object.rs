use crate::{expression::Expr, VariableCell};

#[derive(Debug, PartialEq, Clone)]
pub struct Record<'a>(pub &'a [PropDef<'a>]);

#[derive(Debug, PartialEq, Clone)]
pub enum PropDef<'a> {
    KeyValue(&'a Field<'a>, Expr<'a>), 
    // ComputedKeyValue(ComputedProperty<'a>, Expr<'a>), // TODO
    // MethodDef(MethodDef),// TODO
    Shorthand(&'a Field<'a>, &'a VariableCell<'a>),
    Spread(Expr<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field<'a> {
    pub dynamic_property: &'a str,
    pub static_property: Option<StaticProperty>,    
}

#[derive(Debug, PartialEq, Clone)]
pub enum StaticProperty {
    // known name, type inferred, accessed with fixed offset
    InlineProperty(u16),
    // unknown name, type inferred, accessed with hiddenclass offset
    FastProperty(u16),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedProperty<'a>(Expr<'a>);