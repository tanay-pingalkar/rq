use rq;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main() {
    rq::I::new("heading")
        .unwrap()
        .html("sndjfajasjh")
        .on("click", Box::new(move |event: web_sys::MouseEvent| {}))
        .unwrap()
        .class_name("new class");

    // rq::N::new("button")
    //     .unwrap()
    //     .html("sndjfajasjh")
    //     .on(
    //         "click",
    //         Box::new(move |event: web_sys::MouseEvent| console_log!("hey yo")),
    //     )
    //     .unwrap()
    //     .class_name("new class")
    //     .update()
    //     .unwrap();

    rq::C::new("aclass").unwrap().html("a button from rq").on(
        "click",
        Box::new(move |_event| {
            rq::C::new("aclass").unwrap().html("comon");
        }),
    );
}
