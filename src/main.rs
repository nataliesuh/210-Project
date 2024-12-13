mod data;
mod graph;
mod knn;
mod visualization;

use data::DataLoader;
use graph::create_graph;
use knn::find_nearest_teams;
use visualization::draw_graph;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "game_teams_stats.csv";

    println!("Enter the year for analysis (e.g., 2016):");
    let mut year = String::new();
    io::stdin().read_line(&mut year)?;
    let year = year.trim();

    let data_loader = DataLoader::new(file_path, year);
    let team_averages = data_loader.load_and_average()?;

    if team_averages.is_empty() {
        println!("No data found for the year: {}", year);
        return Ok(());
    }

    let (graph, _team_indices) = create_graph(&team_averages);

    draw_graph(&graph, "team_graph.png")?;
    println!("Graph visualization saved as team_graph.png");

    let neighbors = find_nearest_teams(&team_averages, 3);

    println!("Enter a team ID to see its nearest neighbors (e.g., Bruins):");
    let mut selected_team = String::new();
    io::stdin().read_line(&mut selected_team)?;
    let selected_team = selected_team.trim();

    if let Some(neighbor_list) = neighbors.get(selected_team) {
        println!("Team: {} -> Nearest Neighbors:", selected_team);
        for (neighbor, distance) in neighbor_list {
            println!("Neighbor: {}, Distance: {:.4}", neighbor, distance);
        }
    } else {
        println!("Team ID not found in the data.");
    }

    Ok(())
}
