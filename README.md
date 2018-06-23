## Prismatic

An experimental Rust crate for decomposing the initialization of structs.

![prism](prism.gif)

## Overview

A common idiom in Rust is to define a `new` function that initializes a struct.
Sometimes this function can get quite large. If you try to decompose it into
steps, you're often left with complicated signatures on private functions:

```rust
impl Webpage {
    pub fn new(title: &str) -> Self {
        let window = Self::fetch_window();
        let document = Self::fetch_document();
        let body = Self::fetch_body(&document);
        let canvas = Self::create_canvas(&document);
        let context = Self::fetch_context(&canvas);

        Self::set_title(&document, title);
        Self::reset_styles(&document, &body);
        Self::add_canvas_to_page(&body, &canvas);
        Self::resize_canvas(&body, &canvas);
        Self::bind_resize_event(&window, &body, &canvas);

        Webpage { window, document, body, canvas, context }
    }

    fn fetch_body(document: &Document) -> Body {
      document.body().expect("failed to fetch body")
    }

    fn bind_resize_event(window: &Window, body: &Body, canvas: &canvas) {
      // ...
    }

    // ...
  }
```

### Prismatic

Prismatic provides a macro to make this a bit easier. It generates a special
`Init` struct that can be used for initialization, reducing the overhead of
extracting functions for each of the steps:

```rust
#[macro_use]
extern crate prismatic;

#[derive(New)]
#[Sig = "title: &str"]
struct Webpage {
  window: Window,
  document: Document,
  body: Body,
  canvas: Canvas,
  context: Context,
}

impl Init {
  fn init(&mut self, title: &str) {
    self.fetch_window();
    self.fetch_document();
    self.fetch_body();
    self.create_canvas();
    self.fetch_context();

    self.set_title(title);
    self.reset_styles();
    self.add_canvas_to_page();
    self.resize_canvas();
    self.bind_resize_event();
  }

  fn fetch_body(&mut self) {
    let body = self.document.unwrap().body("failed to fetch body");
    self.body = Some(body);
  }

  fn bind_resize_event(&mut self) {
    // ...
  }

  // ...
}
```

In the example above, the `fetch_body` function sets `self.body` from the
previously initialized value of `self.document`. All fields on the `Init` struct
are `Option` types, which is why we use `unwrap()` and `Some`.

## Usage

Here's a minimal example:

```rust
#[macro_use]
extern crate prismatic;

#[derive(New)]
struct Foo { bar: usize }

impl Init {
  fn init(&mut self) {
    self.bar = Some(123);
  }
}

let foo = Foo::new()
assert_eq!(foo.bar, 123);
```

The `Sig` attribute (short for `Signature`) can be used if the initializer
requires additional arguments:

```rust
#[derive(New)]
#[Sig = "x: usize"]
struct Foo { bar: usize }

impl Init {
  fn init(&mut self, x: usize) {
    self.bar = Some(x);
  }
}

let foo = Foo::new(123);
assert_eq!(foo.bar, 123);
```
