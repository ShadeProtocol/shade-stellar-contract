#![no_std]
// Soroban contract entrypoints (invoices, events, …) legitimately take more
// than 7 arguments; allow it crate-wide, including macro-generated clients.
#![allow(clippy::too_many_arguments)]
pub mod components;
pub mod errors;
pub mod events;
pub mod shade;
pub mod shade_interface;
pub mod types;

#[cfg(test)]
pub mod tests;
