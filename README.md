# Month Reorder

A Rust application that reorders Nepali months according to a base order. This tool takes an input CSV file containing month data and produces an output CSV file with reordered months.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [File Formats](#file-formats)
- [License](#license)

## Overview

This project reorders Nepali months based on a specified base order. The application reads an input CSV file with a list of IDs and months in JSON format, and writes the reordered data to an output CSV file.

## Installation

Ensure you have Rust and Cargo installed on your system. If not, you can follow the installation instructions from the [official Rust website](https://www.rust-lang.org/).

Clone the repository:

```sh
git clone https://github.com/yourusername/month-reorder.git
cd month-reorder
```

Build the project:

```sh
cargo build --release
```

## Usage

### Running the Application

#### If the project is not built:

```sh
cargo run -- --input_path <input_file.csv> --output_path <output_file.csv>
```

Replace `<input_file.csv>` with the path to your input CSV file and `<output_file.csv>` with the desired path for the output CSV file.

#### If the project is built:

Navigate to the release directory:

```sh
cd target/release
```

Run the executable:

```sh
./month-reorder --input_path <input_file.csv> --output_path <output_file.csv>
```

Replace `<input_file.csv>` with the path to your input CSV file and `<output_file.csv>` with the desired path for the output CSV file.

## File Formats

### Input File

The input file should be a CSV with the following columns:

- `id` - A unique identifier for each entry.
- `months` - A JSON array containing the Nepali months.

Example input file (`input.csv`):

```csv
id,months
1,"[""साउन"",""भदौ"",""असोज"",""कात्तिक"",""मङ्सिर"",""पुस"",""माघ"",""फागुन"",""चैत"",""वैशाख"",""असार"",""जेठ""]"
2,"[""माघ"",""पुस"",""साउन"",""भदौ"",""असोज"",""कात्तिक"",""मङ्सिर"",""फागुन"",""चैत"",""वैशाख"",""असार"",""जेठ""]"
```

### Output File

The output file will be a CSV with the reordered months according to the base order:

Base order:

```json
["साउन","भदौ","असोज","कात्तिक","मङ्सिर","पुस","माघ","फागुन","चैत","वैशाख","असार","जेठ"]
```

Example output file (`output.csv`):

```csv
id,months
1,"[""साउन"",""भदौ"",""असोज"",""कात
