pub mod csv_processing {
    use csv::ReaderBuilder;
    use ndarray::Array1;
    use std::collections::HashMap;
    use std::error::Error;

    pub struct DataLoader {
        pub file_path: String,
        pub year: String,
    }

    impl DataLoader {
        pub fn new(file_path: &str, year: &str) -> Self {
            Self {
                file_path: file_path.to_string(),
                year: year.to_string(),
            }
        }

        pub fn load_and_average(&self) -> Result<HashMap<String, Array1<f64>>, Box<dyn Error>> {
            let mut rdr = ReaderBuilder::new()
                .has_headers(true)
                .from_path(&self.file_path)?;

            let headers = rdr.headers()?.clone();

            let mut team_id_idx = None;
            let mut game_id_idx = None;
            let mut numeric_columns = Vec::new();

            for (i, header) in headers.iter().enumerate() {
                if header == "team_id" {
                    team_id_idx = Some(i);
                } else if header == "game_id" {
                    game_id_idx = Some(i);
                } else if header.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    numeric_columns.push(i);
                }
            }

            let team_id_idx = team_id_idx.ok_or("team_id column not found")?;
            let game_id_idx = game_id_idx.ok_or("game_id column not found")?;

            let mut team_data: HashMap<String, Vec<Array1<f64>>> = HashMap::new();

            for result in rdr.records() {
                let record = result?;

                let game_year = &record[game_id_idx][..4];
                if game_year != self.year {
                    continue;
                }

                let team_id = record[team_id_idx].to_string();

                let row_data: Vec<f64> = numeric_columns
                    .iter()
                    .map(|&idx| record[idx].parse::<f64>().unwrap_or(0.0))
                    .collect();

                team_data
                    .entry(team_id)
                    .or_insert_with(Vec::new)
                    .push(Array1::from(row_data));
            }

            let team_averages: HashMap<String, Array1<f64>> = team_data
                .into_iter()
                .map(|(team_id, rows)| {
                    let count = rows.len();
                    let sum = rows.iter().fold(Array1::zeros(rows[0].len()), |acc, row| acc + row);
                    (team_id, sum / count as f64)
                })
                .collect();

            Ok(team_averages)
        }
    }
}
