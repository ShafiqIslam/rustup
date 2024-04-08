#![allow(unused_variables)]
#![allow(unused_mut)]

mod hash_maps;
mod strings;
mod vectors;

pub fn collections() {
    println!("\n\n--- Collections ---\n");

    vectors::vectors();
    strings::strings_are_complicated();
    hash_maps::hash_maps();
}
