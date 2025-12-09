use std::collections::HashSet;

enum SearchType {
    BestImp,
    FirstImp,
}

trait LocalSearch {
    fn swap(&self, graph: &Vec<Vec<usize>>, start: usize, imp: SearchType) -> Self;
    fn two_opt(&self, graph: &Vec<Vec<usize>>, imp: SearchType) -> Self;
    fn shift(&self, graph: &Vec<Vec<usize>>, start: usize, imp: SearchType) -> Self;
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Solution {
    pub route: Vec<usize>,
    pub cost: usize,
}

impl Solution {
    fn neighbourhood_by_swap(&self, graph: &Vec<Vec<usize>>, start: usize) -> HashSet<Self> {
        let mut solutions: HashSet<Solution> = HashSet::new();

        for v in &self.route {
            let mut new_route = self.route.clone();

            let tmp = new_route[start];
            new_route[start] = new_route[*v];
            new_route[*v] = tmp;

            let cost = new_route
                .windows(2)
                .map(|w| graph[w[0]][w[1]])
                .sum::<usize>()
                + graph[new_route[new_route.len() - 1]][new_route[0]];

            solutions.insert(Self {
                route: new_route,
                cost,
            });
        }

        solutions
    }

    fn neighbourhood_by_shift(&self, graph: &Vec<Vec<usize>>, start: usize) -> Vec<Self> {
        let mut solutions: Vec<Solution> = Vec::new();
        let n = self.route.len();

        if start >= n {
            return solutions;
        }

        for target_pos in 0..n {
            if target_pos == start {
                continue;
            }

            let mut new_route = self.route.clone();
            let elem = new_route.remove(start);
            new_route.insert(target_pos, elem);

            let cost = new_route
                .windows(2)
                .map(|w| graph[w[0]][w[1]])
                .sum::<usize>()
                + graph[new_route[new_route.len() - 1]][new_route[0]];

            solutions.push(Self {
                route: new_route,
                cost,
            });
        }

        solutions
    }
}

impl LocalSearch for Solution {
    fn swap(&self, graph: &Vec<Vec<usize>>, start: usize, imp: SearchType) -> Self {
        let mut best_solution: Solution = self.clone();
        let mut found_better_solution = true;

        while found_better_solution {
            found_better_solution = false;

            let solutions = best_solution.neighbourhood_by_swap(&graph, start);

            match imp {
                SearchType::FirstImp => {
                    for s in solutions.iter() {
                        if s.cost < best_solution.cost {
                            found_better_solution = true;
                            best_solution = s.clone();
                            break;
                        }
                    }
                }
                SearchType::BestImp => {
                    let mut tmp_solution = best_solution.clone();

                    for s in solutions.iter() {
                        if s.cost < tmp_solution.cost {
                            tmp_solution = s.clone();
                        }
                    }

                    if tmp_solution.cost < best_solution.cost {
                        best_solution = tmp_solution;
                        found_better_solution = true;
                    }
                }
            }
        }
        best_solution
    }

    fn two_opt(&self, graph: &Vec<Vec<usize>>, imp: SearchType) -> Self {
        self.clone()
    }

    fn shift(&self, graph: &Vec<Vec<usize>>, start: usize, imp: SearchType) -> Self {
        let mut best_solution: Solution = self.clone();
        let mut found_better_solution = true;

        while found_better_solution {
            found_better_solution = false;

            let solutions = best_solution.neighbourhood_by_shift(&graph, start);

            match imp {
                SearchType::FirstImp => {
                    for s in solutions.iter() {
                        if s.cost < best_solution.cost {
                            found_better_solution = true;
                            best_solution = s.clone();
                            break;
                        }
                    }
                }
                SearchType::BestImp => {
                    let mut tmp_solution = best_solution.clone();

                    for s in solutions.iter() {
                        if s.cost < tmp_solution.cost {
                            tmp_solution = s.clone();
                        }
                    }

                    if tmp_solution.cost < best_solution.cost {
                        best_solution = tmp_solution;
                        found_better_solution = true;
                    }
                }
            }
        }
        best_solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_1_first_imp_test_1() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        solution = solution.swap(&graph, 0, SearchType::FirstImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_first_imp_test_2() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 3, 4, 2],
            cost: 12,
        };

        solution = solution.swap(&graph, 0, SearchType::FirstImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_first_imp_test_3() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        solution = solution.swap(&graph, 2, SearchType::FirstImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_best_imp_test_1() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 3, 4, 2],
            cost: 12,
        };

        solution = solution.swap(&graph, 0, SearchType::BestImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_best_imp_test_2() {
        let graph = vec![
            vec![usize::MAX, 1, 2, 4, 3],
            vec![1, usize::MAX, 7, 2, 5],
            vec![2, 7, usize::MAX, 8, 1],
            vec![4, 2, 8, usize::MAX, 6],
            vec![3, 5, 1, 6, usize::MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        solution = solution.swap(&graph, 2, SearchType::BestImp);
        assert_eq!(solution.cost, 12);
    }

    // ----- novos testes para shift -----

    #[test]
    fn graph_1_shift_first_imp_1() {
        let graph = vec![
            vec![MAX, 1, 2, 4, 3],
            vec![1, MAX, 7, 2, 5],
            vec![2, 7, MAX, 8, 1],
            vec![4, 2, 8, MAX, 6],
            vec![3, 5, 1, 6, MAX],
        ];

        // mover o elemento na posição 2 (valor 2) para a posição final gera [0,1,3,4,2] (custo 12)
        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        solution = solution.shift(&graph, 2, SearchType::FirstImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_shift_first_imp_no_change() {
        let graph = vec![
            vec![MAX, 1, 2, 4, 3],
            vec![1, MAX, 7, 2, 5],
            vec![2, 7, MAX, 8, 1],
            vec![4, 2, 8, MAX, 6],
            vec![3, 5, 1, 6, MAX],
        ];

        // rota já ótima para o exemplo; shift não deve piorar
        let mut solution = Solution {
            route: vec![0, 1, 3, 4, 2],
            cost: 12,
        };

        solution = solution.shift(&graph, 0, SearchType::FirstImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_shift_best_imp_1() {
        let graph = vec![
            vec![MAX, 1, 2, 4, 3],
            vec![1, MAX, 7, 2, 5],
            vec![2, 7, MAX, 8, 1],
            vec![4, 2, 8, MAX, 6],
            vec![3, 5, 1, 6, MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        solution = solution.shift(&graph, 2, SearchType::BestImp);
        assert_eq!(solution.cost, 12);
    }

    #[test]
    fn graph_1_shift_start_out_of_bounds() {
        let graph = vec![
            vec![MAX, 1, 2, 4, 3],
            vec![1, MAX, 7, 2, 5],
            vec![2, 7, MAX, 8, 1],
            vec![4, 2, 8, MAX, 6],
            vec![3, 5, 1, 6, MAX],
        ];

        let mut solution = Solution {
            route: vec![0, 1, 2, 3, 4],
            cost: 25,
        };

        // start >= n deve deixar a solução inalterada
        solution = solution.shift(&graph, 10, SearchType::FirstImp);
        assert_eq!(solution.cost, 25);
    }
}
