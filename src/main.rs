/* This file is part of rust-redundant-sentences:
   Counts redundant sentences in text

    Copyright 2020 Fabio A. Correa Duran facorread@gmail.com

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::collections::HashMap;

enum Mode {
    Header,
    TextBody,
}

/// Represents a sentence.
struct Sentence {
    /// Text of the sentence.
    text: String,
    /// Count of occurrences
    count: i32,
    /// Report of occurrences
    report: Vec<String>,
}

impl Sentence {
    fn push(&mut self, s: &str) {
        match self.report.last() {
            None => self.report.push(String::from(s)),
            Some(item) if item != s => self.report.push(String::from(s)),
            _ => (),
        }
        self.count += 1;
    }
}

impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.count)?;
        for v in &self.report {
            write!(f, " {}", v)?;
        }
        write!(f, "\t{}", self.text)
    }
}

static TRY_AGAIN_MSG: &str = "Please review this problem. Before trying again, make sure input/ is a valid directory and your files are in txt format.";
static USAGE_MSG: &str =
    "Before starting rust-redundant-sentences, save your plain text to the input/ directory. Only txt files are accepted.";

fn create_output_file() -> Option<std::fs::File> {
    match std::fs::File::create("report.txt") {
        Ok(out_file) => Some(out_file),
        Err(e) => {
            println!("Could not create report.txt: {}\n{}", e, TRY_AGAIN_MSG);
            None
        }
    }
}

fn get_input_path() -> Option<std::path::PathBuf> {
    let input_path = std::path::PathBuf::from("input");
    if input_path.exists() {
        if input_path.is_dir() {
            Some(input_path)
        } else {
            println!("input/ is not a directory. {}", TRY_AGAIN_MSG);
            None
        }
    } else {
        // The input directory does not exist. Try to create it.
        if let Err(e) = std::fs::create_dir(input_path) {
            println!("Could not create the input/ directory:\n{}\nPlease review this problem. Then, manually create the input/ directory.", e);
        }
        println!("{}", USAGE_MSG);
        None
    }
}

fn open_input_file(
    result_dir_entry: std::result::Result<std::fs::DirEntry, std::io::Error>,
) -> Option<(String, std::fs::File)> {
    let txt_ext = Some(std::ffi::OsStr::new("txt"));
    match result_dir_entry {
        Ok(dir_entry) => {
            let file_path = dir_entry.path();
            let os_string_file_name = dir_entry.file_name();
            let file_name = os_string_file_name.to_string_lossy();
            if let Some(file_name_no_extension) = file_name.get(..file_name.len() - 4) {
                match dir_entry.file_type() {
                    Ok(file_type) => {
                        if file_type.is_file() {
                            if file_path.extension() == txt_ext {
                                match std::fs::File::open(file_path) {
                                    Ok(input_file_unbuffered) => Some((
                                        file_name_no_extension.to_string(),
                                        input_file_unbuffered,
                                    )),
                                    Err(e) => {
                                        println!(
                                            "Error opening input/{}:\n{}\n{}",
                                            file_name, e, TRY_AGAIN_MSG
                                        );
                                        None
                                    }
                                }
                            } else {
                                println!(
                                    "Error: input/{} is not a txt file.\n{}",
                                    file_name, TRY_AGAIN_MSG
                                );
                                None
                            }
                        } else {
                            println!(
                                "Error: input/{} is not a regular file.\n{}",
                                file_name, TRY_AGAIN_MSG
                            );
                            None
                        }
                    }
                    Err(e) => {
                        println!(
                            "Error when checking the type of input/{}:\n{}\n{}",
                            file_name, e, TRY_AGAIN_MSG
                        );
                        None
                    }
                }
            } else {
                println!(
                    "Error: After removing invalid characters, file name input/{} becomes too short\n{}",
                    file_name, TRY_AGAIN_MSG
                );
                None
            }
        }
        Err(e) => {
            println!(
                "Error with an entry in the input/ directory:\n{}\n{}",
                e, TRY_AGAIN_MSG
            );
            None
        }
    }
}

fn main() {
    use std::io::prelude::*;
    let mut sentences: HashMap<String, Sentence> = HashMap::new();
    if let Some(out_file_unbuffered) = create_output_file() {
        let mut out_file = std::io::BufWriter::new(out_file_unbuffered);
        if let Some(input_path) = get_input_path() {
            for result_dir_entry in
                std::fs::read_dir(input_path).expect("Listing the contents of input/")
            {
                match open_input_file(result_dir_entry) {
                    Some((input_file_name_no_extension, input_file_unbuffered)) => {
                        let mut mode = Mode::Header;
                        let input_file = std::io::BufReader::new(input_file_unbuffered);
                        for line_result in input_file.lines() {
                            if let Ok(line) = line_result {
                                match mode {
                                    Mode::Header => {
                                        if line == "Page" {
                                            mode = Mode::TextBody;
                                        }
                                    }
                                    Mode::TextBody => {
                                        line.split('.').for_each(|sentence| {
                                            if sentence.len() > 40 {
                                                let reduced: String = sentence
                                                    .to_lowercase()
                                                    .chars()
                                                    .filter(|c| c.is_alphanumeric())
                                                    .collect();
                                                sentences
                                                    .entry(reduced)
                                                    .or_insert_with(|| Sentence {
                                                        text: String::from(sentence.trim()),
                                                        count: 0,
                                                        report: Vec::new(),
                                                    })
                                                    .push(&input_file_name_no_extension);
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    }
                    None => return,
                }
            }
            let mut sentences: Vec<_> = sentences.values().collect();
            sentences.sort_unstable_by_key(|sentence| -sentence.count);
            writeln!(&mut out_file, "Total sentences: {}", sentences.len())
                .expect("Writing the total number of sentences to report.txt");
            let nontrivial = |x: &&Sentence| !x.text.starts_with("Collapse Subdiscussion");
            for sentence in sentences.into_iter().filter(nontrivial) {
                writeln!(&mut out_file, "{}", sentence).expect("Writing a sentence to report.txt");
            }
        }
        out_file.flush().expect("Writing report.txt");
        println!("report.txt is ready.");
    }
}
