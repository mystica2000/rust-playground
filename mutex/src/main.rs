use std::{sync::{Arc, Mutex}, thread};
use std::fs::File;
use std::io::Write;
fn main() {

  // let mut children = vec![];
  // let count = Arc::new(Mutex::new(0));

  // for con in 1..10 {

  //   let count_clone = Arc::clone(&count);

  //   let handler = thread::spawn(move || {
  //     let mut data =  count_clone.lock().unwrap();
  //     *data = *data + con;
  //   });

  //   children.push(handler);
  // }

  // for child in children {
  //   let _ = child.join();
  // }


  // println!("Count {}", *count.lock().unwrap());

  let file = Arc::new(Mutex::new(File::create("output.txt").expect("Failed to create file")));

  let mut handles = vec![];

  for i in 1..10 {
    let file_clone = Arc::clone(&file);

    let handle = thread::spawn(move || {
      let mut file = file_clone.lock().unwrap();

      let content = format!("Line {}\n", i);

      file.write_all(content.as_bytes()).expect("Failed to write a file");
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  print!("File Writing!!");

}
