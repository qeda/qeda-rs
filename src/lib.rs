#[macro_use]
extern crate bitflags;

#[macro_use]
mod log;
mod cli;

#[macro_use]
mod macros;

pub mod config;
pub mod drawing;
pub mod library;

mod component;
mod error;
mod generators;
mod packages;
mod pinout;
mod symbol;
mod symbols;

pub use error::Result;

pub fn run_cli() -> Result<()> {
    cli::run()
}
