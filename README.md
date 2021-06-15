# rq
A jquery inspired library for wasm made in rust

## rq::I
`I` stands for id to get element.
You can use it like this
``` rust
use req;
use web_sys::console;

rq::I::new("class-name")
  .unwrap()
  .html("manipulating dom from rust")
  .on("click",Box::new(move |event| {
    console::log_2(&"clicked : ".into(), &event.into())
  }))
```

## req::N
`N` stands for new to create new element.
``` rust
use req;
use web_sys::console;

rq::N::new("p") // name of tag
  .unwrap()
  .html("content of tag")
  .on("click",Box::new(move |event| {
    console::log_2(&"event : ".into(), &event.into())
  }))
```

## req::C
`C` stands for class to get elements by class name.
``` rust
use req;
use web_sys::console;

rq::C::new("class-name") // name of tag
  .unwrap()
  .html("rq looks good")
  .on("click",Box::new(move |event| {
    console::log_2(&"event : ".into(), &event.into())
  }))
```

## html_cj
html_cj takes a closure and return the html, it is important when you have to change the html of element based on previous html
``` rust
use req;

rq::N::new("button")
  .unwrap()
  .html("0")
  .id("btn-id")
  .on("click",Box::new(move |event| {
      req::I::new("btn-id", |html| {
        let html = html.parse::<u32>().unwrap() + 1;
        html.to_string()
      });
  }))
```


currently the api is small and i am still learning rust. I will add more features to it so stay tuned.
