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
//! Restful API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::http::{ RestfulBuilder, intraday::ChartRequest };
//! #
//! let client = RestfulBuilder::new().build()?;
//! client.call(ChartRequest::new().symbol_id("2884"))?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! Websocket
//! ```rust no_run
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::websocket::IntradayBuilder;
//! #
//! let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
//!                                                            
//! let rx = ws.chart()?;
//! let response = rx.recv()?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Quote][fuglequoteweb]
//!
//! Restful API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::http::{ RestfulBuilder, intraday::QuoteRequest };
//! #
//! let client = RestfulBuilder::new().build()?;
//! client.call(QuoteRequest::new().symbol_id("2884"))?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! Websocket
//! ```rust no_run
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::websocket::IntradayBuilder;;
//! #
//! let mut ws = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
//!                                                            
//! let rx = ws.quote()?;
//! let response = rx.recv()?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Meta][fuglemetaweb]
//!
//! Restful API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::http::{ RestfulBuilder, intraday::MetaRequest };
//! #
//! let client = RestfulBuilder::new().build()?;
//! client.call(MetaRequest::new().symbol_id("2884"))?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! Websocket
//! ```rust no_run
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::websocket::IntradayBuilder;;
//! #
//! let mut lis = IntradayBuilder::new().symbol_id("2884").odd_lot().build();
//!                                                            
//! let rx = lis.meta()?;
//! let response = rx.recv()?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Dealts][fugledealtsweb]
//!
//! Restful API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::http::{ RestfulBuilder, intraday::DealtsRequest };
//! #
//! let client = RestfulBuilder::new().build()?;
//! client.call(
//!     DealtsRequest::new()
//!     .symbol_id("2884")
//!     .limit(10)
//!     .offset(0)
//! )?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Volumes][fuglevolumesweb]
//!
//! Restful API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::http::{ RestfulBuilder, intraday::VolumesRequest };
//! #
//! let client = RestfulBuilder::new().build()?;
//! client.call(VolumesRequest::new().symbol_id("2884"))?;
//! #
//! # Ok(())
//! # }
//! ```
//!
//! ### [Fugle Candles][fuglecandlesweb]
//!
//! Restful API
//! ```rust
//! # fn main() -> fugle::schema::Result<()> {
//! # use fugle::http::{ RestfulBuilder, marketdata::CandlesRequest };
//! #
//! let client = RestfulBuilder::new().build()?;
//! client.call(
//!     CandlesRequest::new()
//!     .symbol_id("2884")
//!     .from("2022-08-01")
//!     .to("2022-08-08")
//! )?;
//! #
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
pub mod http;
pub mod schema;
#[cfg(any(feature = "websocket", feature = "async-websocket"))]
pub mod websocket;
