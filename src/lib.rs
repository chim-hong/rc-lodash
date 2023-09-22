use js_sys::Object;
use wasm_bindgen::{self, prelude::*};

#[wasm_bindgen(typescript_custom_section)]
const IName: &'static str = r#"
type Name = "key" | "value" | "entries";
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Name")]
    pub type Name;
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        s.into()
    }
}

#[wasm_bindgen(js_name = "objectToArray")]
pub fn object_to_array(object: &JsValue, type_name: Name) -> Vec<JsValue> {
    let values = object.clone().dyn_into::<Object>().unwrap();
    let binding = type_name.clone().as_string().unwrap();
    let type_name = binding.as_str();
    match type_name {
        "key" => Object::keys(&values).to_vec(),
        "value" => Object::values(&values).to_vec(),
        "entries" => Object::entries(&values).to_vec(),
        _ => vec![],
    }
}

