use prelude::*;

type Graph = HashMap<String, Vec<String>>;

fn count_paths_from_node_to_end(
    graph: &Graph,
    node: &str,
    visited: &HashSet<String>,
    visited_twice: Option<&str>,
) -> u64 {
    if node == "end" {
        return 1;
    }

    let mut res = 0;
    for next in &graph[node] {
        if next == "start" {
            continue;
        }

        if node.chars().next().unwrap().is_uppercase() {
            res += count_paths_from_node_to_end(graph, next, visited, visited_twice);
        } else if node.chars().next().unwrap().is_lowercase() {
            match (visited.contains(node), visited_twice) {
                (true, None) => {
                    res += count_paths_from_node_to_end(graph, next, visited, Some(node))
                }
                (true, Some(_)) => continue,
                (false, vt) => {
                    let mut visited = visited.clone();
                    visited.insert(node.to_owned());
                    res += count_paths_from_node_to_end(graph, next, &visited, vt);
                }
            }
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

    let part1 = count_paths_from_node_to_end(
        &edges,
        "start",
        &Default::default(),
        Some("don't let anything else visit twice"),
    );
    dbg!(part1);

    let part2 = count_paths_from_node_to_end(&edges, "start", &Default::default(), None);
    dbg!(part2);
}

fn main() {
    do_main("inputs/day_12.txt");
}
