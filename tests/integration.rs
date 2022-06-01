mod constructor {
    use lexorank::{LexBucket, LexValue, LexoRank};

    #[test]
    fn create() {
        let lex_tuples = [(0, "2a"), (1, "01"), (2, "abc")];

        for (bucket, value) in lex_tuples {
            let lex_bucket = LexBucket::new(bucket).unwrap();
            let lex_value = LexValue::new(value).unwrap();
            let lexorank = LexoRank::new(lex_bucket, lex_value);

            assert_eq!(*lexorank.bucket(), LexBucket::new(bucket).unwrap());
            assert_eq!(*lexorank.rank(), LexValue::new(value).unwrap());
        }
    }

    #[test]
    #[should_panic(expected = "LexoRank bucket value must be between 0 and 2 inclusive. Found: 4")]
    fn create_with_invalid_bucket() {
        let buckets = [3, 4, 10, 100];

        for bucket in buckets {
            let bucket = LexBucket::new(bucket);
            assert!(bucket.is_err());
        }

        LexBucket::new(4).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Lexorank value must only include 0-9 and a-z and must not end with 0. Found: a0"
    )]
    fn create_with_invalid_rank() {
        let values = ["a90", "0", "12B", "C"];

        for value in values {
            let value = LexValue::new(value);
            assert!(value.is_err());
        }

        LexValue::new("a0").unwrap();
    }

    #[test]
    fn create_from_string() {
        let lex_tuples = [(0, "2a"), (1, "01"), (2, "abc")];

        for (bucket, value) in lex_tuples {
            let lex_string = format!("{}|{}", bucket, value);
            let lexorank = LexoRank::from_string(&lex_string).unwrap();

            assert_eq!(*lexorank.bucket(), LexBucket::new(bucket).unwrap());
            assert_eq!(*lexorank.rank(), LexValue::new(value).unwrap());
        }
    }

    #[test]
    #[should_panic(expected = "Cannot create LexoRank from invalid string. Found: 0|a0")]
    fn create_from_string_with_invalid_rank() {
        let values = ["a90", "0", "12B", "C"];

        for value in values {
            let lex_string = format!("0|{}", value);
            let value = LexoRank::from_string(&lex_string);
            assert!(value.is_err());
        }

        LexoRank::from_string("0|a0").unwrap();
    }

    #[test]
    #[should_panic(expected = "Cannot create LexoRank from invalid string. Found: 4|abc")]
    fn create_from_string_with_invalid_bucket() {
        let buckets = [3, 4, 10, 100];

        for bucket in buckets {
            let lex_string = format!("{}|abc", bucket);
            let value = LexoRank::from_string(&lex_string);
            assert!(value.is_err());
        }

        LexoRank::from_string("4|abc").unwrap();
    }
}

mod equality {
    use lexorank::{LexBucket, LexValue, LexoRank};

    #[test]
    fn compare_equal_buckets() {
        let bucket1 = LexBucket::new(0).unwrap();
        let bucket2 = LexBucket::new(0).unwrap();
        assert_eq!(bucket1, bucket2);
    }

    #[test]
    fn compare_unequal_buckets() {
        let bucket1 = LexBucket::new(0).unwrap();
        let bucket2 = LexBucket::new(1).unwrap();
        assert_ne!(bucket1, bucket2);
        assert!(
            bucket1 < bucket2,
            "{:?} was not less than {:?}",
            bucket1,
            bucket2
        );
        assert!(
            bucket2 > bucket1,
            "{:?} was not greater than {:?}",
            bucket2,
            bucket1
        );
    }

    #[test]
    fn compare_equal_ranks() {
        let rank1 = LexValue::new("01").unwrap();
        let rank2 = LexValue::new("01").unwrap();
        assert_eq!(rank1, rank2);
    }

    #[test]
    fn compare_unequal_ranks() {
        let rank1 = LexValue::new("01").unwrap();
        let rank2 = LexValue::new("02").unwrap();
        assert_ne!(rank1, rank2);
        assert!(rank1 < rank2, "{:?} was not less than {:?}", rank1, rank2);
        assert!(
            rank2 > rank1,
            "{:?} was not greater than {:?}",
            rank2,
            rank1
        );
    }

    #[test]
    fn compare_unequal_ranks_2() {
        let rank_pairs = [
            ("1", "9"),
            ("a", "z"),
            ("9", "a"),
            ("5", "f"),
            ("1322", "1323"),
            ("1a22", "1b21"),
            ("azdb", "xabd"),
            ("1zzz", "abz"),
            ("010001", "01001"),
        ];

        for (r1, r2) in rank_pairs {
            let rank1 = LexValue::new(r1).unwrap();
            let rank2 = LexValue::new(r2).unwrap();
            assert_ne!(rank1, rank2);
            assert!(rank1 < rank2, "{:?} was not less than {:?}", rank1, rank2);
            assert!(
                rank2 > rank1,
                "{:?} was not greater than {:?}",
                rank2,
                rank1
            );
        }
    }

