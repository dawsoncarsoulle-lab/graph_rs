pub struct Graph {
    adjency_matrix: Vec<Vec<Option<usize>>>,
    positions: Vec<(f64, f64)>,
    //n: usize,
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        Graph {
            adjency_matrix: vec![vec![None; n]; n],
            positions: Vec::new(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.adjency_matrix.len()
    }

    pub fn add_edge(&mut self, i: usize, j: usize, weight: usize) -> &mut Self {
        self.adjency_matrix[i][j] = Some(weight);
        self
    }

    pub fn add_undirected_edge(&mut self, i: usize, j: usize, weight: usize) -> &mut Self {
        self.add_edge(i, j, weight);
        self.add_edge(j, i, weight);
        self
    }

    pub fn neighbors(&self, i: usize) -> Vec<usize> {
        self.adjency_matrix[i]
            .iter()
            .enumerate()
            .filter(|(_, val)| val.is_some())
            .map(|(j, _)| j)
            .collect()
    }
}
