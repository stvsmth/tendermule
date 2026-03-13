use tendermule::*;

#[test]
fn test_no_valid_words_generates_err() {
    let adjs = vec!["spasmodic"];
    let nouns = vec!["elephants"];

    let config = Config {
        alliterate: false,
        count: 1,
        max_length: 8,
        prefix: String::from("stv"),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());
}

#[test]
fn test_returns_minimal_set_of_ids() {
    let adjs = vec!["blue", "spasmodic"];
    let nouns = vec!["cat", "elephants"];

    let config = Config {
        alliterate: false,
        count: 1,
        max_length: 8,
        prefix: String::new(),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config).unwrap();

    // Ensure we get exactly 1 unique id
    assert_eq!(result.len(), 1);
    let id_1 = result.iter().next().unwrap();
    assert_eq!(id_1, "BlueCat");
}

#[test]
fn test_count_generates_unique_values() {
    let adjs = vec!["blue", "gray", "red", "bold", "sly"];
    let nouns = vec!["cat", "dog", "ape", "flea", "eel"];

    for _ in 0..1000 {
        let config = Config {
            alliterate: false,
            count: 2,
            max_length: 10,
            prefix: String::new(),
            suffix: String::new(),
        };
        let result = generate_ids(&adjs, &nouns, &config);
        let ids = result.unwrap();

        // Ensure we get exactly 2 unique ids
        assert_eq!(ids.len(), 2);
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

#[test]
fn test_fixes_check() {
    let adjs = vec!["blue", "gray"];
    let nouns = vec!["cat", "dog"];
    let mut config = Config {
        alliterate: false,
        count: 1,
        max_length: 25,
        prefix: String::from("123456"),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());

    if let Err(e) = result {
        assert_eq!(format!("{e}"), "Prefix must be 5 characters or less.");
    }
    config.prefix = String::new();
    config.suffix = String::from("123456");
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());

    if let Err(e) = result {
        assert_eq!(format!("{e}"), "Suffix must be 5 characters or less.");
    }
}
#[test]
fn test_max_count_boundary() {
    let adjs = vec!["blue", "gray"];
    let nouns = vec!["cat", "dog"];
    let config = Config {
        alliterate: false,
        count: 1_000_001,
        max_length: 16,
        prefix: String::new(),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(
            format!("{e}"),
            format!("Count must be {} or less.", MAX_IDS_COUNT),
        );
    }
}

#[test]
fn test_max_length_check() {
    let adjs = vec!["blue", "gray"];
    let nouns = vec!["cat", "dog"];
    let mut config = Config {
        alliterate: false,
        count: 2,
        max_length: 255,
        prefix: String::new(),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config).unwrap();
    assert_eq!(result.len(), 2);

    config.max_length = 256;
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(
            format!("{e}"),
            format!("Max length must be {} or less.", MAX_ID_LENGTH),
        );
    }
}

#[test]
fn test_not_enough_unique_ids() {
    let adjs = vec!["blue", "gray"];
    let nouns = vec!["cat", "dog"];
    let config = Config {
        alliterate: false,
        count: 9,
        max_length: 16,
        prefix: String::new(),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(format!("{e}"), "Only 4 IDs available, cannot produce 9.");
    }
}

#[test]
fn test_fixes_overwhelm() {
    let adjs = vec!["sly"];
    let nouns = vec!["cat"];
    let mut config = Config {
        alliterate: false,
        count: 1,
        max_length: 14,
        prefix: String::from("pref"),
        suffix: String::from("suff"),
    };
    let result = generate_ids(&adjs, &nouns, &config).unwrap();
    assert_eq!(result.len(), 1);
    let id_1 = result.iter().next().unwrap();
    assert_eq!(id_1, "prefSlyCatsuff");

    config.max_length = 13;
    let result = generate_ids(&adjs, &nouns, &config);
    if let Err(e) = result {
        assert_eq!(
            format!("{e}"),
            "No unique IDs available for the given constraints."
        );
    }
}

#[test]
fn test_handle_never_finds_small_enough_word() {
    let adjs = vec!["gray"];
    let nouns = vec!["cat", "dog", "elm", "eel"];
    let config = Config {
        alliterate: false,
        count: 1,
        max_length: 6,
        prefix: String::new(),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(
            format!("{e}"),
            "No unique IDs available for the given constraints."
        );
    }
}

#[test]
fn test_did_not_generate_desired_count() {
    let adjs = vec!["sly", "fun", "blue", "gray"];
    let nouns = vec!["cat", "dog", "elephant", "mouse"];
    let config = Config {
        alliterate: false,
        count: 10,
        max_length: 16,
        prefix: String::from("12345"),
        suffix: String::from("54321"),
    };
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());
}

#[test]
fn test_generates_alliteration() {
    let adjs = vec!["sly", "fun", "blue", "gray"];
    let nouns = vec!["shark", "dog", "elephant", "mouse"];
    let config = Config {
        alliterate: true,
        count: 1,
        max_length: 32,
        prefix: String::new(),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_ok());
    let ids = result.unwrap();
    assert_eq!(ids.len(), 1);
    let id_1 = ids.iter().next().unwrap();
    assert_eq!(id_1, "SlyShark");
}

#[test]
fn test_generates_no_alliteration() {
    let adjs = vec!["sly", "fun", "blue", "gray"];
    let nouns = vec!["mutt", "dog", "elephant", "mouse"];
    let config = Config {
        alliterate: true,
        count: 1,
        max_length: 32,
        prefix: String::new(),
        suffix: String::new(),
    };
    let result = generate_ids(&adjs, &nouns, &config);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(
            format!("{e}"),
            "No unique IDs available for the given constraints."
        );
    }
}