    #[test]
    fn compare_equal_lexoranks() {
        let lexorank1: LexoRank = "0|01".try_into().unwrap();
        let lexorank2: LexoRank = "0|01".try_into().unwrap();
        assert_eq!(lexorank1, lexorank2);
    }

    #[test]
    fn compare_unequal_lexoranks() {
        let lexorank1: LexoRank = "0|01".try_into().unwrap();
        let lexorank2: LexoRank = "1|01".try_into().unwrap();
        assert_ne!(lexorank1, lexorank2);

        let lexorank1: LexoRank = "0|01".try_into().unwrap();
        let lexorank2: LexoRank = "0|02".try_into().unwrap();
        assert_ne!(lexorank1, lexorank2);
    }
}

mod increment {
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
}

mod between {
    use lexorank::{LexValue, LexoRank};

    #[test]
    fn between_ranks() {
        let test_cases = [
            ("1", "3", "2"),
            ("1", "9", "2"),
            ("a", "z", "b"),
            ("1", "2", "11"),
            ("a", "b", "a1"),
            ("12", "1a", "13"),
            ("101", "123", "102"),
            ("11", "12", "111"),
            ("az", "b", "az1"),
            ("1a1", "1a11", "1a101"),
            ("z4", "z41", "z401"),
            ("z4", "z401", "z4001"),
            ("z401", "z40100001", "z401000001"),
        ];

        for (rank1, rank2, between) in test_cases {
            println!("{} -> {} <- {}", rank1, between, rank2);
            let rank1 = LexValue::new(rank1).unwrap();
            let rank2 = LexValue::new(rank2).unwrap();
            let between = LexValue::new(between).unwrap();
            assert_eq!(rank1.between(&rank2).unwrap(), between);
            assert_eq!(rank2.between(&rank1).unwrap(), between);
        }
    }

    #[test]
    fn between_equal_ranks() {
        let test_cases = ["1", "z", "1a1", "z4", "z401", "z40100001"];

        for rank in test_cases {
            println!("{} -> {} <- {}", rank, rank, rank);
            let rank1 = LexValue::new(rank).unwrap();
            let rank2 = LexValue::new(rank).unwrap();
            assert_eq!(rank1.between(&rank2), None);
            assert_eq!(rank2.between(&rank1), None);
        }
    }

    #[test]
    fn between_lexoranks() {
        let test_cases = [
            ("0|1", "0|3", "0|2"),
            ("0|1", "0|9", "0|2"),
            ("0|a", "0|z", "0|b"),
            ("0|1", "0|2", "0|11"),
            ("0|a", "0|b", "0|a1"),
            ("0|12", "0|1a", "0|13"),
            ("0|101", "0|123", "0|102"),
            ("0|11", "0|12", "0|111"),
            ("0|az", "0|b", "0|az1"),
            ("0|1a1", "0|1a11", "0|1a101"),
            ("0|z4", "0|z41", "0|z401"),
            ("0|z4", "0|z401", "0|z4001"),
            ("0|z401", "0|z40100001", "0|z401000001"),
        ];

        for (lexorank1, lexorank2, between) in test_cases {
            println!("{} -> {} <- {}", lexorank1, between, lexorank2);
            let lexorank1: LexoRank = lexorank1.try_into().unwrap();
            let lexorank2: LexoRank = lexorank2.try_into().unwrap();
            let between: LexoRank = between.try_into().unwrap();
            assert_eq!(lexorank1.between(&lexorank2).unwrap(), between);
            assert_eq!(lexorank2.between(&lexorank1).unwrap(), between);
        }
    }

    #[test]
    fn between_equal_lexoranks() {
        let test_cases = [
            ("0|1", "0|1"),
            ("2|z", "2|z"),
            ("0|1a1", "0|1a1"),
            ("2|z4", "2|z4"),
            ("0|z401", "0|z401"),
            ("1|z40100001", "1|z40100001"),
        ];

        for (lexorank1, rank2) in test_cases {
            println!("{} -> {} <- {}", lexorank1, lexorank1, rank2);
            let rank1: LexoRank = lexorank1.try_into().unwrap();
            let rank2: LexoRank = rank2.try_into().unwrap();
            assert_eq!(rank1.between(&rank2), None);
            assert_eq!(rank2.between(&rank1), None);
        }
    }
}
