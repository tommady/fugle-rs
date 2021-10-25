# fugle-rs
[![Build status](https://github.com/tommady/fugle-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/tommady/fugle-rs/actions)
[![Latest version](https://img.shields.io/crates/v/fugle)](https://crates.io/crates/fugle)
[![Documentation](https://docs.rs/fugle/badge.svg)](https://docs.rs/fugle)
![License](https://img.shields.io/crates/l/fugle)
---

A Simple, Lightweight, Fast and Safe Fugle Library.

What is [Fugle][fugleweb]

This is a rust version library to access Fugle's

* RESTful API
* Websocket (enable websocket feature)

services in a very easy way.

## Examples

for more please reference to the examples folder


### [Fugle Chart][fuglechartweb]

API
```rust
let agent = IntradayBuilder::new().build();
agent.chart("2884").call()?;
```

Websocket
```rust no_run
let (tx, rx) = mpsc::channel();
let mut lis = listener::Intraday::new("demo", tx.clone());
                                                           
lis.chart("2884", true);
let response = rx.recv()?;
```

### [Fugle Quote][fuglequoteweb]

API
```rust
let agent = IntradayBuilder::new().build();
agent.quote("2884").call()?;
```

Websocket
```rust no_run
let (tx, rx) = mpsc::channel();
let mut lis = listener::Intraday::new("demo", tx.clone());
                                                           
lis.quote("2884", true);
let response = rx.recv()?;
```

### [Fugle Meta][fuglemetaweb]

API
```rust
let agent = IntradayBuilder::new().build();
agent.meta("2884").call()?;
```

Websocket
```rust no_run
let (tx, rx) = mpsc::channel();
let mut lis = listener::Intraday::new("demo", tx.clone());
                                                           
lis.meta("2884", true);
let response = rx.recv()?;
```

### [Fugle Dealts][fugledealtsweb]

API
```rust
let agent = IntradayBuilder::new().build();
agent.dealts("2884").call()?;
```

### [Fugle Volumes][fuglevolumesweb]

API
```rust
let agent = IntradayBuilder::new().build();
agent.volumes("2884").call()?;
```

[fugleweb]: https://developer.fugle.tw
[fuglechartweb]: https://developer.fugle.tw/document/intraday/chart
[fuglequoteweb]: https://developer.fugle.tw/document/intraday/quote
[fuglemetaweb]: https://developer.fugle.tw/document/intraday/meta
[fugledealtsweb]: https://developer.fugle.tw/document/intraday/dealts
[fuglevolumesweb]: https://developer.fugle.tw/document/intraday/volumes
