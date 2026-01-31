use wasm_bindgen::JsValue;
use web_sys::{js_sys::Array};

pub fn log(message : String)
{
	let js_value = JsValue::from_str(&message);
	let arr = Array::new();
	arr.push(&js_value);
	web_sys::console::log(&arr);
}
