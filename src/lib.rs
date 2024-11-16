use std::collections::HashMap;

use glam::Vec2;
use index_vec::{index_vec, IndexVec};
use rand::{seq::SliceRandom, thread_rng, Rng};

const DEFAULT_K: usize = 4;
const DEFAULT_N_SAMP: usize = 100;
const DEFAULT_STD: f32 = 0.8;

pub type Point = Vec2;

index_vec::define_index_type! {
    pub struct PointIdx = usize;
}

index_vec::define_index_type! {
    pub struct CentroidIdx = usize;
}

struct CentroidDistance {
    idx: CentroidIdx,
    distance: f32,
}

fn random_point() -> Point {
    let mut rng = thread_rng();
    let (x, y) = (rng.gen_range(0_f32..20_f32), rng.gen_range(0_f32..20_f32));

    Point::new(x, y)
}

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
        None => DEFAULT_STD * 2.,
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

pub fn update_centroids(assoc: &HashMap<CentroidIdx, Vec<&Point>>) -> IndexVec<CentroidIdx, Point> {
    let mut new_centroids: IndexVec<CentroidIdx, Point> = index_vec!();

    let centroid_indices = assoc.keys();

    for index in centroid_indices {
        let values = assoc.get(index).expect("valid indices returned by .keys()");
        new_centroids.push(calculate_average_point(values));
    }

    new_centroids
}

fn calculate_average_point(points: &Vec<&Point>) -> Point {
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

pub fn sort_point_vec(v: &mut IndexVec<CentroidIdx, Vec2>) {
    v.sort_by(|a, b| a.x.total_cmp(&b.x).cmp(&a.y.total_cmp(&b.y)))
}
