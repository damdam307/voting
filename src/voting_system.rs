
use std::collections::HashSet;
use fraction::GenericFraction;
type Frac = GenericFraction<i32>;
type Ballot = Vec<Vec<usize>>;

pub enum ResultState
{
    NotRunElection,
    EveryWinnerAtCutoff,
    SomeElectedUnderCutoff,
    StoppedBecauseOfTie,
}

pub struct ElectionResult
{
    pub result: ResultState,
    pub elected_candidates: Vec<usize>,
    pub remaining_elected_candidates: Vec<usize>,
    pub tied_at_interruption: Vec<usize>
}

impl ElectionResult {
    pub fn new() -> Self
    {
        return ElectionResult {

            result: ResultState::NotRunElection,
            elected_candidates: Vec::new(),
            remaining_elected_candidates: Vec::new(),
            tied_at_interruption: Vec::new()
        }
    }
}

pub struct SingleTrasferableVote
{
    pub ballots: Vec<Vec<Vec<usize>>>, 
    pub number_of_candidates: usize,
    pub number_of_votes: usize,
    pub candidates_to_elect: usize,
    
    cutoff: GenericFraction<i32>,
    
    is_voted_by: Vec<HashSet<usize>>,
    is_voting_for: Vec<HashSet<usize>>,
    indices: Vec<usize>,
    is_out: Vec<bool>,
    weights: Vec<GenericFraction<i32>>,
    total_votes: Vec<GenericFraction<i32>>,
    remaining_candidates: usize,

    pub results: ElectionResult,
}

impl SingleTrasferableVote
{
    pub fn new(ballots: Vec<Ballot>,number_of_candidates: usize,number_of_votes: usize,candidates_to_elect:usize) -> Self
    {
        return SingleTrasferableVote
        {
            ballots: ballots,
            number_of_candidates: number_of_candidates,
            number_of_votes: number_of_votes,
            candidates_to_elect: candidates_to_elect,
            
            cutoff: Frac::new(0, 1),
            
            is_voted_by: Vec::new(),
            is_voting_for: Vec::new(),
            indices: Vec::new(),
            is_out: Vec::new(),
            weights: Vec::new(),
            total_votes: Vec::new(),
            remaining_candidates: number_of_candidates,
            
            results: ElectionResult::new(),
        }
    }

    fn remove_candidate(&mut self, candidate: usize, vote_decay_factor:Frac)
    {
        self.is_out[candidate] = true;
        self.total_votes[candidate] = if vote_decay_factor != Frac::new(1,1) {self.cutoff} else {Frac::new(0,1)};
        if vote_decay_factor == Frac::new(0,1)
        {
            return;
        }
        
        let voters: Vec<usize> = self.is_voted_by[candidate].iter().copied().collect();
        for voter in voters
        {
            self.is_voting_for[voter].remove(&candidate);
            if !self.is_voting_for[voter].is_empty()
            {
                let split:Frac = Frac::new(self.is_voting_for[voter].len() as i32,1);
                let profiting_candidates: Vec<usize> =
                    self.is_voting_for[voter].iter().copied().collect();
                for profiting_candidate in profiting_candidates
                {
                    self.total_votes[profiting_candidate]+=vote_decay_factor*self.weights[voter]/split;
                }
                let new_weight:Frac = vote_decay_factor*self.weights[voter]/split; 
                self.weights[voter]+=new_weight;
            }
            else
            {
                self.indices[voter]+=1;
                while self.indices[voter]<self.ballots[voter].len()
                {
                    let mut new_candidates: i32 = 0;
                    for possible_candidate in &self.ballots[voter][self.indices[voter]]
                    {
                        if !self.is_out[*possible_candidate]
                        {
                            new_candidates += 1;
                        }
                    }
                    if new_candidates == 0 {self.indices[voter]+=1;continue;}
                    self.weights[voter] *= vote_decay_factor;
                    self.weights[voter] /= new_candidates as f64;
                    
                    let next_candidates: Vec<usize> = self.ballots[voter][self.indices[voter]]
                        .iter()
                        .copied()
                        .filter(|possible_candidate| !self.is_out[*possible_candidate])
                        .collect();
                    for possible_candidate in next_candidates
                    {
                        self.is_voting_for[voter].insert(possible_candidate);
                        self.is_voted_by[possible_candidate].insert(voter);
                        self.total_votes[possible_candidate]+=self.weights[voter];
                    }
                    break;
                }
            }
        }
    }

