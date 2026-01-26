fn main() {
    let s1 = String::from("\u{01F431}");
    let s2 = s1.clone();

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    let v1 = vec![1, 2, 3];
    print_vec(v1.clone());
    println!("Longueur du vecteur : {}", v1.len());
}

fn print_vec(v: Vec<i32>) {
    println!("Vecteur : {:?}", v);
}
