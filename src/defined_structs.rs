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
    withdrawn_candidates: Vec<usize>,
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
        withdrawn_candidates: Vec<usize>,
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
    pub fn withdrawn_candidates(&self) -> &Vec<usize> { &self.withdrawn_candidates }
    pub fn ballots(&self) -> &Vec<Ballot> { &self.ballots }
    pub fn weights(&self) -> &Vec<Frac> { &self.weights }
    pub fn names(&self) -> &Option<Vec<String>> { &self.names }
    pub fn title(&self) -> Option<String> { self.title.clone() }

}
