extern crate reqwest;
extern crate select;
extern crate regex;
extern crate serde_json;
#[macro_use] extern crate lazy_static;

use lazy_static::lazy_static;
use select::document::Document;
use select::predicate::Class;
use select::node::Node;
use regex::Regex;
use std::fmt;

// load pub mods from other files
pub mod regstructs;

// Define Flat structure
pub struct Flat {
    location: String,
    squaremeters: String,
    price: String,
    link: String,
}

impl Flat {
    pub fn new(details: Vec<String>) -> Flat {
        Flat {
            location: details[0].clone(),
            squaremeters: details[1].clone(),
            price: details[2].clone(),
            link: details[3].clone(),
        }
    }
}

impl fmt::Display for Flat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "zip: {}, sm: {}, rent: {}, link: {}",
               self.location, self.squaremeters, self.price, self.link)
    }
}
// End of Flat structure

fn suitable_zip(location: &String) -> bool {
    let suitable_zips: Vec<String> = vec![
        String::from("Kreuzberg"),
        String::from("Neukölln"),
        String::from("Mitte"),
        String::from("Wedding"),
        String::from("Friedrichshain"),
    ];

    for zip in suitable_zips {
        if zip == *location {
            return true
        }
    }
    return false
}

fn suitable_sm(size: &String) -> bool {
    let size_as_int = size.parse::<i32>().unwrap();
    if size_as_int > 30 {
        return true
    } else { return false }
}


/// Main run function
pub fn run() -> Result<(), Box<std::error::Error>> {

    // Collect all flats
    let mut flat_iterator: Vec<Flat> = Vec::new();
    let res = reqwest::get("https://www.ebay-kleinanzeigen.de/s-berlin/anzeige:angebote/mietwohnung/k0l3331")?;

    Document::from_read(res)?

        .find(Class("aditem"))
        .for_each(|x| {

            let new_flat = regstructs::extract_details(x);
            flat_iterator.push(new_flat);
        });


    // Filter flats for suitable candidates
    let filtered_flats: Vec<Flat> = flat_iterator
        .into_iter()
        .filter(|flat| suitable_zip(&flat.location))
        .filter(|flat| suitable_sm(&flat.squaremeters))
        .collect();

    for flat in filtered_flats {
        println!("{}", flat);
    }

    Ok(())
}




