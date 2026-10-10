use crate::defined_structs::*;
use rand::prelude::*;


/// Generates a ballot in the following way:
/// A vector with all the indices of the electable candidates is shuffled
/// At each step, with probability `tie_probability` the candidate i+1 has the 
/// same preference rank as candidate i, otherwise it has rank of one more. Lower rank
/// means that candidate is preferred over another, the first processed candidate has a rank of 0.
/// At each step, with probability `incomplete_probability` the process stops and all unprocessed
/// candidates get treated as unranked by that ballot.
/// # Arguments
/// * `candidate_number` - The number of electable candidates
/// * `tie_probability` - The probability that two adjacent candidates have the same final rank
/// * `incomplete_probability` - The probability of stopping at each step 
/// # Returns
/// A ballot: a double vector of integers. The vector at index i contains all candidates with 
/// preference rank i
fn create_ballot(candidate_number: usize, tie_probability: f64, incomplete_probability: f64) -> Ballot
{
    let mut rng = rand::rng();
    
    let mut starting_order: Vec<usize> = (0..candidate_number).collect();
    let mut preferences: Ballot = Vec::new();
    
    starting_order.shuffle(&mut rng);
    
    let mut add_new_el: bool = true;
    for candidate in starting_order 
    {
        if add_new_el
        {
            preferences.push(Vec::new());
        }

        preferences.last_mut().unwrap().push(candidate);
        add_new_el = rng.random_bool(1.0-tie_probability);
        if rng.random_bool(incomplete_probability)
        {
            break;
        }
    }
    return preferences;
}

/// Randomly generates N ballots
/// # Arguments
/// * `voter_number` - The number of ballots to generate
/// * `candidate_number` - The number of electable candidates
/// * `tie_probability` - The probability that two candidates have the same rank (see create_ballot)
/// * `incomplete_probability` - the probability that not all candidates have a rank (see create_ballot)
/// # Returns
/// A vector containing N ballots
pub fn create_n_ballots(voter_number: usize, candidate_number: usize, tie_probability: f64, incomplete_probability: f64) -> Vec<Ballot>
{
    let mut ballots: Vec<Ballot> = Vec::new();
    
    for _i in 0..voter_number
    {
        ballots.push(create_ballot(candidate_number, tie_probability, incomplete_probability));
    }
    return ballots;
}