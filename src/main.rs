use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Record {
    id: String,
    months: Vec<Month>,
}

#[derive(Debug)]
struct Month {
    index: i16,
    name: String,
}

const MONTHS_LIST: [&str; 12] = [
    "साउन",
    "भदौ",
    "असोज",
    "कात्तिक",
    "मङ्सिर",
    "पुस",
    "माघ",
    "फागुन",
    "चैत",
    "वैशाख",
    "जेठ",
    "असार",
];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];
    let output_file_path = &args[2];

    let csv = fs::read_to_string(input_file_path)?;

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut writer = csv::Writer::from_path(output_file_path)?;

    for record in reader.records() {
        let record = record?;
        let json: Value = serde_json::from_str(&record[1])?;

        if let Value::Array(months) = json {
            let mut record = Record {
                id: record[0].to_string(),
                months: months
                    .into_iter()
                    .enumerate()
                    .filter_map(|(_i, value)| {
                        if let Value::String(name) = value {
                            // Find the corresponding index from MONTHS_LIST
                            let index = MONTHS_LIST.iter().position(|&m| m == name)?;
                            Some(Month {
                                index: index as i16,
                                name,
                            })
                        } else {
                            None
                        }
                    })
                    .collect(),
            };

            record.months.sort_by_key(|month| month.index);

            let months_json = serde_json::to_string(
                &record
                    .months
                    .iter()
                    .map(|m| m.name.clone())
                    .collect::<Vec<_>>(),
            )?;

            writer.write_record(&[record.id, months_json])?;
        } else {
            eprintln!("Expected JSON array, got: {}", json);
        }
    }

    writer.flush()?;
    println!("Output Successfully Written At : {}", &output_file_path);
    Ok(())
}
