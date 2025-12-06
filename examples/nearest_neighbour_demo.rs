use graphs_algorithms::graphs::{nearest_neighbour, MAX};

fn main() {
    let mut graph = vec![
        vec![MAX, 1, 2, 4, 3],
        vec![1, MAX, 7, 2, 5],
        vec![2, 7, MAX, 8, 1],
        vec![4, 2, 8, MAX, 6],
        vec![3, 5, 1, 6, MAX],
    ];

    let solution = nearest_neighbour(&graph, 0);

    assert_eq!(solution.cost, 12);
    assert_eq!(solution.route[0], 0);
    assert_eq!(solution.route.last(), Some(2).as_ref());
}
