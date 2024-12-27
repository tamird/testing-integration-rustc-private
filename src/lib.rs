#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_session;

#[rustversion::since(2024-11-03)]
use rustc_session::config::host_tuple as rustc_host;

#[rustversion::before(2024-11-03)]
use rustc_session::config::host_triple as rustc_host;

pub fn host() -> &'static str {
    rustc_host()
}
