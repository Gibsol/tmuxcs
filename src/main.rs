pub mod input;
pub mod topics;

use input::impl_input;
use std::ptr::read;

fn main() {
    impl_input();
}
