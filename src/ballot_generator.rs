use rand::prelude::*;

fn create_ballot(candidate_number: usize, tie_probability: f64, incomplete_probability: f64) -> Vec<Vec<usize>>
{
    let mut starting_order: Vec<usize> = (0..candidate_number).collect();
    let mut rng = rand::rng();
    starting_order.shuffle(&mut rng);

    let mut preferences: Vec<Vec<usize>> = Vec::new();
    let mut add_new_el: bool = true;

    for candidate in starting_order 
    {
        if preferences.is_empty() || add_new_el
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

pub fn create_n_ballots(number: usize, candidate_number: usize, tie_probability: f64, incomplete_probability: f64) -> Vec<Vec<Vec<usize>>>
{
    let mut ballots: Vec<Vec<Vec<usize>>> = Vec::new();
    for _i in 0..number
    {
        ballots.push(create_ballot(candidate_number, tie_probability, incomplete_probability));
    }
    return ballots;
}