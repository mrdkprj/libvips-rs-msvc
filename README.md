# libvips-rust-bindings
Rust bindings for libvips. Generated from `version 8.16.1`.

This is a safe wrapper for [libvips](https://libvips.github.io/libvips/) C library. It is made on top of the C API and based on the introspection API results.

This crate itself is not documented, but it has no logic or special behavior in comparison to libvips itself. All calls and types described in the official libvips [docs](https://libvips.github.io/libvips/API/current/) are just translated to rust types. All defaults also respected.

## About this crate

This is a fork of libvips-rust-bindings. 

This crate is different from it in that this

- uses `VOption` for optional arguments of some vips operations instead of structs to prevent unnecessary default values. 
- supports operator overloads
- supports some operations to VipsImage like `get_int()` and `set_int()`.

## How to use it

Vips needs to be initialized. You have to call `Vips::init()` at least once before any operations.  

Shutdown is optional. You can shut down by `Vips::shutdown()`. Once Vips is shut down, all operations including `Vips::init()` are no longer available.  

Many vips operations have optional arguments. The ones that have have been implemented with too variants by this crate. Basically there'll be a regular call with only the required parameters and an additional with the suffix `with_opts` which takes `VOption` containing optional arguments.  

```rust
let option = VOption::new().set("embedded", v_value!(true)).set("depth", v_value!(16));
```

In the moment the error messages are not being appended to the errors themselves. They're in the libvips error buffer. The error buffer operations are implented inside the `Vips` struct. 

Most (if not all) vips operations don't mutate the `VipsImage` object, so they'll return a new object for this. The implementation of `VipsImage` in this crate takes care of freeing the internal pointer after it is dropped. <span style="color:red">Be aware that the VipsImage object is not thread safe in the moment.</span>. 

### Example

```rust
use libvips::{v_value, voption::VOption, Vips, VipsImage};

fn main() {
    // this initializes the libvips library.
    Vips::init("Test Libvips", false).expect("Cannot initialize libvips");
    //set number of threads in libvips's threadpool
    Vips::concurrency_set(2);

    // loads an image from file
    let image = VipsImage::new_from_file("test.png").unwrap();

    // will resize the image and return a new instance.
    // libvips works most of the time with immutable objects, so it will return a new object
    // the VipsImage struct implements Drop, which will free the memory
    let resized = image.resize(0.5).unwrap();

    // save with optional parameters
    match resized.jpegsave_with_opts(
        "output.jpeg",
        VOption::new()
            .set("q", v_value!(90))
            .set("background", v_value!(&[255.0]))
            .set("strip", v_value!(true))
            .set("interlace", v_value!(true))
            .set("optimize_scans", v_value!(true))
            .set("optimize_coding", v_value!(true)),
    ) {
        Err(_) => println!("error: {}", Vips::error_buffer().unwrap()),
        Ok(_) => println!("Great Success!"),
    }

    // only when you are done with Vips, shut it down.
    // this is optional as described in the official document.
    Vips::shutdown();
}
```
