use fraction::GenericFraction;

pub type FracBase = i128;
pub type Frac = GenericFraction<FracBase>;
pub type Ballot = Vec<Vec<usize>>;

pub struct ElectionData
{
    blt_file: Option<String>,
    candidate_number: usize,
    available_seats: usize,
    voters: usize,
    withdrawn_candidates: Option<Vec<usize>>,
    ballots: Vec<Ballot>,
    weights: Vec<Frac>,
    names: Option<Vec<String>>,
    title: Option<String>,
}

impl ElectionData
{
    pub fn new(
        blt_file: Option<String>,
        candidate_number: usize,
        available_seats: usize,
        voters: usize,
        withdrawn_candidates: Option<Vec<usize>>,
        ballots: Vec<Ballot>,
        weights: Vec<Frac>,
        names: Option<Vec<String>>,
        title: Option<String>,
    ) -> Self
    {
        return ElectionData {
            blt_file: blt_file,
            candidate_number: candidate_number,
            available_seats: available_seats,
            voters: voters,
            withdrawn_candidates: withdrawn_candidates,
            ballots: ballots,
            weights: weights,
            names: names,
            title: title,
        }
    }

    pub fn get_blt(&self) -> Option<&str> { self.blt_file.as_deref() }
    pub fn candidate_number(&self) -> usize { self.candidate_number }
    pub fn available_seats(&self) -> usize { self.available_seats }
    pub fn voters(&self) -> usize {self.voters}
    pub fn withdrawn_candidates(&self) -> &Option<Vec<usize>> { &self.withdrawn_candidates }
    pub fn ballots(&self) -> &Vec<Ballot> { &self.ballots }
    pub fn weights(&self) -> &Vec<Frac> { &self.weights }
    pub fn names(&self) -> &Option<Vec<String>> { &self.names }
    pub fn title(&self) -> Option<String> { self.title.clone() }

}

pub enum ResultState
{
    NotRunElection,
    EveryWinnerAtCutoff,
    SomeElectedUnderCutoff,
    StoppedBecauseOfTie,
    TooFewCandidatesAllElected,
    TooFewCandidatesSomeUnderCutoff,
}

pub struct ElectionResult
{
    result: ResultState,
    elected_candidates: Vec<usize>,
    remaining_elected_candidates: Option<Vec<usize>>,
    tied_at_interruption: Option<Vec<usize>>
}

impl ElectionResult {
    pub fn new(
        result: ResultState,
        elected_candidates: Vec<usize>,
        remaining_elected_candidates: Option<Vec<usize>>,
        tied_at_interruption: Option<Vec<usize>>
    ) -> Self
    {
        return ElectionResult {

            result: result,
            elected_candidates: elected_candidates,
            remaining_elected_candidates: remaining_elected_candidates,
            tied_at_interruption: tied_at_interruption
        }
    }

    pub fn get_result_state(&self) -> &ResultState {&self.result}
    pub fn elected_candidates(&self) -> &Vec<usize> {&self.elected_candidates}
    pub fn remaining_elected_candidates(&self) -> &Option<Vec<usize>> {&self.remaining_elected_candidates}
    pub fn tied_at_interruption(&self) -> &Option<Vec<usize>> {&self.tied_at_interruption}

}

use clap::Parser;
#[derive(Parser)]
pub struct Args 
{
    #[arg(short,long,exclusive(true))]
    pub input_blt: Option<String>,
    #[arg(short,long,required_unless_present="input_blt")]
    pub votes: Option<usize>,
    #[arg(short,long,required_unless_present="input_blt")]
    pub candidates: Option<usize>,
    #[arg(short,long,required_unless_present="input_blt")]
    pub seats: Option<usize>,
    #[arg(short,long,default_value_t = 0.0)]
    pub partial_ballot_probability: f64,
    #[arg(short,long,default_value_t = 0.0)]
    pub tied_candidates_probability: f64,
}