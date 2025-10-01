use reqwest;
use std::{
    env,
    fs::File,
    io::{self, Write},
};
use xml::reader::{EventReader, XmlEvent};

pub fn download_file(url: &str, file_name: &str) -> Result<File, std::io::Error> {
    // We want to store the file in the temp directory
    let tmp_dir = env::temp_dir();
    let file_path = tmp_dir.join(file_name);
    println!("Downloading file to: {:?}\n", file_path);

    // Download the file and handle the error
    let response = reqwest::blocking::get(url)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Get file content and handle error
    let content = response
        .bytes()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut file = File::create(&file_path)?;
    file.write_all(&content)?;

    // We already open the file before reading it
    let downloaded_file = File::open(&file_path)?;

    Ok(downloaded_file)
}

// See https://crates.io/crates/xml
pub fn parse_xml(xml_file: File) {
    println!("Parsing xml file\n");
    let mut total_price = 0.0;
    let mut num_books = 0;

    let parser = EventReader::new(xml_file);

    // To keep track of which elements we are processing
    let mut is_price_element = false;
    let mut current_characters = String::new();

    // Go through every XmlEvent
    for e in parser {
        // Match event type
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                // Pretty straightforward
                if name.local_name == "book" {
                    num_books += 1;
                }

                // We keep track when we are in a price element to parse the characters between the tags
                if name.local_name == "price" {
                    is_price_element = true;
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                // When we detect characters and we are in a price element, set the current text
                if is_price_element {
                    current_characters = text;
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name == "price" {
                    // Parse the current text, set when we detected characters and we are in a price element.
                    if let Ok(price) = current_characters.parse::<f64>() {
                        total_price += price;
                    }
                    current_characters.clear();
                    is_price_element = false;
                }
            }

            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }

    println!("XML parsing finished!\nSummary of results:\n - number of books: {num_books}\n - total price of books: {total_price}");
}
