#![cfg(test)]

#[test]
fn test_random_names() {
    // property testing like.
    for size in 1usize..100 {
        assert_eq!(size, super::more_strings::name_gen(size).len());
    }
}
