use std::fs;
use std::io::Error;

fn main() {
    let text = fs::read_to_string("logs.txt").unwrap();
    println!("{:#?}", text);

    match divide(5.0, 3.0) {
        Ok(result_of_devision) => println!("{}", result_of_devision),
        Err(what_went_wrong) => println!("{}", what_went_wrong),
    }

    match validate_email(String::from("lekovskigmail.com")) {
        Ok(..) => println!("Email valid"),
        Err(what_is_wrong) => println!("{}", what_is_wrong),
    }

    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        String::from("Olives")
    ];
    match validate_ingredients(&ingredients) {
        Ok(..) => println!("Ok"),
        Err(what_happened) => println!("{}", what_happened),
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") { Ok(()) } else { Err(Error::other("emails must have an @")) }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 { Err(Error::other("can't divide by 0")) } else { Ok(a / b) }
}

fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        Err(Error::other("You can't use more than 3 ingredients"))
    } else {
        Ok(())
    }
}
