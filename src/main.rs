use csv::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./assets/BTC-Daily.csv";

    let mut rdr = Reader::from_path(file_path)?;

    // Process the CSV data
    for result in rdr.records() {
        let record = result?;
        // Process each record here
        println!("{:?}", record);
    }
    
    Ok(())
}
