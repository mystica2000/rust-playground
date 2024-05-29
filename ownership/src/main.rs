
fn some_function(a: &mut i32) {
  *a = 10;
}

fn main() {

    let mut a:i32 = 1;
    let mut b:i32 = a;
    b = 10;
    some_function(&mut a);
    println!("{} {}",b, a);

    // heap, String
    let var_a = String::from("test");
    let var_b = &var_a;

    // value borrowed here: use clone or just pass the reference (read-only)
    println!("{}",var_a);
}
