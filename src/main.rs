#![allow(dead_code)]

mod nativization;
mod tokenization;
mod consts;

use crate::nativization::replacement::preprocess;

fn main() {
    print!("{}", preprocess("ddiddle"));
}
