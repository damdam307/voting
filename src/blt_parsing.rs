use std::io::{BufRead, BufReader};

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

enum ReadingState 
{
    FirstLine,
    WithdrawnCandidates,
    Ballots,
    CandidateNames,
    Title,
    End,
}

struct ElectionData
{
    candidate_number: i32,
    available_seats: i32,
}

fn atoi(s: &str) -> i32
{
    if s == ""
    {
        panic!("Invalid syntax in input file: expected number, got empty string");
    }

    match s.parse::<i32>() {
        Ok(number) => return number,
        Err(e) => panic!("Invalid syntax in input file: expected number, got non numeric string"),
    }
}

fn parse_blt(file_name: Option<String>) -> SingleTransferableVote
{
    let mut candidate_number = 0;
    let mut available_seats = 0;

    let mut withdrawn_candidates: Vec<i32> = Vec::new();
    
    let mut ballots: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut weights: Vec<i32> = Vec::new();

    let mut names: Vec<String> = vec::new();
    let mut title: String = "";

    let mut read_state: ReadingState = FirstLine;
    for line in file_name.open().lines()
    {
        if line.strip()[0]=='#' 
        {
            continue;
        }

        if read_state == WithdrawnCandidates && line.strip()[0]!='-' 
        {
            read_state = Ballots;
        }

        match read_state
        {
            FirstLine => {
                let bits: Vec<&str> = line.split('#')[0]
                                          .strip()
                                          .split(' ')
                                          .filter(|s| !s.is_empty()).collect();
                if bits.len() < 2 || (bits.len() > 2 && bits[2][0]!='#')
                {
                    panic!("Invalid syntax in input file: in first line, expected two numbers and an optional comment");
                } 
                
                candidate_number = atoi(bits[0]);
                available_seats = atoi(bits[1]);
                read_state = WithdrawnCandidates;
            },
            WithdrawnCandidates => {
                let bits: Vec<&str> = line.replace('-',' ')
                                          .strip()
                                          .split('#')[0]
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
                read_state = Ballots;
            },
            Ballots => {
                let bits: Vec<&str> = line.split('#')[0]
                                          .strip()
                                          .split(' ')
                                          .filter(|s| !s.is_empty())
                                          .collect();
                if bits[0]=="0"
                {
                    read_state = CandidateNames;
                    continue;
                }
                let mut finished_ballot:bool = 0;
                let mut read_weight:bool = 0;
                let bal: Vec<Vec<i32>> = Vec::new(); 
                for text in bits
                {
                    if exit
                    {
                        panic!("Invalid syntax in input file: in ballots, final '0's can only be followed by comments");
                    }
                    if text == "0"
                    {
                        exit = 1;
                        continue;
                    }
                    if !read_weight
                    {
                        weights.push(atoi(text));
                        read_weight = 1;
                        continue;
                    }
                    
                    let mut pref: Vec<i32> = Vec::new();
                    
                    if(text=="-")
                    {
                        bal.push(pref);
                        continue;
                    }
                    
                    let paired = text.split('=');
                    for ind in paired
                    {
                        let n = atoi(text);
                        if n>candidate_number
                        {
                            panic!("Invalid data in input file: in a ballot, one candidate has index bigger than the number of candidates in the race");
                        }
                        pref.push(n);
                    }
                    bal.push();
                }
                if !exit 
                {
                    panic!("Invalid syntax in input file: all ballots must end with a '0'");
                }
                ballots.push(bal);
            },
            CandidateNames => {
                let bits: Vec<&str> = line.split('#')[0]
                                          .strip()
                                          .split('"')
                                          .collect();
                if bits.len() != 3 || bits[0] != "" || bits[2] != ""
                {
                    panic!("Invalid syntax in input file: all candidate names should be fromatted as \"Name\"");
                }
                names.push(bits[1].to_string());
                if bits.len() == candidate_number
                {
                    read_state = Title;
                }
            },
            Title => {
                let bits: Vec<&str> = line.split('#')[0]
                                          .strip()
                                          .split('"')
                                          .collect();
                if bits.len() != 3 || bits[0] != "" || bits[2] != ""
                {
                    panic!("Invalid syntax in input file: the title should be fromatted as \"Title\"");
                }
                title = bits[1].to_string();
                read_state = End;
            }
            End => {
                panic!("Invalid syntax in input file: all lines after the title should be comments");
            }
        }

        if read_state == FirstLine
        {
            panic!("Unexpected end of input: input file is empty or consists only of comments");
        }
        if read_state == WithdrawnCandidates || read_state == Ballots
        {
            panic!("Unexpected end of input: the ballot section must end with a line containing only '0'");
        }
        if read_state == CandidateNames
        {
            panic!("Unexpected end of input: there must be as many candidate names as there are candidates");
        }


    }
}