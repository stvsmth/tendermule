use tendermule::generate_ids;

#[test]
fn test_count_generates_unique_values() {
    let adjs = vec!["blue", "gray", "red", "green"];
    let nouns = vec!["cat", "dog", "bird", "fish"];
    let count = 2;
    let max_length = 10;
    let ids = generate_ids(
        &adjs,
        &nouns,
        "".to_string(),
        "".to_string(),
        count,
        max_length,
    );
    let results = ids.into_iter().collect::<Vec<String>>();
    // Ensure we get exactly 2 words
    assert_eq!(results.len(), 2);
    let id_1 = results[0].clone();
    let id_2 = results[1].clone();

    // Ensure the words are not the same
    assert!(id_1 != id_2);

    // Ensure the generated ids contain an adj and noun
    assert!(adjs.iter().any(|&adj| id_1.to_lowercase().contains(adj)));
    assert!(nouns.iter().any(|&noun| id_1.to_lowercase().contains(noun)));
    assert!(adjs.iter().any(|&adj| id_2.to_lowercase().contains(adj)));
    assert!(nouns.iter().any(|&noun| id_2.to_lowercase().contains(noun)));
}
