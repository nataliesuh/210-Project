use petgraph::graph::Graph;
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};
use ndarray::Array1;

fn euclidean_distance(vec_a: &Array1<f64>, vec_b: &Array1<f64>) -> f64 {
    vec_a
        .iter()
        .zip(vec_b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn create_graph(
    team_averages: &HashMap<String, Array1<f64>>,
) -> (Graph<String, f64, Undirected>, HashMap<String, petgraph::graph::NodeIndex>) {
    let mut graph = Graph::<String, f64, Undirected>::new_undirected();
    let mut team_indices: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

    let threshold = 5.0;
    let filtered_teams: HashSet<&String> = team_averages
        .iter()
        .filter(|(_, avg)| avg.sum() > 10.0)
        .map(|(team_id, _)| team_id)
        .collect();

    for (team_id, _) in team_averages.iter() {
        if filtered_teams.contains(team_id) {
            let node_idx = graph.add_node(team_id.clone());
            team_indices.insert(team_id.clone(), node_idx);
        }
    }

    for (team_id_a, avg_a) in team_averages {
        if !filtered_teams.contains(team_id_a) {
            continue;
        }
        for (team_id_b, avg_b) in team_averages {
            if team_id_a != team_id_b && filtered_teams.contains(team_id_b) {
                let distance = euclidean_distance(avg_a, avg_b);
                if distance < threshold {
                    graph.add_edge(
                        team_indices[team_id_a],
                        team_indices[team_id_b],
                        distance,
                    );
                }
            }
        }
    }

    (graph, team_indices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array1;

    #[test]
    fn test_create_graph() {
        let mut team_averages = HashMap::new();
        team_averages.insert("TeamA".to_string(), Array1::from(vec![15.0, 25.0]));
        team_averages.insert("TeamB".to_string(), Array1::from(vec![16.0, 26.0]));
        team_averages.insert("TeamC".to_string(), Array1::from(vec![50.0, 60.0]));

        let (graph, team_indices) = create_graph(&team_averages);

        assert_eq!(graph.node_count(), 3);
        assert!(team_indices.contains_key("TeamA"));
    }
}
