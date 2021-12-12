use prelude::*;

type Graph = HashMap<String, Vec<String>>;

fn count_paths_from_node_to_end(
    graph: &Graph,
    node: &str,
    visits: &HashMap<String, usize>,
    max_visits_to_small: usize,
) -> u64 {
    if node == "end" {
        return 1;
    }

    let mut res = 0;
    for next in &graph[node] {
        if visits.get(next).cloned().unwrap_or_default() >= max_visits_to_small {
            continue;
        }

        if node.chars().next().unwrap().is_uppercase() {
            res += count_paths_from_node_to_end(graph, next, visits, max_visits_to_small);
        } else if node.chars().next().unwrap().is_lowercase() {
            let mut visits = visits.clone();
            *visits.entry(node.to_owned()).or_default() += 1;
            res += count_paths_from_node_to_end(graph, next, &visits, max_visits_to_small);
        }
    }
    res
}

fn do_main(input: &str) {
    let mut edges = Graph::new();
    for line in read_lines_from_file(input) {
        let split = line.split('-').collect_vec();
        let left = split[0];
        let right = split[1];

        for (e0, e1) in [(left, right), (right, left)] {
            edges.entry(e0.to_owned()).or_default().push(e1.to_owned());
        }
    }

    let part1 = count_paths_from_node_to_end(&edges, "start", &Default::default(), 1);
    dbg!(part1);

    let part2 = count_paths_from_node_to_end(&edges, "start", &Default::default(), 2);
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_12.txt");
}
