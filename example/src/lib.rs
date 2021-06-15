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
    rq::N::new("button")
        .unwrap()
        .html("+")
        .on(
            "click",
            Box::new(move |_event| {
                rq::I::new("content").unwrap().html_cj(&|html| {
                    let html = html.parse::<i32>().unwrap() + 1;
                    html.to_string()
                });
            }),
        )
        .unwrap()
        .append("body")
        .unwrap();

    rq::N::new("button")
        .unwrap()
        .html("0")
        .id("content")
        .on("click", Box::new(move |_event| {}))
        .unwrap()
        .append("body")
        .unwrap();

    rq::N::new("button")
        .unwrap()
        .html("-")
        .on("click", Box::new(move |_event| {}))
        .unwrap()
        .on(
            "click",
            Box::new(move |_event| {
                rq::I::new("content").unwrap().html_cj(&|html| {
                    let html = html.parse::<i32>().unwrap() - 1;
                    html.to_string()
                });
            }),
        )
        .unwrap()
        .append("body")
        .unwrap();
}
