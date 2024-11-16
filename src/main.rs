#![forbid(missing_docs)]

//! This program demonstrates writing the K-Means algorithm in Rust.
//! The datapoints are always generated at runtime, and have no real meaning.
//!
//! I wrote this because of my homework.

use k_mea_rs::{
    associate_centroids_to_points, generate_datapoints, pick_centroids, sort_point_vec,
    update_centroids,
};

fn main() {
    let dataset = generate_datapoints(None, None, None);
    let mut old_centroids = pick_centroids(&dataset, None);

    let mut ctr = 1u8;
    loop {
        ctr += 1;
        let assoc = associate_centroids_to_points(&dataset, &old_centroids);
        let mut new_centroids = update_centroids(&assoc);

        if sort_point_vec(&mut new_centroids) == sort_point_vec(&mut old_centroids) {
            break;
        }
    }

    println!("K-Means completed with n={ctr} runs");
}
