use std::fmt::Display;

use time::Name;
mod point;
mod time;

#[derive(Debug, Clone)]
struct Address {
    street: String,
    number: i32,
    zip_code: String,
    city: String,
}
impl Address {
    fn new(street: String, number: i32, city: String) -> Self {
        Self {
            street,
            number,
            zip_code: String::from("0"),
            city,
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "street: {}, number: {}, zip_code: {}, city: {}",
            self.street, self.number, self.zip_code, self.city,
        )
    }
}

fn main() {
    let s = String::from("Harrald Schmitz");
    let name = get_name(s.as_str());
}

fn get_name(s: &str) -> Name {
    Name::new(s)
}

fn point() {
    let mut point = point::Point::new(23, 15);
    let mut point2 = point::Point::new(23, 15);
    println!("{:?}", point);

    *point.x_mut() += 10;

    println!("{:?}", point);

    let point3 = point + point2;
    println!("{:?}", point3);

    let mut point = point::Point::new("true".to_string(), "false".to_string());
    let mut point2 = point::Point::new(true, false);
}

fn address() {
    let address = Address {
        street: String::from("Goldgasse"),
        number: 53,
        zip_code: String::from("51934"),
        city: String::from("Hanover"),
    };

    let address2 = Address::new("Annastr".to_string(), 198, "Hamburg".to_string());

    println!("{}", address);
    println!("{:?}", address);
    println!("{}", address2);
    println!("{:?}", address2);

    let address3 = address2.clone();
    println!("{}", address2);
    println!("{}", address3);
}
