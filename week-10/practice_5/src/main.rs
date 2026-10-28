fn main() {
    let x = vec![100,200,300];
    borrow_vector(&x);
}

fn borrow_vector(z:&Vec<i32>){
    println!("*****************************");
    println!("Inside print_vector function {:?}",z);
    println!("-----------------------------");
}