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
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

pub mod day_xx;

pub const ALL: [Solutions; 25] = [
    (day_01::part_a, day_01::part_b),
    (day_02::part_a, day_02::part_b),
    (day_03::part_a, day_03::part_b),
    (day_04::part_a, day_04::part_b),
    (day_05::part_a, day_05::part_b),
    (day_06::part_a, day_06::part_b),
    (day_07::part_a, day_07::part_b),
    (day_08::part_a, day_08::part_b),
    (day_09::part_a, day_09::part_b),
    (day_10::part_a, day_10::part_b),
    (day_11::part_a, day_11::part_b),
    (day_12::part_a, day_12::part_b),
    (day_13::part_a, day_13::part_b),
    (day_14::part_a, day_14::part_b),
    (day_15::part_a, day_15::part_b),
    (day_16::part_a, day_16::part_b),
    (day_17::part_a, day_17::part_b),
    (day_18::part_a, day_18::part_b),
    (day_19::part_a, day_19::part_b),
    (day_20::part_a, day_20::part_b),
    (day_21::part_a, day_21::part_b),
    (day_22::part_a, day_22::part_b),
    (day_23::part_a, day_23::part_b),
    (day_24::part_a, day_24::part_b),
    (day_25::part_a, day_25::part_b),
];

#[cfg(test)]
mod tests {
    use crate::get_default_data_path;
    use std::{fs, path::PathBuf};

    use super::*;
    use crate::Answer;

    #[rustfmt::skip]
    const ANSWERS: [(Answer, Answer); 25] = [
        /* day 01 */ (Answer::Number(1590491), Answer::Number(22588371)),
        /* day 02 */ (Answer::Number(442), Answer::Number(493)),
        /* day 03 */ (Answer::Number(183669043), Answer::Number(59097164)),
        /* day 04 */ (Answer::Number(2571), Answer::Number(1992)),
        /* day 05 */ (Answer::Number(4959), Answer::Number(4655)),
        /* day 06 */ (Answer::Number(4964), Answer::Number(1740)),
        /* day 07 */ (Answer::Number(3351424677624), Answer::Number(204976636995111)),
        /* day 08 */ (Answer::Number(423), Answer::Number(1287)),
        /* day 09 */ (Answer::Number(6359213660505), Answer::Number(6381624803796)),
        /* day 10 */ (Answer::Number(652), Answer::Number(1432)),
        /* day 11 */ (Answer::Number(185205), Answer::Number(221280540398419)),
        /* day 12 */ (Answer::Number(1465112), Answer::Number(893790)),
        /* day 13 */ (Answer::Number(37128), Answer::Number(74914228471331)),
        /* day 14 */ (Answer::Number(236628054), Answer::Number(7584)),
        /* day 15 */ (Answer::Number(1465523), Answer::Number(1471049)),
        /* day 16 */ (Answer::Number(143580), Answer::Number(645)),
        /* day 17 */ (Answer::String("3,6,7,0,5,7,3,1,4"), Answer::Number(164278496489149)),
        /* day 18 */ (Answer::Number(360), Answer::String("58,62")),
        /* day 19 */ (Answer::Number(347), Answer::Number(919219286602165)),
        /* day 20 */ (Answer::Number(1289), Answer::Number(982425)),
        /* day 21 */ (Answer::Number(164960), Answer::Number(205620604017764)),
        /* day 22 */ (Answer::Number(14622549304), Answer::Number(1735)),
        /* day 23 */ (Answer::Number(1227), Answer::String("cl,df,ft,ir,iy,ny,qp,rb,sh,sl,sw,wm,wy")),
        /* day 24 */ (Answer::Number(69201640933606), Answer::Unimplemented),
        /* day 25 */ (Answer::Number(3242), Answer::Unimplemented),
    ];

    #[test]
    #[ignore]
    fn test_all() {
        for day in 1..ALL.len() + 1 {
            let (part_a, part_b) = ALL
                .get(day.saturating_sub(1))
                .unwrap_or_else(|| panic!("Invalid day {}", day));

            let path: PathBuf = get_default_data_path(day as u32);
            let data = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("Couldn't open file {:?}", path));

            let (exp_a, exp_b) = &ANSWERS[day - 1];

            let out_a = part_a(data.as_str());
            assert_eq!(out_a, *exp_a);

            let out_b = part_b(data.as_str());
            assert_eq!(out_b, *exp_b);
        }
    }
}
