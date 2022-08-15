// ForeignBinding provides a foreign function interface for the kala runtime.
// a foreign call from the script is routed to bindings, which is then
// translated into a rust native function call.
pub trait ForeignBinding {
    type JSValue: crate::runtime::JSValue;

    fn serialize(&self, value: JSValue) -> Vec<u8>;
    fn deserialize(&self, Vec<u8>) -> JSValue;
    fn call(&self, args: &[JSValue]) -> Result<Value, Error>;
}