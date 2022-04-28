#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(coverage, feature(no_coverage))]

//! A Simple, Lightweight, Fast and Safe Fugle Library.
//!
//! What is [Fugle][fugleweb]
//!
//! This is a library of rust version to access Fugle's
//!
//! * RESTful API
//! * Websocket (enable websocket feature)
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
//! # use fugle::intraday::IntradayBuilder;
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
//! # use fugle::ws::Intraday;
//!                                                            
//! let mut lis = Intraday::new("demo");
//!                                                            
//! let rx = lis.chart("2884", true)?;
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
//! # use fugle::intraday::IntradayBuilder;
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
//! # use fugle::ws::Intraday;;
//!                                                            
//! let mut lis = Intraday::new("demo");
//!                                                            
//! let rx = lis.quote("2884", true)?;
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
//! # use fugle::intraday::IntradayBuilder;
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
//! # use fugle::ws::Intraday;;
//!                                                            
//! let mut lis = Intraday::new("demo");
//!                                                            
//! let rx = lis.meta("2884", true)?;
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
//! # use fugle::intraday::IntradayBuilder;
//!                                             
//! let agent = IntradayBuilder::new().build();
//! agent.dealts("2884").call()?;
//!                                             
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Volumes][fuglevolumesweb]
//!
//! API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::intraday::IntradayBuilder;
//!                                             
//! let agent = IntradayBuilder::new().build();
//! agent.volumes("2884").call()?;
//!                                             
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Candles][fuglecandlesweb]
//!
//! API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::marketdata::MarketdataBuilder;
//!
//! let agent = MarketdataBuilder::new().build();
//! agent.candles("2884").call()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! [fugleweb]: https://developer.fugle.tw
//! [fuglechartweb]: https://developer.fugle.tw/docs/data/intraday/chart
//! [fuglequoteweb]: https://developer.fugle.tw/docs/data/intraday/quote
//! [fuglemetaweb]: https://developer.fugle.tw/docs/data/intraday/meta
//! [fugledealtsweb]: https://developer.fugle.tw/docs/data/intraday/dealts
//! [fuglevolumesweb]: https://developer.fugle.tw/docs/data/intraday/volumes
//! [fuglecandlesweb]: https://developer.fugle.tw/docs/data/marketdata/candles

pub mod errors;
pub mod intraday;
pub mod marketdata;
pub mod schema;
#[cfg(feature = "websocket")]
pub mod ws;
