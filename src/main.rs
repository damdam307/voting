mod ballot_generator;
mod voting_system;

use ballot_generator::create_n_ballots;
use voting_system::SingleTrasferableVote;
use voting_system::ResultState;

fn main() {
    let candidates:usize = 500;
    let voters:usize = 30000;
    let seats:usize = 10;

    let mut election:SingleTrasferableVote = SingleTrasferableVote::new(
        create_n_ballots(
            voters, 
            candidates, 
            0.1, 
            0.0
        ),
        candidates,
        voters,
        seats
    );

    election.run_election();

    if matches!(election.results.result, ResultState::EveryWinnerAtCutoff) {
        println!("Full election: {:?}", election.results.elected_candidates);
    } else if matches!(election.results.result, ResultState::SomeElectedUnderCutoff) {
        println!(
            "Partial election: {:?}",
            election.results.elected_candidates
        );
        println!(
            "Remaining seats: {:?}",
            election.results.remaining_elected_candidates
        );
    } else if matches!(election.results.result, ResultState::StoppedBecauseOfTie) {
        println!(
            "Fully elected: {:?}",
            election.results.elected_candidates
        );
        println!(
            "Would have been elected: {:?}",
            election.results.remaining_elected_candidates
        );
        println!(
            "Tied: {:?}",
            election.results.tied_at_interruption
        );
    }
}
