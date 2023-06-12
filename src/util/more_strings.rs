/// Just vocals.
const VOCALS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

/// Just consonants.
const CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'z',
];

fn random_vocal_or_constant(probability_of_vocal: usize) -> char {
    if rand::random::<usize>() % 100 < probability_of_vocal {
        // pick vocal
        VOCALS[rand::random::<usize>() % VOCALS.len()]
    } else {
        // pick consonant
        CONSONANTS[rand::random::<usize>() % CONSONANTS.len()]
    }
}

/// Generate a name by using a capital alphabetic char
/// and adding up to the given _lenght_ more chars.
/// Vocals after Consonants should be more common than another Consonants.
pub fn name_gen(length: usize) -> String {
    // TODO (2021-12-11) better weighted chars. maybe markov chain. (also less visible.)

    let mut chars: Vec<char> = vec![
        ((rand::random::<u8>() % 26) + b'A') as char, // first letter.
    ];

    let mut random_index: usize;

    for i in 1usize..length {
        if VOCALS.contains(chars.get(i - 1).unwrap()) {
            // last char is VOCAL
            // => higher percentage of consonants (66%)
            chars.push(random_vocal_or_constant(34usize));
        } else {
            // last char is constant.
            // => higher percentage of vocals (70%)
            chars.push(random_vocal_or_constant(70usize));
        }
    }

    chars.into_iter().collect()
}
