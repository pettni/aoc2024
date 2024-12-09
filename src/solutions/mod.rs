use crate::Solutions;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;

pub mod day_xx;

pub const ALL: [Solutions; 9] = [
    (day_01::part_a, day_01::part_b),
    (day_02::part_a, day_02::part_b),
    (day_03::part_a, day_03::part_b),
    (day_04::part_a, day_04::part_b),
    (day_05::part_a, day_05::part_b),
    (day_06::part_a, day_06::part_b),
    (day_07::part_a, day_07::part_b),
    (day_08::part_a, day_08::part_b),
    (day_09::part_a, day_09::part_b),
];
