use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashSet;

/// load, cleans, and saves a CSV file.
pub fn load_and_clean_csv(file_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    println!("Starting to process file: {}", file_path);

    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);

    let mut wtr = WriterBuilder::new().from_path(output_path)?;

    let headers = rdr.headers()?.clone();
    wtr.write_record(&headers)?;

    let mut unique_rows = HashSet::new();
    let mut rows_processed = 0;
    let mut rows_saved = 0;

    for result in rdr.records() {
        let record = result?;

        let row: Vec<String> = record.iter().map(|s| s.trim().to_lowercase()).collect();

        if unique_rows.insert(row.clone()) {
            wtr.write_record(&row)?;
            rows_saved += 1;
        }
        rows_processed += 1;
    }

    wtr.flush()?;
    println!(
        "Processed {} rows, saved {} unique rows for file: {}",
        rows_processed, rows_saved, file_path
    );

    Ok(())
}
