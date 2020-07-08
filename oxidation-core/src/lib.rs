#![crate_name = "oxidation_core"]

#[macro_use]
extern crate bitflags;

pub mod cpu;
pub mod memory;
pub mod instructions;
pub mod registers;
pub mod security_context;
pub mod virtual_machine;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
