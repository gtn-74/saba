// fn main() {
//     println!("Hello, world!");
// }

#![no_std]
 #![cfg_attr(not(target_os = "linux"), no_main)]

 use noli::prelude::*;

 fn main() {
     Api::write_string("Hello World\n");
     println!("Hello from println!");
     Api::exit(42);
 }
// noli  crateのentry_point!が、main関数を起動
 entry_point!(main);

