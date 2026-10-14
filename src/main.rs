mod ballot_generator;
mod voting_system;
mod defined_structs;
mod blt_parsing;

use clap::Parser;
use ballot_generator::create_n_ballots;
use voting_system::SingleTransferableVote;
use defined_structs::*;
use blt_parsing::parse_blt;


fn main() {

    let args = Args::parse();

    let mut election:SingleTransferableVote;
    if args.input_blt.is_none()
    {
        election = SingleTransferableVote::new(
            ElectionData::new(
                None, 
                args.candidates.unwrap_or(0), 
                args.seats.unwrap_or(0), 
                args.votes.unwrap_or(0), 
                None, 
                create_n_ballots(
                args.votes.unwrap_or(0), 
                args.candidates.unwrap_or(0), 
                args.tied_candidates_probability, 
                args.partial_ballot_probability,
                ), 
                vec![Frac::new(1,0);args.votes.unwrap_or(0)], 
                None, 
                None)
        );
    }
    else 
    {
        election = SingleTransferableVote::new(parse_blt(args.input_blt));
    }

    election.run_election();

    match election.results.get_result_state()
    {
        ResultState::EveryWinnerAtCutoff => {
            println!("Full election: {:?}", election.results.elected_candidates());
        }
        ResultState::SomeElectedUnderCutoff => {
            println!(
                "Partial election: {:?}",
                election.results.elected_candidates()
            );
            println!(
                "Remaining seats: {:?}",
                election.results.remaining_elected_candidates().clone().unwrap()
            );
        }
        ResultState::StoppedBecauseOfTie => {
            println!(
                "Fully elected: {:?}",
                election.results.elected_candidates()
            );
            println!(
                "Would have been elected: {:?}",
                election.results.remaining_elected_candidates().clone().unwrap()
            );
            println!(
                "Tied: {:?}",
                election.results.tied_at_interruption().clone().unwrap()
            );
        }
        ResultState::TooFewCandidatesAllElected => {
            println!("Too few candidates:\nFull election: {:?}", election.results.elected_candidates());
        }
        ResultState::TooFewCandidatesSomeUnderCutoff => {
            println!(
                "Too few candidates:\nPartial election: {:?}",
                election.results.elected_candidates()
            );
            println!(
                "Remaining seats: {:?}",
                election.results.remaining_elected_candidates().clone().unwrap()
            );
        }
        _ => {}
    }
}
