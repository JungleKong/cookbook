pub mod math;
pub mod collection;
pub mod webserver;
pub mod random;
pub mod macros;
pub mod ffi;
pub mod learn;
pub mod leetcode;
// pub use math::*;

// use math::basic::add;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = math::basic::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_env() {
        use std::env;
        let binary_path = env::var("HOME").unwrap() + "/.cargo/bin/rustfmt";
        println!("{binary_path}");
    }
}
