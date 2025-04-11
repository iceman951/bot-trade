use polars::prelude::*;

fn read_csv(file_path: &str) -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(file_path.into()))?
            .finish()
}

fn main() {
    let file_path = "./assets/BTC-Daily.csv";
    let df = read_csv(&file_path).unwrap();
    println!("{:?}", df);

}
