#![feature(plugin, custom_derive, decl_macro)]
#![plugin(rocket_codegen)]

extern crate web3;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod server;
mod adchain_registry;
