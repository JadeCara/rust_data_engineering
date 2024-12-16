use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use sha3::Digest;
use sha3::Sha3_256;
use std::collections::HashMap;

// List of phrases
static PHRASES: [&str; 10] = [
    "Be curious, not judgmental.",
    "Football is life, but it's not the life.",
    "Believe.",
    "I believe in communism. Rom-communism, that is.",
    "You know what the happiest animal on Earth is? A goldfish. You know why? Got a 10-second memory. Be a goldfish, Sam.",
    "I think that you might be so sure a person is one thing, that sometimes you completely miss who they really are.",
    "I promise you, there is something worse out there than being sad, and that's being alone and being sad. Ain't no one in this room alone.",
    "Winning isn't everything, but wanting to win is.",
    "It's important to find people who challenge and inspire you, people who care about you and push you to be your best. And remember, it's okay to ask for help.",
    "I'm like an incomplete list of Madeline Kahn's best films. I ain't got no clue.",
];

// Generate random phrases
pub fn generate_random_phrases() -> Vec<&'static str> {
    let mut rng = thread_rng();
    let mut phrases = Vec::new();

    for &phrase in PHRASES.iter() {
        let copies = rng.gen_range(1..=3);

        for _ in 0..copies {
            phrases.push(phrase);
        }
    }

    phrases.shuffle(&mut rng);

    phrases
}

// Analyze duplicates
pub fn analyze_duplicates(phrases: &[&str]) {
    let mut hashes: HashMap<_, (usize, &str)> = HashMap::new();
    println!("Total number of phrases: {}", phrases.len());

    for phrase in phrases {
        let hash = Sha3_256::digest(phrase.as_bytes());
        let entry = hashes.entry(hash).or_insert((0, phrase));
        entry.0 += 1;
    }

    let total_unique_phrases = hashes.len();

    let mut total_unique_duplicates = 0;
    let mut total_combined_duplicates = 0;

    for (hash, (count, phrase)) in &hashes {
        if *count > 1 {
            total_unique_duplicates += 1;
            total_combined_duplicates += count - 1; // subtract one to exclude the original
            println!("{} - {} times: {}", hex::encode(hash), count, phrase);
        }
    }

    println!("Total Unique Phrases: {}", total_unique_phrases);
    println!("Total Unique Duplicates: {}", total_unique_duplicates);
    println!("Total Combined Duplicates: {}", total_combined_duplicates);
}
