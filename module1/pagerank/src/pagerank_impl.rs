// The PageRank struct holds the damping factor and the number of iterations to run the algorithm.
pub struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    pub fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    fn _get_new_ranks(&self, graph: &[Vec<usize>], ranks: &[f64]) -> Vec<f64> {
        let mut new_ranks = vec![0.0; graph.len()];

        for (node, edges) in graph.iter().enumerate() {
            let contribution = ranks[node] / (edges.len() as f64);

            for &edge in edges {
                new_ranks[edge] += contribution;
            }
        }

        new_ranks
    }

    pub fn rank(&self, graph: &[Vec<usize>]) -> Vec<f64> {
        let node_count = graph.len();

        // The initial PageRank value for each node.
        let mut ranks = vec![1.0 / (node_count as f64); node_count];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            let mut new_ranks = self._get_new_ranks(graph, &ranks);

            // Updates the PageRank values using the damping factor.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (node_count as f64);
            }
            ranks = new_ranks;
        }

        ranks
    }
}
