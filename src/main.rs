use rand::Rng;
use std::io;

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn get_user_move() -> Move {
    println!("Enter your move (rock, paper, or scissors):");
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().to_lowercase().as_str() {
            "rock" => return Move::Rock,
            "paper" => return Move::Paper,
            "scissors" => return Move::Scissors,
            _ => println!("Invalid move. Please enter rock, paper, or scissors."),
        }
    }
}

fn get_ai_move() -> Move {
    match rand::thread_rng().gen_range(0..=2) {
        0 => Move::Rock,
        1 => Move::Paper,
        _ => Move::Scissors,
    }
}

fn determine_winner(user: &Move, ai: &Move) -> Option<&'static str> {
    if user == ai {
        Some("It's a tie!")
    } else if (user == &Move::Rock && ai == &Move::Scissors)
        || (user == &Move::Paper && ai == &Move::Rock)
        || (user == &Move::Scissors && ai == &Move::Paper)
    {
        Some("You win!")
    } else {
        Some("AI wins!")
    }
}

fn main() {
    let mut user_score = 0;
    let mut ai_score = 0;

    println!("Enter the number of rounds you want to play:");

    let mut max_rounds = String::new();
    io::stdin()
        .read_line(&mut max_rounds)
        .expect("Failed to read line");

    let max_rounds: usize = match max_rounds.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for the number of rounds. Defaulting to 10 rounds.");
            10
        }
    };

    let mut current_round = 0;

    loop {
        println!("Current Score - You: {} | AI: {}", user_score, ai_score);

        if current_round == max_rounds {
            println!("Game over. Max rounds reached.");
            break;
        }

        let user_move = get_user_move();
        let ai_move = get_ai_move();

        println!("You chose: {:?}", user_move);
        println!("AI chose: {:?}", ai_move);

        match determine_winner(&user_move, &ai_move) {
            Some(result) => println!("{}", result),
            None => println!("Error determining the winner."),
        }

        if determine_winner(&user_move, &ai_move) == Some("You win!") {
            user_score += 1;
        } else if determine_winner(&user_move, &ai_move) == Some("AI wins!") {
            ai_score += 1;
        }

        current_round += 1;
        println!("------------------");
    }
}
