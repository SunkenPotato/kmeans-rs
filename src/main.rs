use std::collections::HashMap;

use glam::Vec2;
use ordered_float::OrderedFloat;
use rand::{seq::SliceRandom, thread_rng, Rng};

type Point = Vec2;
type Centroids<'a> = Vec<&'a Point>;
type PCAssoc = HashMap<Point, Vec<Point>>;

struct Point {
    vec: Vec2,
    id: usize,
}

struct CentroidDistance<'p> {
    centroid: &'p Point,
    distance: f32,
}

impl<'p> PartialEq for CentroidDistance<'p> {
    fn eq(&self, other: &Self) -> bool {
        self.centroid == other.centroid && self.distance == other.distance
    }
}

impl<'p> PartialOrd for CentroidDistance<'p> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.total_cmp(&other.distance))
    }
}

impl<'p> CentroidDistance<'p> {
    fn new(centroid: &'p Point, distance: f32) -> Self {
        Self { centroid, distance }
    }
}

const GRID_SIZE: (usize, usize) = (20, 20);

fn init_centroids<'a, 'v>(d: &'v Vec<Point>, k: usize) -> Centroids<'a>
where
    'v: 'a,
{
    let mut centroids: Centroids = vec![];
    for _ in 0..k {
        // ok lol
        let Some(centroid) = d.choose(&mut thread_rng()) else {
            panic!("The dataset is empty!")
        };
        centroids.push(centroid);
    }

    centroids
}

fn generate_datapoints(
    n_points: Option<usize>,
    k_clusters: Option<usize>,
    distribution: Option<f32>,
) -> Vec<Point> {
    let n_points = match n_points {
        Some(v) => v,
        None => 100,
    };

    let k_clusters = match k_clusters {
        Some(v) => v,
        None => 4,
    };

    let distribution = match distribution {
        Some(v) => v,
        None => 4.,
    };

    let mut centroids: Vec<Vec2> = vec![Vec2::default(); k_clusters];
    centroids.fill_with(|| random_point());

    let mut dataset: Vec<Point> = vec![];

    for _centroid in centroids {
        for _ in 0..(n_points / k_clusters) {
            dataset.push(generate_point(_centroid, distribution))
        }
    }

    dataset
}

fn generate_point(center: Point, rad: f32) -> Point {
    let mut point: Point = Point::default();

    while point.distance(center) > rad {
        point = random_point();
    }

    point
}

fn random_point() -> Point {
    let new_x: f32 = rand::thread_rng().gen_range(0_f32..GRID_SIZE.0 as f32);
    let new_y: f32 = rand::thread_rng().gen_range(0_f32..GRID_SIZE.1 as f32);

    Point::from_array([new_x, new_y])
}

fn random_centroids<'a, 'v>(dataset: &'v Vec<Point>, k: usize) -> Centroids<'a>
where
    'v: 'a,
{
    let mut centroids: Centroids = vec![];
    for _ in 0..k {
        let Some(random_point) = dataset.choose(&mut thread_rng()) else {
            panic!("Dataset is empty!");
        };
        centroids.push(random_point);
    }

    centroids
}

fn assign_points_to_centroids(d: Vec<Vec2>, centroids: Centroids) -> PCAssoc {
    let mut assocs: PCAssoc = HashMap::new();

    for point in d {
        let mut centroid_distances: Vec<CentroidDistance> = vec![];

        for ctr in &centroids {
            centroid_distances.push(CentroidDistance::new(*ctr, ctr.distance(point)))
        }

        assocs.insert(centroid_distances.iter().reduce(f32::MIN).unwrap())
    }

    assocs
}

fn main() {
    let k = 4; // Change!

    let dataset = generate_datapoints(None, None, None); // Initialize dataset
    let guessed_centroids = random_centroids(&dataset, k);

    let mut old_centroids = guessed_centroids;

    loop {
        let associations = todo!();
    }
}
