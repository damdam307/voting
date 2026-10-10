mod ballot_generator;
mod voting_system;
mod defined_structs;
mod blt_parsing;

use ballot_generator::create_n_ballots;
use voting_system::SingleTransferableVote;
use defined_structs::*;

fn main() {
    let candidates:usize = 10;
    let voters:usize = 100;
    let seats:usize = 3;

    let mut election:SingleTransferableVote = SingleTransferableVote::new(
        ElectionData::new(
            None, 
            candidates, 
            seats, 
            voters, 
            None, 
            create_n_ballots(
            voters, 
            candidates, 
            0.1, 
            0.1
            ), 
            vec![Frac::new(1,0);voters], 
            None, 
            None)
    );

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
