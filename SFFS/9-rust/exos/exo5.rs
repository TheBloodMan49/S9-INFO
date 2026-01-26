fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let index = find_position(&numbers, 3);
    println!("Position de 3: {}", index);

    let value = get_element(&numbers, 10);
    match value {
        Some(v) => println!("Élément à l'index 10: {}", v),
        None => println!("10 n'est pas un index valide"),
    }

    let text = "42";
    match parse_number(text) {
        Ok(number) => println!("Nombre parsé: {}", number),
        Err(e) => println!("Erreur lors du parsing: {}", e),
    };

    let result = divide(10.0, 0.0);
    match result {
        Ok(quotient) => println!("Résultat de la division: {}", quotient),
        Err(e) => println!("Erreur lors de la division: {}", e),
    }
}

fn find_position(vec: &Vec<i32>, value: i32) -> usize {
    for (i, &v) in vec.iter().enumerate() {
        if v == value {
            return i;
        }
    }
    0
}

fn get_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    // On peut vérifier si l'index est dans les limites avec vec.len()
    if index < vec.len() {
        Some(vec[index])
    } else {
        None
    }
}

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse() // Renvoie déjà un Result
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division par zéro"))
    } else {
        Ok(a / b)
    }
}