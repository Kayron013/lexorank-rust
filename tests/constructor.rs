use lexorank::{Bucket, LexoRank, Rank};

#[test]
fn create() {
    let lex_tuples = [(0, "2a", "0|2a"), (1, "01", "1|01"), (2, "abc", "2|abc")];

    for (bucket, value, lex_string) in lex_tuples {
        let lex_bucket = Bucket::new(bucket).unwrap();
        let lex_value = Rank::new(value).unwrap();
        let lexorank = LexoRank::new(lex_bucket, lex_value);

        assert_eq!(*lexorank.bucket(), Bucket::new(bucket).unwrap());
        assert_eq!(*lexorank.rank(), Rank::new(value).unwrap());
        assert_eq!(lexorank.to_string(), lex_string);
    }
}

#[test]
#[should_panic(expected = "LexoRank bucket value must be between 0 and 2 inclusive. Found: 4")]
fn create_with_invalid_bucket() {
    let buckets = [3, 4, 10, 100];

    for bucket in buckets {
        let bucket = Bucket::new(bucket);
        assert!(bucket.is_err());
    }

    Bucket::new(4).unwrap();
}

#[test]
#[should_panic(
    expected = "Lexorank value must only include 0-9 and a-z and must not end with 0. Found: a0"
)]
fn create_with_invalid_rank() {
    let values = ["a90", "0", "12B", "C"];

    for value in values {
        let value = Rank::new(value);
        assert!(value.is_err());
    }

    Rank::new("a0").unwrap();
}

#[test]
fn create_from_string() {
    let lex_tuples = [(0, "2a"), (1, "01"), (2, "abc")];

    for (bucket, value) in lex_tuples {
        let lex_string = format!("{}|{}", bucket, value);
        let lexorank = LexoRank::from_string(&lex_string).unwrap();

        assert_eq!(*lexorank.bucket(), Bucket::new(bucket).unwrap());
        assert_eq!(*lexorank.rank(), Rank::new(value).unwrap());
        assert_eq!(lexorank.to_string(), lex_string);
    }
}

#[test]
#[should_panic(expected = "LexoRank bucket value must be between 0 and 2 inclusive. Found: 4")]
fn create_from_string_with_invalid_bucket() {
    let buckets = [3, 4, 10, 100];

    for bucket in buckets {
        let lex_string = format!("{}|abc", bucket);
        let value = LexoRank::from_string(&lex_string);
        assert!(value.is_err());
    }

    LexoRank::from_string("4|abc").unwrap();
}

#[test]
#[should_panic(
    expected = "Lexorank value must only include 0-9 and a-z and must not end with 0. Found: a0"
)]
fn create_from_string_with_invalid_rank() {
    let values = ["a90", "0", "12B", "C"];

    for value in values {
        let lex_string = format!("0|{}", value);
        let value = LexoRank::from_string(&lex_string);
        assert!(value.is_err());
    }

    LexoRank::from_string("0|a0").unwrap();
}
