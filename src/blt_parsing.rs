#[path = "defined_structs.rs"]
mod defined_structs;
use std::default;
use std::io::{BufRead, BufReader, Read};
use std::fs::File;
use defined_structs::ElectionData;
trait DefaultToStdin 
{
    fn open(&self) -> Box<dyn BufRead>;
}

impl DefaultToStdin for Option<String>
{
    fn open(&self) -> Box<dyn BufRead> 
    {
        match self 
        {
            None => Box::new(BufReader::new(std::io::stdin())),
            Some(filename) => {
                let file = File::open(filename)
                             .expect("Cannot open source file");
                Box::new(BufReader::new(file))
            }
        }
    }
}

#[derive(PartialEq)]
enum ReadingState 
{
    FirstLine,
    WithdrawnCandidates,
    Ballots,
    CandidateNames,
    Title,
    End,
}

fn atoi(s: &str) -> i32
{
    if s == ""
    {
        panic!("Invalid syntax in input file: expected number, got empty string");
    }

    match s.parse::<i32>() {
        Ok(number) => return number,
        Err(_e) => panic!("Invalid syntax in input file: expected number, got non numeric string"),
    }
}

pub fn parse_blt(file_name: Option<String>) -> ElectionData
{
    let mut candidate_number = 0;
    let mut available_seats = 0;

    let mut withdrawn_candidates: Vec<i32> = Vec::new();
    
    let mut ballots: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut weights: Vec<i32> = Vec::new();

    let mut names: Vec<String> = Vec::new();
    let mut title: String = "".to_string();

    let mut read_state: ReadingState = ReadingState::FirstLine;
    let mut blt_file = file_name.open();

    for line_it in (&mut blt_file).lines()
    {
        let line:String;

        match line_it
        {
            Err(_e) => {panic!("iterato error");}
            Ok(line1) => {
                line = line1;
            } 
        }

        // skip comments
        if line.trim().chars().nth(0)==Some('#') 
        {
            continue;
        }

        if read_state == ReadingState::WithdrawnCandidates && line.trim().chars().nth(0)!=Some('-') 
        {
            read_state = ReadingState::Ballots;
        }

        match read_state
        {
            ReadingState::FirstLine => {
                let bits: Vec<&str> = line.split('#')
                                          .next()
                                          .unwrap_or("")
                                          .trim()
                                          .split(' ')
                                          .filter(|s| !s.is_empty())
                                          .collect();
                if bits.len() < 2 || (bits.len() > 2 && bits[2].chars().nth(0)!=Some('#'))
                {
                    panic!("Invalid syntax in input file: in first line, expected two numbers and an optional comment");
                } 
                
                candidate_number = atoi(bits[0]);
                available_seats = atoi(bits[1]);
                read_state = ReadingState::WithdrawnCandidates;
            },
            ReadingState::WithdrawnCandidates => {
                let normalized_line = line.replace('-', " ");
                let bits: Vec<&str> = normalized_line
                    .trim()
                    .split('#')
                    .next()
                    .unwrap_or("")
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .collect();

                for text in bits
                {
                    let n = atoi(text);
                    if n>candidate_number
                    {
                        panic!("Invalid data in input file: in withdrawn candidates line, one candidate has index bigger than the number of candidates in the race");
                    }
                    withdrawn_candidates.push(n);
                }
                read_state = ReadingState::Ballots;
            },
            ReadingState::Ballots => {
                let bits: Vec<&str> = line.split('#')
                                          .next()
                                          .unwrap_or("")
                                          .trim()
                                          .split(' ')
                                          .filter(|s| !s.is_empty())
                                          .collect();
                if bits[0]=="0"
                {
                    read_state = ReadingState::CandidateNames;
                    continue;
                }
                let mut finished_ballot:bool = false;
                let mut read_weight:bool = false;
                let mut bal: Vec<Vec<i32>> = Vec::new(); 
                for text in bits
                {
                    if finished_ballot
                    {
                        panic!("Invalid syntax in input file: in ballots, final '0's can only be followed by comments");
                    }
                    if text == "0"
                    {
                        finished_ballot = true;
                        continue;
                    }
                    if !read_weight
                    {
                        weights.push(atoi(text));
                        read_weight = true;
                        continue;
                    }
                    
                    let mut pref: Vec<i32> = Vec::new();
                    
                    if text=="-" 
                    {
                        bal.push(pref);
                        continue;
                    }
                    
                    let paired = text.split('=');
                    for ind in paired
                    {
                        let n = atoi(ind);
                        if n>candidate_number
                        {
                            panic!("Invalid data in input file: in a ballot, one candidate has index bigger than the number of candidates in the race");
                        }
                        pref.push(n);
                    }
                    bal.push(pref);
                }
                if !finished_ballot 
                {
                    panic!("Invalid syntax in input file: all ballots must end with a '0'");
                }
                ballots.push(bal);
            },
            ReadingState::CandidateNames => {
                let bits: Vec<&str> = line.split('#')
                                          .next()
                                          .unwrap_or("")
                                          .trim()
                                          .split('"')
                                          .collect();
                if bits.len() != 3 || bits[0] != "" || bits[2] != ""
                {
                    panic!("Invalid syntax in input file: all candidate names should be fromatted as \"Name\"");
                }
                names.push(bits[1].to_string());
                if names.len() == candidate_number as usize
                {
                    read_state = ReadingState::Title;
                }
            },
            ReadingState::Title => {
                let bits: Vec<&str> = line.split('#')
                                          .next()
                                          .unwrap_or("")
                                          .trim()
                                          .split('"')
                                          .collect();
                if bits.len() != 3 || bits[0] != "" || bits[2] != ""
                {
                    panic!("Invalid syntax in input file: the title should be fromatted as \"Title\"");
                }
                title = bits[1].to_string();
                read_state = ReadingState::End;
            }
            ReadingState::End => {
                panic!("Invalid syntax in input file: all lines after the title should be comments");
            }
        }

    }

    match read_state
    {
        ReadingState::FirstLine => {panic!("Unexpected end of input: input file is empty or consists only of comments");}
        ReadingState::WithdrawnCandidates => {panic!("Unexpected end of input: the ballot section must end with a line containing only '0'");}
        ReadingState::CandidateNames => {panic!("Unexpected end of input: there must be as many candidate names as there are candidates");}
        _ => {}
    }

    let mut str:String = String::new();
    let _ = file_name.open().read_to_string(&mut str);

    return ElectionData::new(
        Some(str),
        candidate_number,
        available_seats,
        withdrawn_candidates,
        ballots,
        weights,
        Some(names),
        Some(title)
    );
}