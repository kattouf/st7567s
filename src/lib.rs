//! # ST7567S Display Controller Driver
//!
//! This crate provides a driver for the ST7567S display controller that can be used with Rust embedded projects.
//!
//! # Features
//!
//! - Supports I2C and SPI communication protocols via the [`display_interface`](https://docs.rs/display_interface) crate.
//! - Provides two display modes:
//!   - Direct Write Mode (by default): This mode allows you to write directly to the display memory by calling the [`draw`] method.
//!   - Buffered Mode: This mode allows you to modify an internal buffer by using methods like [`set_pixel`], [`clear`], or by using the [`embedded-graphics`] crate. Once you have made your changes, you can call the [`flush`] method to write the buffer to the display.
//!
//! [`embedded-graphics`]: https://docs.rs/embedded-graphics
//! [`set_pixel`]: crate::display::ST7567S#method.set_pixel
//! [`clear`]: crate::display::ST7567S#method.clear
//! [`flush`]: crate::display::ST7567S#method.flush
//! [`draw`]: crate::display::ST7567S#method.draw
//!
//! # Notes
//! - This driver is designed to work with a more common 128x64 resolution, instead of the original 132x65 resolution of the ST7567S controller.
//! - SPI communication is not tested yet.
//!
//! # Examples
//!
//! ### Direct write mode
//! ```rust
//! use st7567s::{
//!     display::{DirectWriteMode, ST7567S},
//!     interface::{I2CDisplayInterface, I2CInterface},
//! };
//! struct I2CStub;
//! impl embedded_hal::blocking::i2c::Write for I2CStub {
//!     type Error = ();
//!     fn write(&mut self, _addr: u8, _buf: &[u8]) -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! let i2c = I2CStub;
//! let interface = I2CDisplayInterface::new(i2c);
//! let mut display = ST7567S::new(interface);
//! display.init().unwrap();
//!
//! // Set all pixels to enabled state
//! display
//!     .draw([0xff; 128 * 64 / 8].as_slice())
//!     .unwrap();
//!
//! ```
//!
//! ### Buffered mode + embedded_graphics
//! ```rust
//! use st7567s::{
//!     display::{BufferedMode, ST7567S},
//!     interface::{I2CDisplayInterface, I2CInterface},
//! };
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     text::{Baseline, Text},
//! };
//! struct I2CStub;
//! impl embedded_hal::blocking::i2c::Write for I2CStub {
//!     type Error = ();
//!     fn write(&mut self, _addr: u8, _buf: &[u8]) -> Result<(), ()> {
//!         Ok(())
//!     }
//! }
//!
//! let i2c = I2CStub;
//! let interface = I2CDisplayInterface::new(i2c);
//! let mut display = ST7567S::new(interface)
//!     .into_buffered_graphics_mode();
//! display.init().unwrap();
//!
//! let text_style = MonoTextStyleBuilder::new()
//!     .font(&FONT_6X10)
//!     .text_color(BinaryColor::On)
//!     .build();
//!
//! Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
//!     .draw(&mut display)
//!     .unwrap();
//!
//! Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
//!     .draw(&mut display)
//!     .unwrap();
//!
//! display.flush().unwrap();
//! ```

#![no_std]

mod command;
mod consts;
pub mod display;
#[cfg(feature = "graphics")]
pub mod graphics;
pub mod interface;
pub mod prelude;
