use prelude::*;

type Graph = HashMap<String, Vec<String>>;

fn count_paths_from_node_to_end(graph: &Graph, node: &str, visited: &HashSet<String>) -> u64 {
    if visited.contains(node) {
        return 0;
    }
    if node == "end" {
        return 1;
    }

    let mut res = 0;
    for next in &graph[node] {
        if node.chars().next().unwrap().is_uppercase() {
            res += count_paths_from_node_to_end(graph, next, visited);
        } else if node.chars().next().unwrap().is_lowercase() {
            let mut visited = visited.clone();
            visited.insert(node.to_owned());
            res += count_paths_from_node_to_end(graph, node, &visited);
        }
    }
    res
}

fn do_main(input: &str) {
    let mut edges  = Graph::new();
    for line in read_lines_from_file(input) {
        let split = line.split('-').collect_vec();
        let left = split[0];
        let right = split[1];

        for (e0, e1) in [(left, right), (right, left)] {
            edges.entry(e0.to_owned()).or_default().push(e1.to_owned());
        }
    }

    let part1 = count_paths_from_node_to_end(&edges, "start", &HashSet::from(["start".into()]));
    dbg!(part1);
}

fn main() {
    do_main("inputs/day_12.txt");
}
