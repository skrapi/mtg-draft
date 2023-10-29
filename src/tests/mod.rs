mod tests {
    #[test]
    fn test_mtg_downloader() {
        use std::fs;
        use serde_json::Value;

        let card_json_text = fs::read_to_string("data/card.json").unwrap();
        let v: Value = serde_json::from_str(&card_json_text).unwrap();
        println!("{card_json_text}");
        println!("The card is {}", v["card"]["name"]);
    }
}
