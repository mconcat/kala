// Object model in Kala
//
// - Statically known object, with inlined properties
// - Unknown object, with linked list of properties(XS style)
// - Trait object, with vtable mapping to internal object(for generic functions and high order functions)

// Similar with HiddenClass, but we don't have classes in Jessie.
// Hidden interfaces are constructed in type checker by static analysis
// but it could be dynamically generated too just like V8
pub struct HiddenInterface {
    id: u32,
    properties: Vec<(String, u32)>
}

pub struct PropertyMap {
    hidden_interface: Rc<HiddenInterface>, // to get the offset in case we don't know
    store: Vec<Value>, // linearlized store
}

pub struct ObjectInternal(pub Rc<RefCell<PropertyMap>>);


pub enum Object {
    // For 0.1, the objects will not be destructed. All objects have persistent lifetime.

    // Object
    //RAIIObject(ObjectInternal), // the lifetime of the object ends inside the block where it is declared
    LifetimeObject(ObjectInternal), // the lifetime is statically known, and drop analysis had inserted where it should be destructed
    //RcObject(ObjectInternal), // the lifetime could not be statically known, wrap in Rc
}