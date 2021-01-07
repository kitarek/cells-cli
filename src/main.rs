use structopt::StructOpt;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp;
use colored::*;

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
    name:        String,
    extra_lines: Vec<String>
}

struct FileCellLength {
    name_len:        usize,
    cols_num:        usize,
    extra_line_lens: Vec<usize>
}

//struct ColumnColors {
//    color_names:     Vec<Color>
//    let color_res : Result<Color, ()> = "zorglub".parse();
 //   "red string".color(color_res.unwrap_or(Color::Red));
//}

fn load_file(path :String) -> std::io::Result<Vec<String>> {
    let f = File::open(&path)?;
    let reader = BufReader::new(f);
    let mut counter = 0;
    let mut lines = Vec::new();
    for new_line in reader.lines() {
        counter += 1;
        let line = new_line.unwrap_or("".to_string());
        if counter > 10 {
            break;
        }
        lines.push(line);
    }
    Ok(lines)
}

fn main() {
    let args = Cli::from_args();
    if let Ok(entries) = fs::read_dir(&args.path) {
        let mut lines: Vec<FileCell> = Vec::new();
        let mut len: FileCellLength = FileCellLength {name_len: 0, cols_num: 0, extra_line_lens: std::iter::repeat(0).take(10).collect::<Vec<_>>()};
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Ok(top_10_lines) = load_file(entry.path().into_os_string().into_string().unwrap()) {
                            let cell = FileCell {
                                name: entry.file_name().to_str().unwrap_or("-/-").to_string(),
                                extra_lines: top_10_lines
                            };
                            lines.push(cell.clone());
                            len.name_len = cmp::max(len.name_len, cell.name.chars().count());
                            let mut index = 0;
                            for extra_line in &cell.extra_lines {
                                let current_len = len.extra_line_lens[index];
                                len.extra_line_lens[index] = cmp::max(current_len, extra_line.chars().count());
                                index += 1;
                            }
                            len.cols_num = cmp::max(len.cols_num, index);
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
        let color_filename : Result<Color, ()> = "blue".parse();
        let color_1st : Result<Color, ()> = "yellow".parse();
        let color_2nd : Result<Color, ()> = "green".parse();
        for line in lines {
          print!("{:sep2width1$}",
            &line.name.color(color_filename.unwrap_or(Color::Red)),
            sep = '-'
            width1 = &len.name_len);
          let mut index = 0;
          let cols_num = line.extra_lines.len();
          for extra_line in &line.extra_lines {
            let effective_color = if index % 2 == 0 { color_1st } else { color_2nd };
            let effective_width = if index < cols_num - 1
                { len.extra_line_lens[index] } else { 0usize };
            if extra_line.chars().count() > 0usize || effective_width > 0usize {
              let effective_line = if effective_width == 0usize
                { extra_line.trim_end() } else { extra_line };
              print!("  {:width1$}",
                effective_line.color(effective_color.unwrap_or(Color::White)),
                width1 = effective_width);
            }
            index += 1;
          }
          println!("");
        }
    } else {
        eprintln!("The path: {:?} is not a directory", &args.path);
    }
}
