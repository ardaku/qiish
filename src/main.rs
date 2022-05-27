//! Not really a crate :P
#![warn(
clippy::all,
clippy::restriction,
clippy::pedantic,
clippy::nursery,
clippy::cargo,
)]

#![allow(clippy::implicit_return)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::print_stdout)]
#![allow(clippy::blanket_clippy_restriction_lints)]

use std::path::Path;

/// Actual qiish bit
#[path="qiish/qiish.rs"]
mod qiish;

fn main() {

    println!("{}", qiish::rewrite_relative_dir(Path::new("~/qiishhome").to_owned(), Path::new("/Users/will").to_owned(), Path::new("/Users/will/qiishhome").to_owned()).unwrap().to_str().unwrap());

    if let Ok(qiish) = qiish::Qiish::new(Path::new("/Users/will/qiishhome")) {
        qiish
    } else {
        println!("Mate that aint gonna work sorry bro");
        return;
    }.call_qiish(0);
}
