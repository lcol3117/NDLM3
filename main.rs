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
  let options: Vec<f64> = data
  .iter()
  .map(|x| {l2d_not_self(x,point)})
  .collect::<Vec<f64>>();
  f64min(options)
}

fn crossing(a: Vec<f64>, b: Vec<f64>, data: Vec<Vec<f64>>, ndt: Vec<f64>, eta: f64) -> bool {
  let regionpts: Vec<Vec<f64>> = filter
}

fn l2d_not_self(a: Vec<f64>, b: Vec<f64>) -> f64 {
  let orig_l2dist: f64 = l2d(a,b)
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
  let dist_sum: f64 = z
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

fn median(l: Vec<f64>) -> f64 {
  let index_f64 = l.len() / 2;
  let index: usize = index_f64 as usize;
  l[index]
}