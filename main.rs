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

fn crossing(a: Vec<f64>, b: Vec<f64>, data: Vec<Vec<f64>>, ndt: Vec<f64>, eta: f64) -> bool {
  let a_index = data
    .iter()
    .position(|&x| x == )
}

fn alongLine(a: Vec<f64>, b: Vec<f64>, how_far: f64): Vec<f64> {
  let r = 0..a.length()
    .iter()
    .map(|x| {x as usize})
    .collect::<Vec<usize>>();
  let total_deltas = r
    .iter()
    .map(|x| {b[x] - a[x]})
    .collect::<Vec<f64>>();
  let new_deltas = total_deltas
    .iter()
    .map(|x| {x * how_far})
    .collect::<Vec<f64>>();
  r
    .iter()
    .map(|x| {a[x] + new_deltas[x]})
    .collect::<Vec<f64>>()
}

fn medianLeft(l: Vec<f64>): Vec<f64> {
  let sorted = l
    .iter()
    .sorted()
    .collect::<Vec<f64>>();
  let index = ((l.len() - 1) / 2).floor();
  let index_usize = index as usize;
  sorted[index_usize]
}

fn l2d_not_self(a: Vec<f64>, b: Vec<f64>) -> f64 {
  let orig_l2dist = l2d(a,b)
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
