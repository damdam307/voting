
pub struct ElectionData
{
    blt_file: Option<String>,
    candidate_number: i32,
    available_seats: i32,
    withdrawn_candidates: Vec<i32>,
    ballots: Vec<Vec<Vec<i32>>>,
    weights: Vec<i32>,
    names: Option<Vec<String>>,
    title: Option<String>,
}

impl ElectionData
{
    pub fn new(
        blt_file: Option<String>,
        candidate_number: i32,
        available_seats: i32,
        withdrawn_candidates: Vec<i32>,
        ballots: Vec<Vec<Vec<i32>>>,
        weights: Vec<i32>,
        names: Option<Vec<String>>,
        title: Option<String>,
    ) -> Self
    {
        return ElectionData {
            blt_file: blt_file,
            candidate_number: candidate_number,
            available_seats: available_seats,
            withdrawn_candidates: withdrawn_candidates,
            ballots: ballots,
            weights: weights,
            names: names,
            title: title,
        }
    }

    pub fn get_blt(&self) -> Option<&str> { self.blt_file.as_deref() }
    pub fn candidate_number(&self) -> i32 { self.candidate_number }
    pub fn available_seats(&self) -> i32 { self.available_seats }
    pub fn withdrawn_candidates(&self) -> Vec<i32> { self.withdrawn_candidates.clone() }
    pub fn ballots(&self) -> Vec<Vec<Vec<i32>>> { self.ballots.clone() }
    pub fn weights(&self) -> Vec<i32> { self.weights.clone() }
    pub fn names(&self) -> Option<Vec<String>> { self.names.clone() }
    pub fn title(&self) -> Option<String> { self.title.clone() }

}
