#![allow(unused_variables)]
#![allow(dead_code)]

mod aggregator;
mod generics;
mod traits;

pub fn polymorphism() {
    println!("\n\n--- Generics, Traits and Lifetimes ---\n");

    generics::deduplication_with_generics();
    traits::traits();
}
