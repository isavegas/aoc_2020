#![feature(str_split_once)]

pub trait AoCDay {
    fn day(&self) -> usize;
    fn part1(&self) -> Result<String, ErrorWrapper>;
    fn part2(&self) -> Result<String, ErrorWrapper>;
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>);
}

pub fn block_char() -> char {
    std::char::from_u32(9608).unwrap()
}

pub mod parse;

mod vec3;
pub use vec3::Vec3;

mod vec2;
pub use vec2::Vec2;

mod error;
pub use error::ErrorWrapper;

mod day;
pub use day::get_days;

pub mod math;
