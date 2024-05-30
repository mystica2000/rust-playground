use std::thread;

fn main() {
  let mut data = vec![1,2,3,4,5];

  let handles: Vec<_> = data.iter().map(|x| {
    let mut x = x.clone();
    thread::spawn(move || {
      x = x * 2;
    })
  }).collect();

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Data after thread is {:?}",data);


  thread::scope(|s| {
    for x in &mut data {
    s.spawn(|| {
      *x *= 2;
    });
    }
  });

  println!("Data after thread scope is {:?}",data);
}
