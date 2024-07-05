use crate::linalg;
use rand::distributions::Distribution;

pub fn generate(n_points: usize, n_dimensions: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();
    let normal = rand::distributions::Standard;

    let mut points = Vec::with_capacity(n_points);
    for _ in 0..n_points {
        let mut point: Vec<f32> = normal.sample_iter(&mut rng).take(n_dimensions).collect();
        let magnitude = linalg::dot(&point,&point).sqrt();
        for x in &mut point {
            *x /= magnitude;
        }
        points.push(point);
    }

    // Simplified optimization
    let iterations = 100;
    let step_size = 0.1;

    for _ in 0..iterations {
        let mut forces = vec![vec![0.0; n_dimensions]; n_points];
        for i in 0..n_points {
            for j in (i+1)..n_points {
                let diff: Vec<f32> = points[i].iter().zip(&points[j])
                    .map(|(&a, &b)| a - b)
                    .collect();
                let dist_sq: f32 = diff.iter().map(|&x| x * x).sum();
                let force = diff.into_iter().map(|x| x / dist_sq).collect::<Vec<f32>>();
                for k in 0..n_dimensions {
                    forces[i][k] += force[k];
                    forces[j][k] -= force[k];
                }
            }
        }

        for i in 0..n_points {
            for k in 0..n_dimensions {
                points[i][k] += step_size * forces[i][k];
            }
            let magnitude = linalg::dot(&points[i],&points[i]).sqrt();
            for x in &mut points[i] {
                *x /= magnitude;
            }
        }
    }

    points
}

