use regex::Regex;
use lazy_static::lazy_static;
use crate::Flat;
use select::node::Node;
use select::predicate::Class;


/// Define lazy regex
lazy_static! {
    static ref RE_SM_CAN: Regex = Regex::new(r"[0-9]{2,3}\sm²").unwrap();
    static ref RE_SM: Regex = Regex::new(r"[0-9]{2,3}").unwrap();

    static ref RE_ZIP_CAN: Regex = Regex::new(r"\d{5}\n\s+[a-zA-Zöäü]+").unwrap();
    static ref RE_ZIP: Regex = Regex::new(r"[a-zA-Zöäü]+").unwrap();

    static ref RE_RENT_CAN: Regex = Regex::new(r"[0-9\\.]{3,5}\s€").unwrap();
    static ref RE_RENT: Regex = Regex::new(r"[0-9\\.]{3,5}").unwrap();
}

// extract details from text
pub fn extract_details(x: Node) -> Flat {

    let text = &x.text();

    let sm = handle_outer_capture(text, &*RE_SM_CAN, &*RE_SM);
    let zip = handle_outer_capture(text, &*RE_ZIP_CAN, &*RE_ZIP);
    let rent = handle_outer_capture(text, &*RE_RENT_CAN, &*RE_RENT);

    let link = extract_link(x);

    let new_flat = Flat::new(vec![zip.to_string(),
                                  sm.to_string(),
                                  rent.to_string(),
                                  link.to_string(),
    ]);
    new_flat
}

// extract link
fn extract_link(x: Node) -> &str {

    let mut links = Vec::new();

    x.find(Class("ellipsis"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| links.push(x));

    links[0]
}

// handle captures
fn handle_outer_capture<'t>(text: &'t str, re_out: &Regex, re_in: &Regex) -> &'t str {
    let cap = match re_out.captures(text) {
        None => "empty",
        Some(i) => handle_inner_capture(i.get(0).unwrap().as_str(), re_in)
    };
    cap
}

fn handle_inner_capture<'t>(text: &'t str, re: &Regex) -> &'t str {
    let cap = match re.captures(text) {
        None => "empty",
        Some(i) => i.get(0).unwrap().as_str()
    };
    cap
}