#![forbid(missing_docs)]

//! This program demonstrates writing the K-Means algorithm in Rust.
//! The datapoints are always generated at runtime, and have no real meaning.
//!
//! I wrote this because of my homework.

use glam::Vec2;
use index_vec::IndexVec;
use k_mears::{
    associate_centroids_to_points, calc_sse, generate_datapoints, pick_centroids,
    silhouette::{global_silhouette_coefficient, SilhouetteResult},
    sort_point_vec, update_centroids, KMeansResult, PointIdx,
};

fn k_means(dataset: &IndexVec<PointIdx, Vec2>, k: usize) -> KMeansResult {
    let mut old_centroids = pick_centroids(&dataset, Some(k));

    let mut ctr = 0u8;
    loop {
        ctr += 1;
        let assoc = associate_centroids_to_points(&dataset, &old_centroids);
        let new_centroids = update_centroids(&assoc);

        if sort_point_vec(&new_centroids) == sort_point_vec(&old_centroids) {
            println!("D{ctr}");
            return calc_sse(&assoc, &new_centroids);
        }

        old_centroids = new_centroids;
    }
}

fn k_means_silhouette(dataset: &IndexVec<PointIdx, Vec2>, k: usize) -> SilhouetteResult {
    let mut old_centroids = pick_centroids(&dataset, Some(k));

    let mut ctr = 0u8;
    loop {
        ctr += 1;
        let assoc = associate_centroids_to_points(&dataset, &old_centroids);
        let new_centroids = update_centroids(&assoc);

        if sort_point_vec(&new_centroids) == sort_point_vec(&old_centroids) {
            println!("S{ctr}");
            return SilhouetteResult {
                k,
                coefficient: global_silhouette_coefficient(&assoc, &new_centroids),
            };
        }

        old_centroids = new_centroids;
    }
}

fn main() {
    let dataset = generate_datapoints(Some(10), None, None);

    let mut def_results: Vec<KMeansResult> = vec![];
    let mut sil_results: Vec<SilhouetteResult> = vec![];
    for _ in 1..10 {
        let def_res = k_means(&dataset, 4);
        def_results.push(def_res);
        let sil_res = k_means_silhouette(&dataset, 4);
        sil_results.push(sil_res);
    }

    let def_minimum = def_results.iter().min_by(|x, y| x.partial_cmp(y).unwrap());
    let sil_maximum = sil_results
        .iter()
        .max_by(|x, y| x.coefficient.partial_cmp(&y.coefficient).unwrap());
    println!("K-Means completed with: {def_minimum:?}");
    println!("K-Means completed with: {sil_maximum:?}");
}
