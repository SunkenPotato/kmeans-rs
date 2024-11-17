#![forbid(missing_docs)]

//! This program demonstrates writing the K-Means algorithm in Rust.
//! The datapoints are always generated at runtime, and have no real meaning.
//!
//! I wrote this because of my homework.

use glam::Vec2;
use index_vec::IndexVec;
use k_mears::{
    associate_centroids_to_points, calc_sse, generate_datapoints, pick_centroids, sort_point_vec,
    update_centroids, KMeansResult, PointIdx,
};

fn k_means(dataset: &IndexVec<PointIdx, Vec2>, k: usize) -> KMeansResult {
    let mut old_centroids = pick_centroids(&dataset, Some(k));

    loop {
        let assoc = associate_centroids_to_points(&dataset, &old_centroids);
        let mut new_centroids = update_centroids(&assoc);

        if sort_point_vec(&mut new_centroids) == sort_point_vec(&mut old_centroids) {
            return calc_sse(&assoc, &new_centroids);
        }
    }
}

fn main() {
    let dataset = generate_datapoints(Some(10), None, None);

    let mut results: Vec<KMeansResult> = vec![];
    for _ in 1..10 {
        let res = k_means(&dataset, 4);
        results.push(res);
    }

    let minimum = results.iter().min_by(|x, y| x.partial_cmp(y).unwrap());

    println!("K-Means completed with: {minimum:?}");
}
