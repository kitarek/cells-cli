use structopt::StructOpt;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp;

#[derive(StructOpt)]
/// Print subdirectories and short contents of small files as spredsheet cells.
///
/// Selected tree of files and directories would be printed on screen in a tabular
/// format (padded with spaces) or with tab characters.
///
/// Allows configuration of headers and simple re-calculation and post-processing.
struct Cli {
    #[structopt(parse(from_os_str))]
    /// the root directory of spreadsheet cells document
    path: std::path::PathBuf
}

#[derive(Clone)]
struct FileCell {
    name: String,
    line: String
}

struct FileCellLength {
    name_len: usize,
    line_len: usize
}

fn load_file(path :String) -> std::io::Result<String> {
    let f = File::open(&path)?;
    let reader = BufReader::new(f);
    let mut line = String::new();
    for new_line in reader.lines() {
        line = new_line.unwrap_or("".to_string());
        break;
    }
    Ok(line)
}

fn main() {
    let args = Cli::from_args();
    if let Ok(entries) = fs::read_dir(&args.path) {
        let mut lines: Vec<FileCell> = Vec::new();
        let mut len: FileCellLength = FileCellLength {name_len: 0, line_len: 0};
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Ok(line) = load_file(entry.path().into_os_string().into_string().unwrap()) {
                            let cell = FileCell {
                                name: entry.file_name().to_str().unwrap_or("-/-").to_string(),
                                line: line
                            };
                            lines.push(cell.clone());
                            len.name_len = cmp::max(len.name_len, cell.name.chars().count());
                            len.line_len = cmp::max(len.line_len, cell.line.chars().count());
                        } else {
                            eprintln!("Couldn't read file {:?}", &entry.path());
                        }
                    }
                } else {
                    eprintln!("Couldn't get file type for {:?}",
                        entry.path());
                }
            }
        }
        for line in lines {
            println!("{:width1$}  {:width2$}", &line.name, &line.line, width1 = &len.name_len, width2 = &len.line_len);
        }
    } else {
        eprintln!("The path: {:?} is not a directory", &args.path);
    }
}
