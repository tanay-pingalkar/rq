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
N stands for new to create new element.
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
N stands for class to get elements by class name.
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


currently the api is small and i am still learning rust. I will add more features to it so stay tuned.
