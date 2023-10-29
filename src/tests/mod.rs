mod tests {
    #[test]
    fn test_mtg_downloader() {
        use std::fs;
        let card_json_text = fs::read_to_string("data/card.json").unwrap();
        println!("{card_json_text}");
    }
}
