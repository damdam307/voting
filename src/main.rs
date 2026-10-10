mod ballot_generator;
mod voting_system;
mod blt_parsing;
mod defined_structs;

use blt_parsing::parse_blt;

fn main() {
    
    let election = parse_blt(Some("../votes.blt".to_string()));

    print!("{}",election.candidate_number);

    // let candidates:usize = 500;
    // let voters:usize = 30000;
    // let seats:usize = 10;

    

    // election.run_election();

    // if matches!(election.results.result, ResultState::EveryWinnerAtCutoff) {
    //     println!("Full election: {:?}", election.results.elected_candidates);
    // } else if matches!(election.results.result, ResultState::SomeElectedUnderCutoff) {
    //     println!(
    //         "Partial election: {:?}",
    //         election.results.elected_candidates
    //     );
    //     println!(
    //         "Remaining seats: {:?}",
    //         election.results.remaining_elected_candidates
    //     );
    // } else if matches!(election.results.result, ResultState::StoppedBecauseOfTie) {
    //     println!(
    //         "Fully elected: {:?}",
    //         election.results.elected_candidates
    //     );
    //     println!(
    //         "Would have been elected: {:?}",
    //         election.results.remaining_elected_candidates
    //     );
    //     println!(
    //         "Tied: {:?}",
    //         election.results.tied_at_interruption
    //     );
    // }
}
