use crate::{expression::Expr, VariableCell};

#[derive(Debug, PartialEq, Clone)]
pub struct Record(pub Vec<PropDef>);

#[derive(Debug, PartialEq, Clone)]
pub enum PropDef {
    KeyValue(Box<Field>, Expr), 
    // ComputedKeyValue(ComputedProperty, Expr), // TODO
    // MethodDef(MethodDef),// TODO
    Shorthand(Box<Field>, Box<VariableCell>),
    Spread(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub dynamic_property: String,
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
pub struct ComputedProperty(Expr);