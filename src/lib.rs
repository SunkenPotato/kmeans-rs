#![forbid(missing_docs)]
//! Extensions and methods for the K-Means algorithm
//!

use std::collections::HashMap;

use glam::Vec2;
use index_vec::{index_vec, IndexVec};
use rand::{seq::SliceRandom, thread_rng, Rng};

/// The default amount of clusters to generate
const DEFAULT_K: usize = 4;
/// The default number of points to generate
const DEFAULT_N_SAMP: usize = 100;
/// The standard deviation, i.e. how far away the points are from their centroids when being generated
const DEFAULT_STD: f32 = 0.8;

/// An alias to `glam::f32::Vec2`, representing a point in 2D space
pub type Point = Vec2;

index_vec::define_index_type! {
    /// Index for `IndexVec`s with Points, such as the \[generated\] dataset
    pub struct PointIdx = usize;
}

index_vec::define_index_type! {
    /// Index for centroid `IndexVec`s, enabling the associations for Centroids and `Vec<Point>`s
    pub struct CentroidIdx = usize;
}

/// A struct for keeping track of how far points are from a centroid
#[derive(Debug)]
struct CentroidDistance {
    /// Centroid Index. References a centroid in an `IndexVec<CentroidIdx, [&]Point>`
    idx: CentroidIdx,
    /// The distance to the point from the centroid (or vice-versa)
    distance: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// The result of a k-means run.
pub struct KMeansResult {
    /// The Sum Squared Error
    pub sse: f64,
    /// The amount of clusters used in that k-means run.0
    pub k: usize,
}

impl PartialOrd for KMeansResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.sse.partial_cmp(&other.sse)
    }
}

/// Generates a random point in the area of a 20x20 coordinate grid
fn random_point() -> Point {
    let mut rng = thread_rng();
    let (x, y) = (rng.gen_range(0_f32..20_f32), rng.gen_range(0_f32..20_f32));

    Point::new(x, y)
}

/// Creates a random dataset for the k-means algorithm.
///
/// `clusters` defaults to `DEFAULT_K`
///
/// `n_samp` defaults to `DEFAULT_N_SAMP`
///
/// `std` defaults to `DEFAULT_STD`
pub fn generate_datapoints(
    clusters: Option<usize>,
    n_samp: Option<usize>,
    std: Option<f32>,
) -> IndexVec<PointIdx, Point> {
    let clusters = match clusters {
        Some(v) => v,
        None => DEFAULT_K,
    };

    let n_samp = match n_samp {
        Some(v) => v,
        None => DEFAULT_N_SAMP,
    };

    let std = match std {
        Some(v) => v,
        None => DEFAULT_STD,
    };

    let mut centroids: IndexVec<PointIdx, Point> = index_vec![Point::default(); clusters];

    for el in &mut centroids {
        *el = random_point();
    }

    let mut points = index_vec!();

    for ctr in centroids.iter() {
        for _ in 0..n_samp / clusters {
            let mut point = random_point();

            while point.distance(*ctr) > std {
                point = random_point();
            }

            points.push(point);
        }
    }

    points.append(&mut centroids);

    points
}

/// Picks random centroids from a dataset
///
/// Panics if: the provided dataset is empty
pub fn pick_centroids(
    dataset: &IndexVec<PointIdx, Point>,
    k: Option<usize>,
) -> IndexVec<CentroidIdx, Point> {
    let k = match k {
        Some(v) => v,
        None => DEFAULT_K,
    };

    let mut thread_rng = thread_rng();

    let mut centroid_vec = index_vec![];

    for _ in 0..k {
        let random_centroid = dataset.raw.choose(&mut thread_rng).expect("non empty list");
        centroid_vec.push(random_centroid.clone());
    }

    centroid_vec
}

/// Associate the centroids with points by returning a `HashMap<CentroidIdx, Vec<&Point>>`
/// The `CentroidIdx` points to a centroid in the picked centroids (see pick_centroisd)
pub fn associate_centroids_to_points<'d, 'c>(
    dataset: &'d IndexVec<PointIdx, Point>,
    centroids: &'c IndexVec<CentroidIdx, Point>,
) -> HashMap<CentroidIdx, Vec<&'d Point>> {
    let mut assoc: HashMap<CentroidIdx, Vec<&'d Point>> = HashMap::new();

    for point in dataset {
        let mut distances: Vec<CentroidDistance> = vec![];

        for (idx, ctr) in centroids.iter_enumerated() {
            let cdist = CentroidDistance {
                idx,
                distance: point.distance(*ctr),
            };

            distances.push(cdist);
        }

        let smallest = distances
            .iter()
            .min_by(|x, y| x.distance.partial_cmp(&y.distance).unwrap())
            .expect("non-empty vec");

        assoc.entry(smallest.idx).or_insert(vec![]).push(point);
    }

    assoc
}

/// Updates centroids by calculating the mean of all the points associated with them.
pub fn update_centroids(assoc: &HashMap<CentroidIdx, Vec<&Point>>) -> IndexVec<CentroidIdx, Point> {
    let mut new_centroids: IndexVec<CentroidIdx, Point> = index_vec!();

    let centroid_indices = assoc.keys();

    for index in centroid_indices {
        let values = assoc.get(index).expect("valid indices returned by .keys()");
        new_centroids.push(calculate_average_point(values));
    }

    new_centroids
}

/// Calculates the mean point for a given vector of points
pub fn calculate_average_point(points: &Vec<&Point>) -> Point {
    let mut x: f32 = 0f32;
    let mut y: f32 = 0f32;

    let l = points.iter().len() as f32;

    for point in points {
        x += point.x;
        y += point.y;
    }

    x /= l;
    y /= l;

    Point::new(x, y)
}

/// Sorts a `Point` Vector with the following criteria:
/// The closer a point is to the coordinate origin (i.e., \[0, 0\]), the 'smaller' it is.
pub fn sort_point_vec(v: &IndexVec<CentroidIdx, Vec2>) -> IndexVec<CentroidIdx, Vec2> {
    let mut cloned_vec = v.clone();
    cloned_vec.sort_by(|a, b| a.x.total_cmp(&b.x).cmp(&a.y.total_cmp(&b.y)));
    cloned_vec
}

/// Compute the SSE
pub fn calc_sse(
    assoc: &HashMap<CentroidIdx, Vec<&Point>>,
    centroids: &IndexVec<CentroidIdx, Vec2>,
) -> KMeansResult {
    let mut sse = 0f64;

    for (ctr_idx, points) in assoc {
        let ctr = centroids.get(*ctr_idx).unwrap();
        let mut sub_sse = 0f32;

        for point in points {
            sub_sse += point.distance_squared(*ctr);
        }

        sse += sub_sse as f64;
    }

    KMeansResult {
        sse,
        k: assoc.keys().into_iter().len(),
    }
}
