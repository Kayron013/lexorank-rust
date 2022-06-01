use lexorank::{Bucket, LexoRank, Rank};

#[test]
fn compare_equal_buckets() {
    let bucket1 = Bucket::new(0).unwrap();
    let bucket2 = Bucket::new(0).unwrap();
    assert_eq!(bucket1, bucket2);
}

#[test]
fn compare_unequal_buckets() {
    let bucket1 = Bucket::new(0).unwrap();
    let bucket2 = Bucket::new(1).unwrap();
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
    let rank1 = Rank::new("01").unwrap();
    let rank2 = Rank::new("01").unwrap();
    assert_eq!(rank1, rank2);
}

#[test]
fn compare_unequal_ranks() {
    let rank1 = Rank::new("01").unwrap();
    let rank2 = Rank::new("02").unwrap();
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
        let rank1 = Rank::new(r1).unwrap();
        let rank2 = Rank::new(r2).unwrap();
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
