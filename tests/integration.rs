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
        assert!(bucket1 < bucket2);
        assert!(bucket2 > bucket1);
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
        assert!(rank1 < rank2);
        assert!(rank2 > rank1);
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
        ];

        for (r1, r2) in rank_pairs {
            let rank1 = LexValue::new(r1).unwrap();
            let rank2 = LexValue::new(r2).unwrap();
            assert_ne!(rank1, rank2);
            assert!(rank1 < rank2);
            assert!(rank2 > rank1);
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
