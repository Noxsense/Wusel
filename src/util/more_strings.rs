/// Generate a name by using a capital alphabetic char
/// and adding up to the given _lenght_ more chars.
/// Vocals after Consonants should be more common than another Consonants.
pub fn name_gen(length: usize) -> String {
    // TODO (2021-12-11) better weighted chars. maybe markov chain. (also less visible.)

    /// Just vocals.
    const VOCALS: [char; 12] = ['a', 'e', 'i', 'o', 'u', 'y', 'A', 'E', 'I', 'O', 'U', 'Y'];

    /// Weighted Chars after Vocals.
    /// With Higher change for consonants.
    const ALPHA_AFTER_VOCAL: [char; 66] = [
        'a', 'e', 'i', 'o', 'u', 'y', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q',
        'r', 's', 't', 'v', 'w', 'x', 'z', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p',
        'q', 'r', 's', 't', 'v', 'w', 'x', 'z', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n',
        'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z',
    ];

    /// Weighted Chars after Consonants
    /// With slightly higher chance for vocals.
    const ALPHA_AFTER_CONSONANT: [char; 100] = [
        'a', 'e', 'a', 'e', 'i', 'o', 'u', 'y', 'a', 'e', 'i', 'o', 'u', 'y', 'a', 'e', 'i', 'o', 'u',
        'y', 'a', 'e', 'i', 'o', 'u', 'y', 'a', 'e', 'i', 'o', 'u', 'y', 'a', 'e', 'i', 'o', 'u', 'y',
        'a', 'e', 'i', 'o', 'u', 'y', 'a', 'e', 'i', 'o', 'u', 'y', 'a', 'e', 'i', 'o', 'u', 'y', 'a',
        'e', 'i', 'o', 'u', 'y', 'a', 'e', 'i', 'o', 'u', 'y', 'a', 'e', 'i', 'o', 'u', 'y', 'a', 'e',
        'i', 'o', 'u', 'y', 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's',
        't', 'v', 'w', 'x', 'z',
    ];

    let mut chars: Vec<char> = vec![
        ((rand::random::<u8>() % 26) + b'A') as char, // first letter.
    ];

    let mut random_index: usize;

    for i in 1usize..length {
        if VOCALS.contains(chars.get(i - 1).unwrap()) {
            random_index = rand::random::<usize>() % ALPHA_AFTER_VOCAL.len();
            chars.push(ALPHA_AFTER_VOCAL[random_index]);
        } else {
            random_index = rand::random::<usize>() % ALPHA_AFTER_CONSONANT.len();
            chars.push(ALPHA_AFTER_CONSONANT[random_index]);
        }
    }

    chars.into_iter().collect()
}
