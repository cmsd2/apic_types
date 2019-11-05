#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate bitflags;

pub mod local;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
