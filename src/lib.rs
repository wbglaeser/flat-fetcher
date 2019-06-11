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

// Define iterator over flats

pub fn run() -> Result<(), Box<std::error::Error>> {

    let mut FlatIterator: Vec<Flat> = Vec::new();

    let res = reqwest::get("https://www.ebay-kleinanzeigen.de/s-berlin/mietwohnung/k0l3331")?;

    Document::from_read(res)?

        .find(Class("aditem"))
        //.filter_map(|n| n.attr("href"))
        .for_each(|x| {

            let new_flat = regstructs::extract_details(x);
            FlatIterator.push(new_flat);

        });

    for flat in FlatIterator {
        println!("{}", flat);
    }

    Ok(())
}




