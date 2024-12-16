/*
Generates random duplicate phrases from a list of phrases
and prints the number of unique phrases and the number of duplicate phrases.

Example output:

Total number of phrases: 23
2a2e73e2a2d6b56eee4c5c8ad738020d2434a2af922e28293ae7911ae7bddcb2 - 3 times: Believe.
e55ec34eff9881511727b17a583919b6e0e49e15799693a3aa000b1428a35f9d - 2 times: Winning isn't everything, but wanting to win is.
f61635abca438d9ea7576b240a6d04163c4a2a33d9aced6feab999feb31cf8d1 - 2 times: Football is life, but it's not the life.
6a4f1a4e33e60565fe9be08113b3700cd87b4e5b9b99a2cd73d8cc679b070f01 - 3 times: I think that you might be so sure a person is one thing, that sometimes you completely miss who they really are.
7501e06b5c21dd83151e3b924e9513e937cc1f083faf017987ba2a4e982536d4 - 2 times: It's important to find people who challenge and inspire you, people who care about you and push you to be your best. And remember, it's okay to ask for help.
2a77903cddd57b9612116009d78845025a577b18ffabfa1418e25cb9adc21ddb - 2 times: Be curious, not judgmental.
b0a8c43b81216d3c2226dd492561d6db50adcdd9eccf7a1b1c9bc0588a76cae2 - 3 times: I'm like an incomplete list of Madeline Kahn's best films. I ain't got no clue.
eeb975a58d29186433e4edbc45cf57869b56cc12dec28addada2961e706f548c - 2 times: I promise you, there is something worse out there than being sad, and that's being alone and being sad. Ain't no one in this room alone.
8ea1b6258b0e074076d7bb75d5c8e8bdf5d571efac0a1ba2370083214d08fd3c - 3 times: You know what the happiest animal on Earth is? A goldfish. You know why? Got a 10-second memory. Be a goldfish, Sam.
Total Unique Phrases: 10
Total Unique Duplicates: 9
Total Combined Duplicates: 13

*/
use duplicate_text_detection::generate_random_phrases;

fn main() {
    let phrases = generate_random_phrases();
    duplicate_text_detection::analyze_duplicates(&phrases);
}
