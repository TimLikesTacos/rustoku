
use crate::move_change::Move;

pub(crate) trait Report {
    fn report(&self) -> String;
}

impl Report for Vec<Move> {
    fn report(&self) -> String {
        let mut string = format!("Report of {} moves: \n", self.len());
        let mut difficulty = 0f32;
        for each in self {
            string += &each.report();
            difficulty += each.technique().default_difficulty();
        }
        string += &format!("\nTotal difficulty rating: {} ", difficulty);
        string
    }
}
