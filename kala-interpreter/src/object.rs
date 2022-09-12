#[derive(PartialEq, Debug, Clone)]
pub struct JSObject {
    prototype: Option<Prototype>, // none if simple Object
    properties: BTreeMap<String, JSValue>,
}

impl ToString for JSObject {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("{");
        for (k, v) in &self.properties {
            s.push_str(&format!("{}:{},", k, v.to_string()));
        }
        s.push_str("}");
        s
    }
}