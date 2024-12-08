#![cfg(debug_assertions)]
#![allow(missing_docs)]

use std::collections::HashMap;

use index_vec::IndexVec;

use crate::{CentroidDistance, CentroidIdx, Point, PointIdx};

#[derive(Debug)]
pub struct SilhouetteResult {
    pub k: usize,
    pub coefficient: f32,
}

pub fn silhouette_coefficient(
    point: &Point,
    centroid_idx: &CentroidIdx,
    assoc: &HashMap<CentroidIdx, IndexVec<PointIdx, &Point>>,
    centroids: &IndexVec<CentroidIdx, Point>,
) -> f32 {
    let a = avg_distance_to_point_in_cluster(
        point,
        &assoc.get(&centroid_idx).expect("valid centroid index"),
    );

    let next_cluster = next_cluster(centroid_idx, &centroids);
    let b = avg_distance_to_point_in_cluster(point, &assoc.get(&next_cluster).unwrap());

    (b - a) / [a, b].iter().max_by(|a, b| a.total_cmp(&b)).unwrap()
}

pub fn global_silhouette_coefficient(
    assoc: &HashMap<CentroidIdx, IndexVec<PointIdx, &Point>>,
    centroids: &IndexVec<CentroidIdx, Point>,
) -> f32 {
    let mut coefficient_sum = 0f32;
    let mut point_count = 0usize;

    for (centroid_idx, cluster) in assoc {
        for point in cluster {
            coefficient_sum += silhouette_coefficient(point, centroid_idx, assoc, centroids);
            point_count += 1;
        }
    }

    coefficient_sum / point_count as f32
}

fn avg_distance_to_point_in_cluster(point: &Point, cluster: &&IndexVec<PointIdx, &Point>) -> f32 {
    let mut distance_sum = 0f32;

    for c_point in cluster.iter() {
        distance_sum += point.distance(**c_point);
    }

    distance_sum / cluster.iter().len() as f32
}

fn next_cluster(
    centroid_idx: &CentroidIdx,
    centroids: &&IndexVec<CentroidIdx, Point>,
) -> CentroidIdx {
    let mut distances: Vec<CentroidDistance> = vec![];
    let main_centroid = centroids.get(*centroid_idx).expect("valid centroid index");

    for (idx, centroid) in centroids.iter_enumerated() {
        if &idx == centroid_idx {
            continue;
        }

        distances.push(CentroidDistance {
            idx,
            distance: centroid.distance(*main_centroid),
        });
    }

    distances
        .into_iter()
        .min_by(|a, b| a.distance.total_cmp(&b.distance))
        .unwrap()
        .idx
}

// random traits i wanna write
// average trait for iterators
