#![forbid(unsafe_code)]
#![warn(clippy::all)]

//! A Simple, Lightweight, Fast and Safe Fugle Library.
//!
//! What is [Fugle][fugleweb]
//!
//! This is a rust version library to access Fugle's
//!
//! * RESTful API
//! * Websocket
//!
//! services in a very easy way.
//!
//! ## Examples
//!
//! for more please reference to the examples folder
//!
//!
//! ### [Fugle Chart][fuglechartweb]
//!
//! API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::crawler::IntradayBuilder;
//!                                             
//! let agent = IntradayBuilder::new().build();
//! agent.chart("2884").call()?;
//!                                             
//! # Ok(())
//! # }
//! ```
//!
//! Websocket
//! ```rust no_run
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::listener;
//! # use std::sync::mpsc;
//!                                                            
//! let (tx, rx) = mpsc::channel();
//! let mut lis = listener::Intraday::new("demo", tx.clone());
//!                                                            
//! lis.chart("2884", true);
//! let response = rx.recv()?;
//!                                                            
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Quote][fuglequoteweb]
//!
//! API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::crawler::IntradayBuilder;
//!                                             
//! let agent = IntradayBuilder::new().build();
//! agent.quote("2884").call()?;
//!                                             
//! # Ok(())
//! # }
//! ```
//!
//! Websocket
//! ```rust no_run
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::listener;
//! # use std::sync::mpsc;
//!                                                            
//! let (tx, rx) = mpsc::channel();
//! let mut lis = listener::Intraday::new("demo", tx.clone());
//!                                                            
//! lis.quote("2884", true);
//! let response = rx.recv()?;
//!                                                            
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Meta][fuglemetaweb]
//!
//! API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::crawler::IntradayBuilder;
//!                                             
//! let agent = IntradayBuilder::new().build();
//! agent.meta("2884").call()?;
//!                                             
//! # Ok(())
//! # }
//! ```
//!
//! Websocket
//! ```rust no_run
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::listener;
//! # use std::sync::mpsc;
//!                                                            
//! let (tx, rx) = mpsc::channel();
//! let mut lis = listener::Intraday::new("demo", tx.clone());
//!                                                            
//! lis.meta("2884", true);
//! let response = rx.recv()?;
//!                                                            
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Dealts][fugledealtsweb]
//!
//! API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::crawler::IntradayBuilder;
//!                                             
//! let agent = IntradayBuilder::new().build();
//! agent.dealts("2884").call()?;
//!                                             
//! # Ok(())
//! # }
//! ```
//!
//! [fugleweb]: https://developer.fugle.tw
//! [fuglechartweb]: https://developer.fugle.tw/document/intraday/chart
//! [fuglequoteweb]: https://developer.fugle.tw/document/intraday/quote
//! [fuglemetaweb]: https://developer.fugle.tw/document/intraday/meta
//! [fugledealtsweb]: https://developer.fugle.tw/document/intraday/dealts

pub mod crawler;
pub mod listener;
pub mod schema;
