pub enum Type {
    Number,
    Bigint,
    String,
    Boolean,
    Undefined,

    Function {
        type_parameters: Vec<String>,
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
    Reference {
        name: String,
        type_arguments: Vec<Type>,
    },
    Array {
        element_type: Box<Type>,
    },
    Tuple {
        element_types: Vec<Type>,
    },

    // Union(UnionType),
    // Intersection(IntersectionType),
    // Conditional(ConditionalType),

    Optional {
        element_type: Box<Type>,
    },
    Rest {
        element_type: Box<Type>,
    },

    NumberLiteral {
        value: u32,
    },
    StringLiteral {
        value: String,
    },

    ObjectLiteral {
        type_parameters: Vec<String>,
        properties: Vec<Property>,
    },
}

pub enum Property {
    Property {
        name: String,
        optional: bool,
        element_type: Type,
    },
    Method {
        name: String,
        optional: bool,
        type_parameters: Vec<String>,
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

pub struct BoundType {
    upper_bound: Type,
    lower_bound: Type,
}