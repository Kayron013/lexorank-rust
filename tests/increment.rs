use lexorank::{LexBucket, LexValue, LexoRank};

#[test]
fn increment_bucket() {
    let bucket_pairs = [(0, 1), (1, 2), (2, 0)];

    for (before, after) in bucket_pairs {
        println!("{} -> {}", before, after);
        let before_bucket = LexBucket::new(before).unwrap();
        let after_bucket = LexBucket::new(after).unwrap();
        assert_eq!(before_bucket.next(), after_bucket);
    }
}

#[test]
fn decrement_bucket() {
    let bucket_pairs = [(0, 2), (1, 0), (2, 1)];

    for (before, after) in bucket_pairs {
        println!("{} -> {}", before, after);
        let before_bucket = LexBucket::new(before).unwrap();
        let after_bucket = LexBucket::new(after).unwrap();
        assert_eq!(before_bucket.prev(), after_bucket);
    }
}

#[test]
fn increment_rank() {
    let test_cases = [
        ("1", "2"),
        ("8", "9"),
        ("9", "a"),
        ("a", "b"),
        ("y", "z"),
        ("z", "z1"),
        ("11", "12"),
        ("2b", "2c"),
        ("109", "10a"),
        ("abz", "ac"),
        ("yzz", "z"),
        ("y2wzz", "y2x"),
        ("zzz", "zzz1"),
    ];

    for (before, after) in test_cases {
        println!("{} -> {}", before, after);
        let before_rank = LexValue::new(before).unwrap();
        let after_rank = LexValue::new(after).unwrap();
        assert_eq!(before_rank.next(), after_rank);
    }
}

#[test]
fn decrement_rank() {
    let test_cases = [
        ("1", "01"),
        ("8", "7"),
        ("9", "8"),
        ("a", "9"),
        ("b", "a"),
        ("z", "y"),
        ("11", "1"),
        ("2c", "2b"),
        ("10a", "109"),
        ("abz", "aby"),
        ("z1", "z"),
        ("01", "001"),
        ("01001", "01"),
    ];

    for (before, after) in test_cases {
        println!("{} -> {}", before, after);
        let before_rank = LexValue::new(before).unwrap();
        let after_rank = LexValue::new(after).unwrap();
        assert_eq!(before_rank.prev(), after_rank);
    }
}

#[test]
fn increment_lexorank() {
    let test_cases = [
        ("1|01", "1|02"),
        ("0|9", "0|a"),
        ("0|a", "0|b"),
        ("1|y", "1|z"),
        ("0|z", "0|z1"),
        ("2|11", "2|12"),
        ("0|2b", "0|2c"),
        ("0|109", "0|10a"),
        ("2|abz", "2|ac"),
        ("0|yzz", "0|z"),
        ("1|y2wzz", "1|y2x"),
        ("0|zzz", "0|zzz1"),
    ];

    for (before, after) in test_cases {
        println!("{} -> {}", before, after);
        let before_lexorank: LexoRank = before.try_into().unwrap();
        let after_lexorank: LexoRank = after.try_into().unwrap();
        assert_eq!(before_lexorank.next(), after_lexorank);
    }
}

#[test]
fn decrement_lexorank() {
    let test_cases = [
        ("1|1", "1|01"),
        ("0|8", "0|7"),
        ("2|9", "2|8"),
        ("0|a", "0|9"),
        ("0|b", "0|a"),
        ("2|z", "2|y"),
        ("1|11", "1|1"),
        ("0|2c", "0|2b"),
        ("0|10a", "0|109"),
        ("1|abz", "1|aby"),
        ("0|z1", "0|z"),
        ("0|01", "0|001"),
        ("2|01001", "2|01"),
    ];

    for (before, after) in test_cases {
        println!("{} -> {}", before, after);
        let before_lexorank: LexoRank = before.try_into().unwrap();
        let after_lexorank: LexoRank = after.try_into().unwrap();
        assert_eq!(before_lexorank.prev(), after_lexorank);
    }
}