    fn disqualify_candidate(&mut self, candidate: usize)
    {
        self.remaining_candidates -= 1;
        self.remove_candidate(candidate, Frac::new(1,1));

    }
    
    fn elect_candidate(&mut self, candidate: usize)
    {
        self.results.elected_candidates.push(candidate);
        let vote_decay_factor: Frac = (self.total_votes[candidate]-self.cutoff)/(self.total_votes[candidate]);
        self.remove_candidate(candidate, vote_decay_factor);
    }

    fn initialise_variables(&mut self)
    {
        self.is_out = vec![false;self.number_of_candidates];
        self.indices = vec![0;self.number_of_votes];
        self.weights = vec![Frac::new(1,1);self.number_of_votes];
        self.remaining_candidates = self.number_of_candidates;
        self.cutoff = Frac::new(self.number_of_votes as i32,self.candidates_to_elect as i32);

        self.is_voted_by = vec![HashSet::new();self.number_of_candidates];
        self.is_voting_for = vec![HashSet::new();self.number_of_votes];
        self.total_votes = vec![Frac::new(0,1);self.number_of_candidates];

        for i in 0..self.number_of_votes
        {
            let ballot = &self.ballots[i];
            let single_weight: Frac = Frac::new(1,ballot[0].len() as i32);
            self.weights[i]=single_weight;
            for candidate in &ballot[0]
            {
                self.is_voting_for[i].insert(*candidate);
                self.is_voted_by[*candidate].insert(i);
                self.total_votes[*candidate] += single_weight;
            }
        }
    }

    pub fn run_election(&mut self)
    {
        self.initialise_variables();
        
        while self.remaining_candidates>=self.candidates_to_elect && self.results.elected_candidates.len()<self.candidates_to_elect
        {
            let mut have_elected_someone = false;
            for candidate in 0..self.number_of_candidates
            {
                if self.is_out[candidate] 
                {
                    continue;
                }
                if self.total_votes[candidate]>= self.cutoff
                {
                    self.elect_candidate(candidate);       
                    have_elected_someone = true;
                    break;
                }
            }

            if have_elected_someone
            {
                continue;
            }

            let mut min: Frac = self.cutoff;
            let mut least_voted_candidates = Vec::new();
            for candidate in 0..self.number_of_candidates
            {
                if self.is_out[candidate] 
                {
                    continue;
                }   
                    
                if min == self.cutoff || self.total_votes[candidate] < min
                {
                    min = self.total_votes[candidate];
                    least_voted_candidates = vec![candidate];
                }
                else if min == self.total_votes[candidate]
                {
                    least_voted_candidates.push(candidate);
                }
            }

            if self.remaining_candidates-least_voted_candidates.len() < self.candidates_to_elect
            {
                if self.remaining_candidates==self.candidates_to_elect
                {
                    for candidate in least_voted_candidates
                    {
                        self.results.remaining_elected_candidates.push(candidate);
                    }
                    self.results.result=ResultState::SomeElectedUnderCutoff;
                    return;
                }
                else 
                {
                    for candidate in least_voted_candidates
                    {
                        self.results.tied_at_interruption.push(candidate);
                        self.is_out[candidate]=true;
                    } 
                    for candidate in 0..self.number_of_candidates
                    {
                        if !self.is_out[candidate]
                        {
                            self.results.remaining_elected_candidates.push(candidate);
                        }
                    }
                    self.results.result=ResultState::StoppedBecauseOfTie;
                    return;
                }
            }
            
            for candidate in least_voted_candidates
            {
                self.disqualify_candidate(candidate);
            }
        }

        self.results.result=ResultState::EveryWinnerAtCutoff;
        return;
    }
}