# plctag-rs

a rust wrapper of [libplctag](https://github.com/libplctag/libplctag), with rust style APIs and useful extensions.

[![crates.io](https://img.shields.io/crates/v/plctag.svg)](https://crates.io/crates/plctag)
[![docs](https://docs.rs/plctag/badge.svg)](https://docs.rs/plctag)
[![build](https://github.com/joylei/plctag-rs/workflows/Test%20and%20Build/badge.svg?branch=master)](https://github.com/joylei/plctag-rs/actions?query=workflow%3A%22Test+and+Build%22)
[![license](https://img.shields.io/crates/l/plctag.svg)](https://github.com/joylei/plctag-rs/blob/master/LICENSE)

## Why plctag-rs

- thin wrapper on `libplctag`
- UDT support with macros
- with asynchronous & thread-safe based on `tokio`
- builder to build tag

## How to use

Add `plctag` to your Cargo.toml

```toml
[dependencies]
plctag= "0.1"
```

## crates

- [plctag](https://crates.io/crates/plctag) reexports everything from below crates.
- [plctag-core](https://crates.io/crates/plctag-core) a rust wrapper of [libplctag](https://github.com/libplctag/libplctag), with rust style APIs and useful extensions.
- [plctag-async](https://crates.io/crates/plctag-async) tokio based async wrapper.
- [plctag-log](https://crates.io/crates/plctag-log) log adapter for `libplctag`
- [plctag-derive](https://crates.io/crates/plctag-derive) macros for `plctag`
- [plctag-sys](https://crates.io/crates/plctag-sys) native libplctag binding

## Examples

### read/write tag

```rust
use plctag::{Encode, Decode, RawTag};
let timeout = 100;//ms
let path="protocol=ab-eip&plc=controllogix&path=1,0&gateway=192.168.1.120&name=MyTag1&elem_count=1&elem_size=16";// YOUR TAG DEFINITION
let tag = RawTag::new(path, timeout).unwrap();

//read tag
let status = tag.read(timeout);
assert!(status.is_ok());
let offset = 0;
let value:u16 = tag.get_value(offset).unwrap();
println!("tag value: {}", value);

let value = value + 10;
tag.set_value(offset, value).unwrap();

//write tag
let status = tag.write(timeout);
assert!(status.is_ok());
println!("write done!");
```

### UDT

read/write UDT

```rust
use plctag::{Decode, Encode, RawTag, Result};

// define your UDT
#[derive(Default, Debug, Decode, Encode)]
struct MyUDT {
    #[tag(offset = 0)]
    v1: u16,
    #[tag(offset = 2)]
    v2: u16,
}

fn main() {
    let timeout = 100; //ms
                       // YOUR TAG DEFINITION
    let path = "protocol=ab-eip&plc=controllogix&path=1,0&gateway=192.168.1.120&name=MyTag2&elem_count=2&elem_size=16";
    let tag = RawTag::new(path, timeout).unwrap();

    //read tag
    let status = tag.read(timeout);
    assert!(status.is_ok());
    let offset = 0;
    let mut value: MyUDT = tag.get_value(offset).unwrap();
    println!("tag value: {:?}", value);

    value.v1 = value.v1 + 10;
    tag.set_value(offset, value).unwrap();

    //write tag
    let status = tag.write(timeout);
    assert!(status.is_ok());
    println!("write done!");
}
```

Note:
Do not perform expensive operations when you derives `Decode` or `Encode`.

### Async

```rust
use plctag::futures::{AsyncTag, Error, TagEntry};

use tokio::runtime;

fn main() {
    let rt = runtime::Runtime::new().unwrap();
    let res: Result<_, Error> = rt.block_on(async {
        let path="protocol=ab-eip&plc=controllogix&path=1,0&gateway=192.168.1.120&name=MyTag1&elem_count=1&elem_size=16"; // YOUR TAG DEFINITION
        let tag = TagEntry::create(path).await?;
        let tag_ref = tag.get().await?;
        let offset = 0;
        let value: u16 = tag_ref.read_value(offset).await?;
        println!("tag value: {}", value);

        let value = value + 10;
        tag_ref.write_value(offset, value).await?;
        Ok(())
    });
    res.unwrap();
}

```

### Path Builder

```rust
use plctag::builder::*;
use plctag::RawTag;

fn main() {
    let timeout = 100;
    let path = PathBuilder::default()
        .protocol(Protocol::EIP)
        .gateway("192.168.1.120")
        .plc(PlcKind::ControlLogix)
        .name("MyTag1")
        .element_size(16)
        .element_count(1)
        .path("1,0")
        .read_cache_ms(0)
        .build()
        .unwrap();
    let tag = RawTag::new(path, timeout).unwrap();
    let status = tag.status();
    assert!(status.is_ok());
}

```

### Logging adapter for `libplctag`

```rust
use plctag::log::log_adapt;
use plctag::log::set_debug_level;
use plctag::log::DebugLevel;

log_adapt(); //register logger
set_debug_level(DebugLevel::Info); // set debug level

// now, you can receive log messages by any of logging implementations of crate `log`

```

## Thread-safety

Operations are not thread-safe in this library except async wrappers, please use `std::sync::Mutex` or something similar to enforce thread-safety.

## Build & Test

Please refer to `How to use` to setup build environment.

Because mutithread will cause troubles, you need to run tests with:

```shell
cargo test -- --test-threads=1
```

## License

MIT
