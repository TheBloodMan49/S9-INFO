fn main() {
    let mut s = String::from("Chaaaaaaaaaaaaa");

    modify_string(&mut s);
    println!("Après modification: {}", s);

    let mut counter = 0;

    let r1 = &mut counter;
    *r1 += 1;

    let r2 = &mut counter;
    *r2 += 1;

    println!("Compteur: {}", counter);

    let mut text = String::from("Rust");
    add_text(&mut text, " est cool");
    
    let ref1 = &text;
    let ref2 = &text;

    println!("{} et {}", ref1, ref2);
}

fn modify_string(s: &mut String) {
    s.push_str("aaaaaaaaaaaaaaaaaaaat");
}

fn add_text(s: &mut String, addition: &str) {
    s.push_str(addition);
}
