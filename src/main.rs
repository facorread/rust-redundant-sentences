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
}

impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\t{}", self.count, self.text)
    }
}

fn main() {
    let mut sentences: HashMap<String, Sentence> = HashMap::new();

    // Useful for debugging on vscode.
    let interactive_run = true;
    if interactive_run {
        'new_page_of_text: loop {
            println!("Paste a page of text here.\nEnter EOF when you are done with the page, or Ctrl + C to close this program:");
            let mut mode = Mode::Header;
            use std::io::BufRead;
            let stdin_raw = std::io::stdin();
            let stdin = stdin_raw.lock();
            for line_result in stdin.lines() {
                if let Ok(line) = line_result {
                    match mode {
                        Mode::Header => match line.as_str() {
                            "Page" => mode = Mode::TextBody,
                            "EOF" => break 'new_page_of_text,
                            _ => {
                                println!("{}", line);
                            }
                        },
                        Mode::TextBody => {
                            if line == "EOF" {
                                continue 'new_page_of_text;
                            }
                            line.split('.').for_each(|sentence| {
                                if sentence.len() > 40 {
                                    sentences
                                        .entry(String::from(sentence))
                                        .or_insert_with_key(|sentence| Sentence {
                                            text: sentence.clone(),
                                            count: 0,
                                        })
                                        .count += 1;
                                }
                            });
                        }
                    }
                }
            }
        }
    }
    let mut sentences: Vec<_> = sentences.values().collect();
    sentences.sort_unstable_by_key(|sentence| -sentence.count);
    println!("Total sentences: {}", sentences.len());
    let nontrivial = |x: &&Sentence| !x.text.starts_with("Collapse Subdiscussion");
    let repeated = |x: &&Sentence| nontrivial(x) && x.count > 1;
    if sentences.iter().any(repeated) {
        for sentence in sentences.into_iter().filter(repeated) {
            println!("{}", sentence);
        }
    } else {
        for sentence in sentences.into_iter().filter(nontrivial) {
            println!("{}", sentence);
        }
    }
}
