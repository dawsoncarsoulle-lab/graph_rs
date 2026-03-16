use crate::graph::Graph;
use rand::RngExt;

fn min(a: f64, b: f64) -> f64 {
    if a > b {
        return b;
    }
    a
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    b
}

pub fn fruchterman_reingold(
    graph: &Graph,
    iterations: usize,
    width: f64,
    height: f64,
) -> Vec<(f64, f64)> {
    let mut rng = rand::rng();
    let mut positions: Vec<(f64, f64)> = (0..graph.node_count())
        .map(|_| (rng.random_range(0.0..width), rng.random_range(0.0..height)))
        .collect();

    let area = width * height;
    let n = graph.node_count();
    let k = (area / n as f64).sqrt();

    fn repulsion(d: f64, k: f64) -> f64 {
        (k * k) / d
    }

    fn attraction(d: f64, k: f64) -> f64 {
        (d * d) / k
    }

    let mut temperature: f64 = width * 0.1;

    for _ in 0..iterations {
        let mut disp: Vec<(f64, f64)> = vec![(0.0, 0.0); n];

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let dx = positions[i].0 - positions[j].0;
                let dy = positions[i].1 - positions[j].1;
                let dist = (dx * dx + dy * dy).sqrt().max(1e-9);
                let force = repulsion(dist, k);
                disp[i].0 += (dx / dist) * force;
                disp[i].1 += (dy / dist) * force;
            }
        }
        for u in 0..n {
            for v in graph.neighbors(u) {
                let dx: f64 = positions[u].0 - positions[v].0;
                let dy: f64 = positions[u].1 - positions[v].1;
                let dist: f64 = (dx * dx + dy * dy).sqrt().max(1e-9);
                let force: f64 = attraction(dist, k);
                disp[u].0 -= (dx / dist) * force;
                disp[u].1 -= (dy / dist) * force;
                // disp[v].0 -= (dx / dist) * force;
                // disp[v].1 -= (dy / dist) * force;
            }
        }

        for i in 0..n {
            let (dx, dy) = disp[i];
            let magnitude = (dx * dx + dy * dy).sqrt().max(1e-9);
            let scale = min(magnitude, temperature) / magnitude;
            positions[i].0 += dx * scale;
            positions[i].1 += dy * scale;
            positions[i].0 = max(0.0, min(width, positions[i].0));
            positions[i].1 = max(0.0, min(height, positions[i].1));
        }

        temperature *= 1.0 - (1.0 / iterations as f64);
    }
    positions
}
