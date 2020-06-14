extern crate itertools;

use itertools::sorted;

fn main() {
  println!("Hello World!");
}

fn build_nd_transform(data: Vec<Vec<f64>>) -> Vec<f64> {
  data
    .iter()
    .map(|x| {get_nd(x.to_vec(),(&data).to_vec())})
    .collect::<Vec<f64>>()
}

fn get_nd(point: Vec<f64>, data: Vec<Vec<f64>>) -> f64 {
  let options = data
    .iter()
    .map(|x| {l2d_not_self(x.to_vec(),(&point).to_vec())})
    .collect::<Vec<f64>>();
  f64min(options)
}

fn same_cluster(a: Vec<f64>, b: Vec<f64>, data: Vec<Vec<f64>>, ndt: Vec<f64>, eta: u32) -> bool {
  let a_alt = get_closest_point((&a).to_vec(), (&data).to_vec());
  let b_alt = get_closest_point((&b).to_vec(), (&data).to_vec());
  let options = vec![
    vec![(&a).to_vec(), (&b).to_vec()],
    vec![(&a_alt).to_vec(), b],
    vec![a, (&b_alt).to_vec()],
    vec![a_alt, b_alt]
  ];
  let crossings = options
    .iter()
    .map(|x| {crossing(x.to_vec()[0], x.to_vec()[1], (&data).to_vec(), (&ndt).to_vec(), eta)})
    .collect::<Vec<bool>>();
  let crossings_n = crossings
    .iter()
    .map(|x| {if *x {1_u8} else {0_u8}})
    .collect::<Vec<u8>>();
  let median_crossings_n = median_left_u8(crossings_n);
  median_crossings_n == 0_u8
}

fn crossing(a: Vec<f64>, b: Vec<f64>, data: Vec<Vec<f64>>, ndt: Vec<f64>, eta: u32) -> bool {
  let a_index = data
    .iter()
    .position(|x| {x.clone() == (&a).to_vec()})
    .unwrap();
  let b_index = data
    .iter()
    .position(|x| {x.clone() == (&b).to_vec()})
    .unwrap();
  let a_index_usize = a_index as usize;
  let b_index_usize = b_index as usize;
  let a_ndt = ndt[a_index_usize];
  let b_ndt = ndt[b_index_usize];
  let eta_range = (0_u32..(eta - 1_u32))
    .collect::<Vec<u32>>()
    .iter()
    .map(|x| {(*x) as f64})
    .collect::<Vec<f64>>();
  let middle_ndts = eta_range
  .iter()
    .map(|x| {(&ndt).to_vec()[get_closest_index(along_line((&a).to_vec(), (&b).to_vec(), x / (eta as f64)), (&data).to_vec()) as usize]})
    .collect::<Vec<f64>>();
  let outer_ndts = vec![a_ndt, b_ndt];
  let outer_ndts_min = f64min(outer_ndts);
  let middle_ndts_accept = (&middle_ndts)
    .clone()
    .to_vec()
    .iter()
    .filter(|&x| {(*x) < outer_ndts_min})
    .map(|x| {*x})
    .collect::<Vec<f64>>();
  let delta_ndt = (middle_ndts_accept.len() as f64) / (middle_ndts.len() as f64);
  delta_ndt >= (1_f64 / 3_f64)
}

fn along_line(a: Vec<f64>, b: Vec<f64>, how_far: f64) -> Vec<f64> {
  let r = (0..a.len()).map(|x| {x as usize}).collect::<Vec<usize>>();
  let total_deltas = (&r)
    .clone()
    .to_vec()
    .iter()
    .map(|x| {b[*x] - a[*x]})
    .collect::<Vec<f64>>();
  let new_deltas = total_deltas
    .iter()
    .map(|x| {x * how_far})
    .collect::<Vec<f64>>();
  (&r)
    .clone()
    .to_vec()
    .iter()
    .map(|x| {a[*x] + new_deltas[*x]})
    .collect::<Vec<f64>>()
}

fn get_closest_point(p: Vec<f64>, data: Vec<Vec<f64>>) -> Vec<f64> {
  let dists = (&data)
    .clone()
    .to_vec()
    .iter()
    .map(|x| {l2d_not_self((&p).to_vec(), x.to_vec())})
    .collect::<Vec<f64>>();
  let min_dist = f64min((&dists).to_vec());
  let min_dist_index = (&dists)
    .clone()
    .to_vec()
    .iter()
    .position(|&x| {x == min_dist})
    .unwrap();
  let min_dist_index_usize = min_dist_index as usize;
  (&data).to_vec()[min_dist_index_usize]
}

fn get_closest_index(p: Vec<f64>, data: Vec<Vec<f64>>) -> usize {
  let dists = (&data)
    .clone()
    .to_vec()
    .iter()
    .map(|x| {l2d_not_self((&p).to_vec(), x.to_vec())})
    .collect::<Vec<f64>>();
  let min_dist = f64min((&dists).to_vec());
  let min_dist_index = (&dists)
    .clone()
    .to_vec()
    .iter()
    .position(|&x| {x == min_dist})
    .unwrap();
  min_dist_index as usize
}

fn median_left_u8(l: Vec<u8>) -> u8 {
  let sorted = sorted((&l).to_vec())
    .collect::<Vec<u8>>();
  let index = (((l.len() as f64) - 1_f64) / 2_f64).floor();
  let index_usize = index as usize;
  sorted[index_usize]
}

fn l2d_not_self(a: Vec<f64>, b: Vec<f64>) -> f64 {
  let orig_l2dist = l2d(a,b);
  if orig_l2dist == 0_f64 {
    std::f64::INFINITY
  } else {
    orig_l2dist
  }
}

fn l2d(a: Vec<f64>, b: Vec<f64>) -> f64 {
  let z = a
    .iter()
    .zip(b.iter());
  let dist_sum = z
    .collect::<Vec<(&f64,&f64)>>()
    .iter()
    .fold(0_f64, |a, x| {a + (((*x).0 - (*x).1).powf(2_f64))});
  dist_sum.sqrt()
}

fn f64max(l: Vec<f64>) -> f64 {
  l
    .iter()
    .cloned()
    .fold(0./0., f64::max)
}

fn f64min(l: Vec<f64>) -> f64 {
  l
    .iter()
    .cloned()
    .fold(0./0., f64::min)
}
