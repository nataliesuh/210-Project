mod data; 
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the input and output file paths
    let datasets = vec![
        ("game_penalties.csv", "cleaned_game_penalties.csv"),
        ("game_teams_stats.csv", "cleaned_game_teams_stats.csv"),
        ("game.csv", "cleaned_game.csv"),
        ("player_info.csv", "cleaned_player_info.csv"),
        ("team_info.csv", "cleaned_team_info.csv"),
    ];

    // Load and clean each dataset
    for (input, output) in &datasets {
        match data::load_and_clean_csv(input, output) {
            Ok(_) => println!("Successfully cleaned and saved {}", output),
            Err(e) => eprintln!("Error processing {}: {}", input, e),
        }
    }

    println!("All datasets processed successfully.");
    Ok(())
}
