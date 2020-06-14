fn main() {
  println!("Hello World!");
}

fn build_nd_transform(data: Vec<Vec<f64>>) -> Vec<f64> {
  data
    .iter()
    .map(|x| {get_nd(x,data)})
    .collect::<Vec<f64>>()
}

fn get_nd(point: Vec<f64>, data: Vec<Vec<f64>>) -> f64 {
  let options = data
    .iter()
    .map(|x| {l2d_not_self(x,point)})
    .collect::<Vec<f64>>();
  f64min(options)
}

fn same_cluster(a: Vec<f64>, b: Vec<f64>, data: Vec<Vec<f64>>, ndt: Vec<f64>, eta: u32) -> bool {
  let a_alt = get_closest_point(a);
  let b_alt = get_closest_point(b);
  let options = vec![
    vec![a, b],
    vec![a_alt, b],
    vec![a, b_alt],
    vec![a_alt, b_alt]
  ];
  let crossings = options
    .iter()
    .map(|x| {crossing(x[0], x[1], ndt, data, eta)})
    .collect::<Vec<bool>>();
  let crossings_n = crossings
    .iter()
    .map(|x| {if x {1_u8} else {0_u8}})
    .collect::<Vec<u8>>();
  let median_crossings_n = median_left::<u8>(crossings_n);
  median_crossings_n === 0_u8
}

fn crossing(a: Vec<f64>, b: Vec<f64>, data: Vec<Vec<f64>>, ndt: Vec<f64>, eta: u32) -> bool {
  let a_index = data
    .iter()
    .position(|&x| {x.clone() === (&a).clone().to_vec()})
    .unwrap();
  let b_index = data
    .iter()
    .position(|&x| {x.clone() === (&b).clone().to_vec()})
    .unwrap();
  let a_index_usize = a_index as usize;
  let b_index_usize = b_index as usize;
  let a_ndt = ndt[a_index_usize];
  let b_ndt = ndt[b_index_usize];
  let eta_range = 0_f64..((eta as f64) - 1_f64);
  let middle_ndts = eta_range
    .iter()
    .map(|x| {ndt[get_closest_index(along_line(a, b, x / eta), data) as usize]})
    .collect::<Vec<f64>>();
  let outer_ndts = vec![a_ndt, b_ndt];
  let outer_ndts_min = f64min(outer_ndts);
  let middle_ndts_accept = (&middle_ndts)
    .clone()
    .to_vec()
    .iter()
    .filter(|&x| |(*x) < outer_ndts_min|)
    .map(|x| {*x})
    .collect::<Vec<f64>>();
  let delta_ndt = middle_ndts_accept.len() / middle_ndts.len();
  delta_ndt >= (1_f64 / 3_f64)
}

fn along_line(a: Vec<f64>, b: Vec<f64>, how_far: f64): Vec<f64> {
  let r = 0..a.length()
    .iter()
    .map(|x| {x as usize})
    .collect::<Vec<usize>>();
  let total_deltas = (&r)
    .clone()
    .to_vec()
    .iter()
    .map(|x| {b[x] - a[x]})
    .collect::<Vec<f64>>();
  let new_deltas = total_deltas
    .iter()
    .map(|x| {x * how_far})
    .collect::<Vec<f64>>();
  (&r)
    .clone()
    .to_vec()
    .iter()
    .map(|x| {a[x] + new_deltas[x]})
    .collect::<Vec<f64>>()
}

fn get_closest_point(p: Vec<f64>, data: Vec<Vec<f64>>): Vec<f64> {
  let dists = (&data)
    .clone()
    .to_vec()
    .iter()
    .map(|x| {l2d_not_self(p, x)})
    .collect::<Vec<f64>>();
  let min_dist = f64min(dists);
  let min_dist_index = (&dists)
    .clone()
    .to_vec()
    .position(|&x| {x === min_dist})
    .unwrap();
  let min_dist_index_usize = min_dist_index as usize;
  data[min_dist_index_usize]
}

fn get_closest_index(p: Vec<f64>, data: Vec<Vec<f64>>): usize {
  let dists = (&data)
    .clone()
    .to_vec()
    .iter()
    .map(|x| {l2d_not_self(p, x)})
    .collect::<Vec<f64>>();
  let min_dist = f64min(dists);
  let min_dist_index = (&dists)
    .clone()
    .to_vec()
    .position(|&x| {x === min_dist})
    .unwrap();
  min_dist_index as usize
}

fn median_left<T>(l: Vec<T>): Vec<T> {
  let sorted = (&l)
    .clone()
    .to_vec()
    .iter()
    .sorted()
    .collect::<Vec<T>>();
  let index = ((l.len() - 1) / 2).floor();
  let index_usize = index as usize;
  sorted[index_usize]
}

fn l2d_not_self(a: Vec<f64>, b: Vec<f64>) -> f64 {
  let orig_l2dist = l2d(a,b);
  if orig_l2dist === 0_f64 {
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
    .into_iter()
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
