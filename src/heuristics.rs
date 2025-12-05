pub fn nearest_neighbour(mut graph: Vec<Vec<usize>>, start: usize) {
    // TODO: if we'll only use integer nodes then we should use a Vec<bool> to use less memory.
    // TODO: otherwise, we use a HashSet<N> and check by .contains()

    // Estou fazendo bem diretamente sem muitas abstrações em razão de uma suposta eficiência!!

    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut path: Vec<usize> = Vec::new();
    path.push(start);

    let mut cost = 0;
    let mut current_node = start;
    visited[current_node] = true;
    let mut visited_count = 1;

    let mut better_cost = usize::MAX;
    let mut next_on_path: Option<usize> = None;

    while visited_count != graph.len() {
        for (i, val) in visited.iter().enumerate() {
            if !*val || current_node == i {
                continue;
            }

            if graph[current_node][i] < better_cost {
                better_cost = graph[current_node][i];
                next_on_path = Some(i);
            }
        }

        if next_on_path.is_some() {
            let n = next_on_path.unwrap();
            path.push(n);
            cost += better_cost;
            current_node = n;
            visited[n] = true;
            visited_count += 1;
        }
    }

    // Completa o ciclo
    cost += graph[current_node][start];

    // Adicionar ou não novamente o vértice inicial? Matheus comentou que não era interessante...
}
