fn main() {
    let s = String::from("Chaaaaaaaaaaaaaaaaaaat");

    let length = compute_length(&s);
    println!("La longueur de '{}' est {}", s, length);

    let v = vec![1, 2, 3, 4, 5];
    let first = get_first(&v);
    let last = get_last(&v);

    println!("Premier: {}, Dernier: {}", first, last);
}

fn compute_length(s: &String) -> usize {
    s.len()
}

fn get_first(v: &Vec<i32>) -> i32 {
    v[0]
}

fn get_last(v: &Vec<i32>) -> i32 {
    v[v.len() - 1]
}
