use std::io;
const OPTIONS: [&str; 3] = ["steen", "papier", "schaar"];
const RESULTS: [&str; 3] = [
    "Het is gelijkspel!",
    "Speler twee heeft gewonnen!",
    "Speler één heeft gewonnen!"
];

fn main() {
    let player_answers: (usize, usize) = (get_answer(), get_answer());
    let mut player_wins: &str = "Something went wrong!";
    for i in 0.. {
        if player_answers.0 + i % 3 == player_answers.1 {
            player_wins = RESULTS[i];
            break;
        }
    }
    println!("{}", player_wins)

}

fn get_answer() -> usize {
    let mut result: String = String::new();
    let answer: &str;
    io::stdin().read_line(&mut result).expect("epic fail");
    answer = (&result as &str).trim();
    OPTIONS.iter().position(|&r| r == answer).unwrap()
}