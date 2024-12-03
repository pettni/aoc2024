use crate::Solutions;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod template;

pub const ALL: [Solutions; 3] = [
    (day_01::part_a, day_01::part_b),
    (day_02::part_a, day_02::part_b),
    (day_03::part_a, day_03::part_b),
];
