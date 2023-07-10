use tendermule::{generate_ids, Config};

#[test]
fn test_no_valid_words_generates_err() {
    let adjs = vec!["spasmodic"];
    let nouns = vec!["elephants"];

    let config = Config {
        prefix: String::from("stv"),
        suffix: String::from(""),
        count: 1,
        max_length: 8,
    };
    let ids = generate_ids(&adjs, &nouns, &config);
    assert!(ids.is_err());
}

#[test]
fn test_invalid_max_length_param() {
    let adjs = vec!["blue"];
    let nouns = vec!["cat"];

    let config = Config {
        prefix: String::from(""),
        suffix: String::from(""),
        count: 1,
        max_length: 257,
    };
    let ids = generate_ids(&adjs, &nouns, &config);
    assert!(ids.is_err());
}

#[test]
fn test_invalid_count_param() {
    let adjs = vec!["blue"];
    let nouns = vec!["cat"];

    let config = Config {
        prefix: String::from(""),
        suffix: String::from(""),
        count: 1_000_001,
        max_length: 8,
    };
    let ids = generate_ids(&adjs, &nouns, &config);
    assert!(ids.is_err());
}

#[test]
fn test_returns_minimal_set_of_ids() {
    let adjs = vec!["blue", "spasmodic"];
    let nouns = vec!["cat", "elephants"];

    let config = Config {
        prefix: String::from(""),
        suffix: String::from(""),
        count: 1,
        max_length: 8,
    };
    let ids = generate_ids(&adjs, &nouns, &config).unwrap();

    // Ensure we get exactly 1 unique id
    assert_eq!(ids.len(), 1);
    let id_1 = ids.iter().next().unwrap();
    assert_eq!(id_1, "BlueCat");
}

#[test]
fn test_count_generates_unique_values() {
    let adjs = vec!["blue", "gray"];
    let nouns = vec!["cat", "dog"];

    for _ in 0..24 {
        let config = Config {
            prefix: String::from(""),
            suffix: String::from(""),
            count: 2,
            max_length: 10,
        };
        let ids = generate_ids(&adjs, &nouns, &config).unwrap();

        // Ensure we get exactly 2 unique words
        assert_eq!(ids.len(), config.count);
        let id_1 = ids.iter().next().unwrap();
        let id_2 = ids.iter().nth(1).unwrap();
        assert_ne!(id_1, id_2);

        // Ensure the generated ids contain an adj and noun
        assert!(adjs.iter().any(|&adj| id_1.to_lowercase().contains(adj)));
        assert!(nouns.iter().any(|&noun| id_1.to_lowercase().contains(noun)));

        assert!(adjs.iter().any(|&adj| id_2.to_lowercase().contains(adj)));
        assert!(nouns.iter().any(|&noun| id_2.to_lowercase().contains(noun)));
    }
}
