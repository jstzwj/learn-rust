use indicatif::ProgressBar;
use rand::Rng;
use std::f64::consts::PI;

fn is_in_same_semicircle(points: &[(f64, f64)]) -> bool {
    if points.len() < 2 {
        panic!("至少需要两个点");
    }
    // 思路是这样，任意选择一个点到圆心连线
    // 然后利用叉乘判断剩余三个点都在直线同侧

    // 计算圆心到p1, p2, p3...的向量
    let vector_list: Vec<(f64, f64)> = points.iter().map(|p| (p.0, p.1)).collect();

    // 依次选择参考点
    for i in 0..points.len() {
        let p0 = points[i];
        let v0 = (p0.0, p0.1);

        // 计算叉乘
        let cross_product_list: Vec<f64> = vector_list.iter().map(|v| v.0 * v0.1 - v.1 * v0.0).collect();

        // 判断 p1, p2, p3 是否在同一侧
        let same_side = cross_product_list.iter().all(|&x| x >= 0.0) || cross_product_list.iter().all(|&x| x <= 0.0);

        // 只要满足一次，就返回true
        if same_side {
            return true;
        }
    }

    false
}

fn monte_carlo_simulation(trials: usize) -> f64 {
    let mut same_semicircle_count = 0;
    let mut rng = rand::thread_rng();
    
    // 创建一个进度条
    let pb = ProgressBar::new(trials as u64);

    for _ in 0..trials {
        let mut points = Vec::new();

        while points.len() < 5 {
            let x: f64 = rng.gen_range(-1.0..=1.0);
            let y: f64 = rng.gen_range(-1.0..=1.0);

            // 检查是否在圆内
            if x.powi(2) + y.powi(2) <= 1.0 {
                points.push((x, y));
            }
        }

        if is_in_same_semicircle(&points) {
            same_semicircle_count += 1;
        }
        
        // 每次迭代更新进度条
        pb.inc(1);
    }
    
    // 结束进度条
    pb.finish_with_message("Simulation complete");

    same_semicircle_count as f64 / trials as f64
}

fn main() {
    let trials = 100_000_000;
    let probability = monte_carlo_simulation(trials);
    println!("在同一半圆内的概率: {:.6}", probability);
}
