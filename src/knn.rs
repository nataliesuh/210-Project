use ndarray::{Array1, Array2};
use std::collections::HashMap;

fn euclidean_distance(point1: &Array1<f64>, point2: &Array1<f64>) -> f64 {
    point1
        .iter()
        .zip(point2.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn knn(
    data: &Array2<f64>,
    query: &Array1<f64>,
    k: usize,
) -> Vec<(usize, f64)> {
    let mut distances: Vec<(usize, f64)> = data
        .axis_iter(ndarray::Axis(0))
        .enumerate()
        .map(|(idx, row)| {
            let row = row.to_owned();
            (idx, euclidean_distance(&row, query))
        })
        .collect();

    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    distances.into_iter().take(k).collect()
}

pub fn find_nearest_teams(
    team_averages: &HashMap<String, Array1<f64>>,
    k: usize,
) -> HashMap<String, Vec<(String, f64)>> {
    let keys: Vec<String> = team_averages.keys().cloned().collect();
    let data: Array2<f64> = Array2::from_shape_vec(
        (team_averages.len(), team_averages.values().next().unwrap().len()),
        team_averages.values().flat_map(|arr| arr.iter().cloned()).collect(),
    )
    .unwrap();

    keys.iter()
        .enumerate()
        .map(|(i, team)| {
            let query = data.row(i).to_owned();
            let mut neighbors = knn(&data, &query, k + 1);
            neighbors.retain(|(idx, _)| keys[*idx] != *team);
            neighbors.truncate(k);
            neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let neighbor_pairs = neighbors
                .into_iter()
                .map(|(idx, dist)| (keys[idx].clone(), dist))
                .collect();
            (team.clone(), neighbor_pairs)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{Array1, Array2};

    #[test]
    fn test_knn() {
        let data = Array2::from_shape_vec((3, 2), vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0]).unwrap();
        let query = Array1::from(vec![1.5, 1.5]);

        let neighbors = knn(&data, &query, 2);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors[0].1 <= neighbors[1].1);
    }

    #[test]
    fn test_find_nearest_teams() {
        let mut team_averages = HashMap::new();
        team_averages.insert("TeamA".to_string(), Array1::from(vec![1.0, 1.0]));
        team_averages.insert("TeamB".to_string(), Array1::from(vec![2.0, 2.0]));
        team_averages.insert("TeamC".to_string(), Array1::from(vec![3.0, 3.0]));
        team_averages.insert("TeamD".to_string(), Array1::from(vec![4.0, 4.0]));

        let nearest_teams = find_nearest_teams(&team_averages, 2);

        assert!(nearest_teams.contains_key("TeamA"));
        assert_eq!(nearest_teams["TeamA"].len(), 2);
        assert_eq!(nearest_teams["TeamA"][0].0, "TeamB");
        assert_eq!(nearest_teams["TeamA"][1].0, "TeamC");
    }
}
