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
    assert_eq!(results.len(), 2);
    let id_1 = results[0].clone();
    let id_2 = results[1].clone();
    assert!(id_1 != id_2);
}
