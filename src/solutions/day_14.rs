use crate::{vec2::Vec2i, Answer};

#[derive(Debug, PartialEq)]
struct Robot {
    p: Vec2i,
    v: Vec2i,
}

fn parse_vec2(data: &str) -> Vec2i {
    let mut parts = data.split(",");
    let x = parts.next().map(str::parse::<i64>).unwrap().unwrap();
    let y = parts.next().map(str::parse::<i64>).unwrap().unwrap();
    Vec2i { x, y }
}

fn parse_robot(line: &str) -> Robot {
    let mut parts = line.trim().split(" ");
    let p_part = parts.next().unwrap();
    let v_part = parts.next().unwrap();
    assert_eq!(&p_part[..2], "p=");
    let p = parse_vec2(&p_part[2..]);
    assert_eq!(&v_part[..2], "v=");
    let v = parse_vec2(&v_part[2..]);
    Robot { p, v }
}

fn simulate_robot(p: Vec2i, v: Vec2i, h: usize, w: usize, t: usize) -> Vec2i {
    let mut pn = p + v * t as i64;
    pn.x = pn.x.rem_euclid(w as i64);
    pn.y = pn.y.rem_euclid(h as i64);
    pn
}

fn solve_part_a(input: &str, h: usize, w: usize) -> Answer {
    let robots = input.trim().lines().map(parse_robot).collect::<Vec<_>>();

    let mut n_tl = 0;
    let mut n_tr = 0;
    let mut n_br = 0;
    let mut n_bl = 0;
    for robot in robots.iter() {
        let new_pos = simulate_robot(robot.p, robot.v, h, w, 100);
        let top = new_pos.y < h as i64 / 2;
        let bot = new_pos.y > h as i64 / 2;
        let left = new_pos.x < w as i64 / 2;
        let rght = new_pos.x > w as i64 / 2;
        if top & left {
            n_tl += 1;
        } else if top & rght {
            n_tr += 1;
        } else if rght & bot {
            n_br += 1;
        } else if bot & left {
            n_bl += 1;
        }
    }

    Answer::Number(n_tl * n_tr * n_br * n_bl)
}

pub fn part_a(input: &str) -> Answer {
    solve_part_a(input, 103, 101)
}

pub fn part_b(input: &str) -> Answer {
    let _ = input;
    Answer::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn test_part_a() {
        let result = solve_part_a(TEST_INPUT, 7, 11);
        assert_eq!(result, Answer::Number(12));
    }
}
