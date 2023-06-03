use tendermule::generate_ids;

#[test]
fn test_count_generates_unique_values() {
    let adjs = vec!["blue", "gray"];
    let nouns = vec!["cat", "dog"];
    let count = 2;
    let max_length = 10;
    let ids = generate_ids(
        &adjs,
        &nouns,
        "".to_string(),
        "".to_string(),
        count,
        max_length,
    )
    .unwrap();

    // Ensure we get exactly 2 unique words
    assert_eq!(ids.len(), count);
    let id_1 = ids.iter().next().unwrap();
    let id_2 = ids.iter().nth(1).unwrap();
    assert_ne!(id_1, id_2);

    // Ensure the generated ids contain an adj and noun
    assert!(adjs.iter().any(|&adj| id_1.to_lowercase().contains(adj)));
    assert!(nouns.iter().any(|&noun| id_1.to_lowercase().contains(noun)));

    assert!(adjs.iter().any(|&adj| id_2.to_lowercase().contains(adj)));
    assert!(nouns.iter().any(|&noun| id_2.to_lowercase().contains(noun)));
}
