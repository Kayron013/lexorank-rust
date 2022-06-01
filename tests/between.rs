use lexorank::{LexoRank, Rank};

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
        let rank1 = Rank::new(rank1).unwrap();
        let rank2 = Rank::new(rank2).unwrap();
        let between = Rank::new(between).unwrap();
        assert_eq!(rank1.between(&rank2).unwrap(), between);
        assert_eq!(rank2.between(&rank1).unwrap(), between);
    }
}

#[test]
fn between_equal_ranks() {
    let test_cases = ["1", "z", "1a1", "z4", "z401", "z40100001"];

    for rank in test_cases {
        println!("{} -> {} <- {}", rank, rank, rank);
        let rank1 = Rank::new(rank).unwrap();
        let rank2 = Rank::new(rank).unwrap();
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
