use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "csv.pest"]
struct CSVParser;

fn main() {
    let csv = std::fs::read_to_string("src/example.csv").expect("csv file should be readable");
    println!("{csv}");
    let mut file = CSVParser::parse(Rule::file, &csv)
        .expect("csv file should be valid")
        .next()
        .unwrap();

    let mut record_count = 0;
    let mut field_sum = 0.0;

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::EOI => (),
            Rule::record => {
                record_count += 1;

                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                }
            }
            _ => unreachable!(),
        }
    }
    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}
