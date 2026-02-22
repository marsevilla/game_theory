#[derive(PartialEq, Debug)]
pub enum Action {
    Cooperate,
    Defect,
}

pub trait Strategy {
    fn name(&self) -> &str;
    fn next_move(&mut self, opponent_history: &[Action]) -> Action;
}