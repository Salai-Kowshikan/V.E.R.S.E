use std::error::Error;
use std::fs;
use csv::ReaderBuilder;
use std::collections::HashMap;

const BATCH_SIZE: usize = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "dataset.csv";
    let batches_dir = "batches";
    fs::create_dir_all(batches_dir)?;

    println!("📂 Reading dataset from {input_path}...");

    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(input_path)?;
    let headers = rdr.headers()?.clone();
    let mut batch_records = Vec::new();
    let mut batch_count = 0;

    for (i, result) in rdr.deserialize().enumerate() {
        let record: HashMap<String, String> = result?;
        batch_records.push(record);

        if (i + 1) % BATCH_SIZE == 0 {
            batch_count += 1;
            save_batch(&batch_records, batch_count, batches_dir, &headers)?;
            batch_records.clear();
        }
    }

    if !batch_records.is_empty() {
        batch_count += 1;
        save_batch(&batch_records, batch_count, batches_dir, &headers)?;
    }

    println!("✅ Created {batch_count} batch files in `{batches_dir}`.");
    Ok(())
}

fn save_batch(
    records: &[HashMap<String, String>],
    batch_num: usize,
    dir: &str,
    headers: &csv::StringRecord,
) -> Result<(), Box<dyn Error>> {
    let filename = format!("{}/batch_{}.csv", dir, batch_num);
    let mut wtr = csv::Writer::from_path(&filename)?;
    wtr.write_record(headers)?;

    for record in records {
        let row: Vec<&str> = headers.iter().map(|h| record.get(h).map(|s| s.as_str()).unwrap_or("")).collect();
        wtr.write_record(&row)?;
    }

    wtr.flush()?;
    println!("🧩 Saved {filename}");
    Ok(())
}
