use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element};

/*
usage
```
use req;

req::I::("element-id")
    .unwrap()
    .on("click", Box::new(move |event|{..}));

I is just a word for Id.
```
*/

pub struct I {
    document: Document,
    element: Element,
}

impl I {
    pub fn new(class_or_id: &str) -> Result<I, &'static str> {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");

        let element: Element = document.get_element_by_id(&class_or_id).unwrap();
        let element: Element = element.dyn_into::<Element>().map_err(|_| ()).unwrap();
        Ok(I { document, element })
    }

    pub fn html(&self, text: &str) -> &I {
        self.element.set_inner_html(&text);
        self
    }

    pub fn class_name(&self, class_name: &str) -> &I {
        self.element.set_class_name(class_name);
        self
    }

    pub fn on(&self, event: &str, func: Box<dyn Fn(web_sys::MouseEvent)>) -> Result<&I, &str> {
        let closure = Closure::wrap(func);
        self.element
            .add_event_listener_with_callback(&event, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        Ok(self)
    }

    pub fn update(&self) -> Result<&I, &'static str> {
        self.document
            .body()
            .unwrap()
            .append_child(&self.element)
            .unwrap();
        Ok(self)
    }
}

pub struct N {
    document: Document,
    element: Element,
}

impl N {
    pub fn new(name: &str) -> Result<N, &'static str> {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");

        let element: Element = document
            .create_element(&name)
            .unwrap()
            .dyn_into::<Element>()
            .unwrap();

        Ok(N { document, element })
    }

    pub fn html(&self, text: &str) -> &N {
        self.element.set_inner_html(&text);
        self
    }

    pub fn class_name(&self, class_name: &str) -> &N {
        self.element.set_class_name(class_name);
        self
    }

    pub fn on(&self, event: &str, func: Box<dyn Fn(web_sys::MouseEvent)>) -> Result<&N, &str> {
        let closure = Closure::wrap(func);
        self.element
            .add_event_listener_with_callback(&event, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        Ok(self)
    }

    pub fn update(&self) -> Result<&N, &'static str> {
        self.document
            .body()
            .unwrap()
            .append_child(&self.element)
            .unwrap();
        Ok(self)
    }
}
