use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlCollection};

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
    pub fn new(id: &str) -> Result<I, &'static str> {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");

        let element: Element = document.get_element_by_id(id).unwrap();
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

    pub fn append(&self) -> Result<&I, &'static str> {
        self.document
            .body()
            .unwrap()
            .append_child(&self.element)
            .unwrap();
        Ok(self)
    }
}

/*
usage
```
use req;

req::N::("element-name")
    .unwrap()
    .html("ll")
    .update()
    .unwrap();

N is just a word for New.
```
*/
pub struct N {
    document: Document,
    element: Element,
}

impl N {
    pub fn new(name: &str) -> Result<N, &'static str> {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");

        let element: Element = document
            .create_element(name)
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

    pub fn append(&self) -> Result<&N, &'static str> {
        self.document
            .body()
            .unwrap()
            .append_child(&self.element)
            .unwrap();
        Ok(self)
    }
}

pub struct C {
    document: Document,
    element: HtmlCollection,
}

impl C {
    pub fn new(class_name: &str) -> Result<C, &'static str> {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");

        let element: HtmlCollection = document.get_elements_by_class_name(class_name);
        let element: HtmlCollection = element
            .dyn_into::<HtmlCollection>()
            .map_err(|_| ())
            .unwrap();

        Ok(C { document, element })
    }

    pub fn html(&self, text: &str) -> &C {
        for i in 0..self.element.length() {
            self.element.item(i).unwrap().set_inner_html(text);
        }
        self
    }

    pub fn class_name(&self, class_name: &str) -> &C {
        for i in 0..self.element.length() {
            self.element.item(i).unwrap().set_class_name(class_name);
        }
        self
    }

    pub fn on(&self, event: &str, func: Box<dyn Fn(web_sys::MouseEvent)>) -> Result<&C, &str> {
        let closure = Closure::wrap(func);
        for i in 0..self.element.length() {
            self.element
                .item(i)
                .unwrap()
                .add_event_listener_with_callback(&event, closure.as_ref().unchecked_ref())
                .unwrap();
        }
        closure.forget();
        Ok(self)
    }

    pub fn append(&self) -> Result<&C, &'static str> {
        for i in 0..self.element.length() {
            self.document
                .body()
                .unwrap()
                .append_child(&self.element.item(i).unwrap())
                .unwrap();
        }
        Ok(self)
    }
}
