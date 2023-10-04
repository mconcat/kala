use super::slot::Slot;

pub struct Stack(pub Vec<Slot>);

pub struct Frame<'a> {
    pub stack: &'a mut [Slot],
}

pub struct StaticFunction<Code> {
    code: Code,
}