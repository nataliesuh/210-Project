mod data;
mod graph;
mod knn;

use crate::data::DataLoader;
use crate::graph::create_graph;
use crate::knn::{find_nearest_teams, knn};
use tempfile::tempdir;
use std::fs::File;
use std::io::Write;
use ndarray::{Array1, Array2};
use std::collections::HashMap;

#[test]
fn test_data_loader_new() {
    let data_loader = DataLoader::new("test_file.csv", "2023");
    assert_eq!(data_loader.file_path, "test_file.csv");
    assert_eq!(data_loader.year, "2023");
}

#[test]
fn test_load_and_average() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test_data.csv");

    let csv_content = "team_id,game_id,stat1,stat2\nTeamA,20230101,10,20\nTeamA,20230102,30,40\nTeamB,20230101,50,60";
    let mut file = File::create(&file_path).unwrap();
    file.write_all(csv_content.as_bytes()).unwrap();

    let data_loader = DataLoader::new(file_path.to_str().unwrap(), "2023");
    let averages = data_loader.load_and_average().unwrap();

    assert!(averages.contains_key("TeamA"));
    assert!(averages.contains_key("TeamB"));
    assert_eq!(averages["TeamA"], Array1::from(vec![20.0, 30.0]));
    assert_eq!(averages["TeamB"], Array1::from(vec![50.0, 60.0]));
}

#[test]
fn test_create_graph() {
    let mut team_averages = HashMap::new();
    team_averages.insert("TeamA".to_string(), Array1::from(vec![15.0, 25.0]));
    team_averages.insert("TeamB".to_string(), Array1::from(vec![16.0, 26.0]));
    team_averages.insert("TeamC".to_string(), Array1::from(vec![50.0, 60.0]));
    
    let (graph, team_indices) = create_graph(&team_averages);
    assert_eq!(graph.node_count(), 3);
    assert!(team_indices.contains_key("TeamA"));
    assert!(team_indices.contains_key("TeamB"));
}

#[test]
fn test_knn() {
    let data = Array2::from_shape_vec((3, 2), vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0]).unwrap();
    let query = Array1::from(vec![1.5, 1.5]);

    let neighbors = knn::knn(&data, &query, 2);
    assert_eq!(neighbors.len(), 2);
    assert!(neighbors[0].1 <= neighbors[1].1);
}

#[test]
fn test_find_nearest_teams() {
    let mut team_averages = HashMap::new();
    team_averages.insert("TeamA".to_string(), Array1::from(vec![1.0, 1.0]));
    team_averages.insert("TeamB".to_string(), Array1::from(vec![2.0, 2.0]));
    team_averages.insert("TeamC".to_string(), Array1::from(vec![3.0, 3.0]));

    let nearest_teams = find_nearest_teams(&team_averages, 2);
    assert!(nearest_teams.contains_key("TeamA"));
    assert_eq!(nearest_teams["TeamA"].len(), 2);
    assert_eq!(nearest_teams["TeamA"][0].0, "TeamB");
}
