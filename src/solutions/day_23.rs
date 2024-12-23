use crate::hash::*;

use crate::Answer;

fn parse(input: &str) -> (FxHashSet<&str>, FxHashMap<&str, FxHashSet<&str>>) {
    let edges = input
        .trim()
        .lines()
        .map(|l| {
            let mut spl = l.split("-");
            (spl.next().unwrap(), spl.next().unwrap())
        })
        .collect::<Vec<_>>();

    // enumerate edges
    let mut nodes: FxHashSet<&str> = FxHashSet::new();
    for (a, b) in edges.iter() {
        nodes.insert(a);
        nodes.insert(b);
    }

    // create adjacency lists
    let mut adj: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::new();
    for (a, b) in edges.iter() {
        adj.entry(*a).or_default().insert(b);
        adj.entry(*b).or_default().insert(a);
    }

    (nodes, adj)
}

pub fn part_a(input: &str) -> Answer {
    let (nodes, adj) = parse(input);
    // find number of 3-cliques that contain a 't' node
    let mut result = 0;
    for a in nodes.iter() {
        for b in &adj[a] {
            if b <= a {
                continue;
            }
            for c in adj[a].intersection(&adj[b]) {
                if c <= b {
                    continue;
                }
                if a.as_bytes()[0] == b't' || b.as_bytes()[0] == b't' || c.as_bytes()[0] == b't' {
                    result += 1;
                }
            }
        }
    }
    Answer::Number(result)
}

pub fn part_b(input: &str) -> Answer {
    let (nodes, adj) = parse(input);
    // find largest clique
    let mut queue: Vec<(Vec<&str>, FxHashSet<&str>)> = Vec::new();
    for a in nodes.iter() {
        let mut adj_a = adj[a].clone();
        adj_a.retain(|x| a < x);
        queue.push((vec![a], adj_a));
    }

    let mut best_clique: Vec<&str> = vec![];
    while let Some((clique, clique_adj)) = queue.pop() {
        if clique.len() > best_clique.len() {
            best_clique = clique.clone();
        }
        // See if we can still reach target
        if clique.len() + clique_adj.len() < best_clique.len() {
            continue;
        }
        // Branch to all possible extensions
        for y in clique_adj.iter() {
            let mut new_clique = clique.clone();
            new_clique.push(y);
            let mut new_clique_adj = clique_adj.clone();
            new_clique_adj.retain(|z| y < z && adj[y].contains(z));
            queue.push((new_clique, new_clique_adj));
        }
    }

    Answer::String(best_clique.join(",").leak::<'static>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(7));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::String("co,de,ka,ta"));
    }
}
