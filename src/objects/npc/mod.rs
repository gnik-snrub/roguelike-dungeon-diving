pub mod ai;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    max_hp: i32,
    hp: i32,
    defense: i32,
    power: i32,
}
