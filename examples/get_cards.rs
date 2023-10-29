// # Get all cards
// curl "https://api.magicthegathering.io/v1/cards"

// # Filter cards
// curl "https://api.magicthegathering.io/v1/cards?supertypes=legendary&types=creature&colors=red,white"

// # Get specific page of data
// curl "https://api.magicthegathering.io/v1/cards?page=50&pageSize=50"

// Get card by ID
//"https://api.magicthegathering.io/v1/cards/386616"
fn main() {
    let body = reqwest::blocking::get("https://api.magicthegathering.io/v1/cards/386616").unwrap()
    .text().unwrap();

    println!("body = {:#?}", body);
}
