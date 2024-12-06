use crate::Answer;
use itertools::Itertools;
use std::cmp::Ordering;

fn read_input(input: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let lines: Vec<_> = input.lines().collect();
    let mut split_iter = lines.split(|x| x.is_empty());

    let ordering: Vec<_> = split_iter
        .next()
        .unwrap()
        .iter()
        .map(|x| {
            x.split("|")
                .flat_map(str::parse::<i64>)
                .collect_tuple::<(i64, i64)>()
                .unwrap()
        })
        .collect();

    let updates: Vec<_> = split_iter
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            line.split(",")
                .flat_map(str::parse::<i64>)
                .collect::<Vec<i64>>()
        })
        .collect();

    (ordering, updates)
}

struct Graph {
    edges: Vec<Vec<bool>>,
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        Graph {
            edges: vec![vec![false; n]; n],
        }
    }

    pub fn create_from_edges(edges: &[(i64, i64)]) -> Graph {
        let n = edges.iter().map(|(a, b)| *a.max(b)).max().unwrap() + 1;
        let mut ret = Graph::new(n as usize);
        for (ei, eo) in edges.iter() {
            ret.add_edge(*ei as usize, *eo as usize);
        }
        ret
    }

    pub fn add_edge(&mut self, e_in: usize, e_out: usize) {
        self.edges[e_in][e_out] = true;
    }

    pub fn is_edge(&self, e_in: usize, e_out: usize) -> bool {
        self.edges[e_in][e_out]
    }
}

fn graph_ordering(graph: &Graph, a: &i64, b: &i64) -> Ordering {
    if graph.is_edge(*a as usize, *b as usize) {
        return Ordering::Less;
    } else if graph.is_edge(*b as usize, *a as usize) {
        return Ordering::Greater;
    }
    Ordering::Equal
}

pub fn part_a(input: &str) -> Answer {
    let (ordering, updates) = read_input(input);
    let graph = Graph::create_from_edges(&ordering);
    let res = updates
        .iter()
        .filter(|update| update.is_sorted_by(|a, b| graph_ordering(&graph, a, b) == Ordering::Less))
        .map(|update| update[update.len() / 2])
        .sum();
    Answer::Number(res)
}

pub fn part_b(input: &str) -> Answer {
    let (ordering, mut updates) = read_input(input);
    let graph = Graph::create_from_edges(&ordering);
    let mut unordered_updates: Vec<_> = updates
        .iter_mut()
        .filter(|update| {
            !update.is_sorted_by(|a, b| graph_ordering(&graph, a, b) == Ordering::Less)
        })
        .collect();
    for update in &mut unordered_updates {
        update.sort_by(|a, b| graph_ordering(&graph, a, b));
    }
    let res = unordered_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    Answer::Number(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(143));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(123));
    }
}
