use crate::bitset::BitSetInt;

use crate::human_calcs::basic::{
    ClaimingCandidate, PointingCandidate, SingleCandidate, SinglePossibility,
};
use crate::human_calcs::fish::basicfish::BasicFish;
use crate::human_calcs::fish::finned::FinnedFish;
use crate::human_calcs::guess::Guess;
use crate::human_calcs::tuples::*;
use crate::human_calcs::TechStruct;

use std::fmt::{Display, Formatter};

/// Enum for different ways to solve the puzzle.  Guesses is done by brute force solving.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Technique {
    SingleCandidate,
    SinglePossibility,
    Pointing,
    Claiming,
    NakedDouble,
    NakedTriple,
    NakedQuad,
    NakedTuple(usize),
    HiddenDouble,
    HiddenTriple,
    HiddenQuad,
    XWing,
    Swordfish,
    Jellyfish,
    FinnedXWing,
    FinnedSwordfish,
    FinnedJellyfish,
    HiddenTuple(usize),
    FishN(usize),
    FinnedN(usize),
    Guess,
}

/// While the algorithms for solving should work for 16x16 puzzles, this foundation of selecting the next difficult
/// technique is not implemented
impl Technique {
    pub fn solver<V: BitSetInt>(&self) -> Box<dyn TechStruct<SquareValue = V>> {
        match self {
            Technique::SingleCandidate => Box::new(SingleCandidate::new()),
            Technique::SinglePossibility => Box::new(SinglePossibility::new()),
            Technique::Pointing => Box::new(PointingCandidate::new()),
            Technique::Claiming => Box::new(ClaimingCandidate::new()),
            Technique::NakedDouble => Box::new(NakedTuple::new(2)),
            Technique::NakedTriple => Box::new(NakedTuple::new(3)),
            Technique::NakedQuad => Box::new(NakedTuple::new(4)),
            Technique::HiddenDouble => Box::new(HiddenTuple::new(2)),
            Technique::HiddenTriple => Box::new(HiddenTuple::new(3)),
            Technique::HiddenQuad => Box::new(HiddenTuple::new(4)),
            Technique::XWing => Box::new(BasicFish::new(2)),
            Technique::Swordfish => Box::new(BasicFish::new(3)),
            Technique::Jellyfish => Box::new(BasicFish::new(4)),
            Technique::FinnedXWing => Box::new(FinnedFish::new(2)),
            Technique::FinnedSwordfish => Box::new(FinnedFish::new(3)),
            Technique::FinnedJellyfish => Box::new(FinnedFish::new(4)),
            Technique::NakedTuple(n) => Box::new(NakedTuple::new(*n)),
            Technique::HiddenTuple(n) => Box::new(HiddenTuple::new(*n)),
            Technique::FishN(n) => Box::new(BasicFish::new(*n)),
            Technique::FinnedN(n) => Box::new(FinnedFish::new(*n)),
            Technique::Guess => Box::new(Guess::new()),
        }
    }

    // todo expand this to allow more than 3x3
    pub(crate) fn iterator<'a>() -> impl Iterator<Item = &'a Technique> {
        use Technique::*;
        [
            SingleCandidate,
            SinglePossibility,
            Pointing,
            Claiming,
            NakedDouble,
            NakedTriple,
            NakedQuad,
            HiddenDouble,
            HiddenTriple,
            HiddenQuad,
            XWing,
            Swordfish,
            Jellyfish,
            FinnedXWing,
            FinnedSwordfish,
            FinnedJellyfish,
            Guess,
        ]
        .iter()
    }

    pub fn default_difficulty(&self) -> f32 {
        match self {
            Technique::SingleCandidate => 0.2,
            Technique::SinglePossibility => 1.3,
            Technique::Pointing => 1.8,
            Technique::Claiming => 2.1,
            Technique::NakedDouble => 1.0,
            Technique::NakedTriple => 1.7,
            Technique::NakedQuad => 2.2,
            Technique::HiddenDouble => 2.1,
            Technique::HiddenTriple => 2.9,
            Technique::HiddenQuad => 3.7,
            Technique::XWing => 3.4,
            Technique::Swordfish => 4.0,
            Technique::Jellyfish => 5.0,
            Technique::FinnedXWing => 4.1,
            Technique::FinnedSwordfish => 4.9,
            Technique::FinnedJellyfish => 5.9,
            Technique::NakedTuple(n) => 2.2 + (0.5 * (*n as f32 - 4.0)),
            Technique::HiddenTuple(n) => 3.7 + (0.5 * (*n as f32 - 4.0)),
            Technique::FishN(n) => 5.0 + (0.5 * (*n as f32 - 4.0)),
            Technique::FinnedN(n) => 5.9 + (0.5 * (*n as f32 - 4.0)),
            Technique::Guess => 8.0,
        }
    }
}

impl Display for Technique {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Technique::Guess => write!(f, "Guess"),
            Technique::SingleCandidate => write!(f, "Single Candidates"),
            Technique::SinglePossibility => write!(f, "Single Possibilities"),
            Technique::Pointing => write!(f, "Pointing Candidates"),
            Technique::Claiming => write!(f, "Claiming Candidates"),
            Technique::NakedDouble => write!(f, "Naked Double"),
            Technique::NakedTriple => write!(f, "Naked Triple"),
            Technique::NakedQuad => write!(f, "Naked Quadruple"),
            Technique::HiddenDouble => write!(f, "Hidden Double"),
            Technique::HiddenTriple => write!(f, "Hidden Triple"),
            Technique::HiddenQuad => write!(f, "Hidden Quadruple"),
            Technique::XWing => write!(f, "X-Wing"),
            Technique::Swordfish => write!(f, "Swordfish"),
            Technique::Jellyfish => write!(f, "Jellyfish"),
            Technique::FinnedXWing => write!(f, "Finned X-Wing"),
            Technique::FinnedSwordfish => write!(f, "Finned Swordfish"),
            Technique::FinnedJellyfish => write!(f, "Finned Jellyfish"),
            Technique::NakedTuple(n) => write!(f, "Naked Tuple size: {n}"),
            Technique::HiddenTuple(n) => write!(f, "Hidden Tuple size: {n}"),
            Technique::FishN(n) => write!(f, "Fish size: {n}"),
            Technique::FinnedN(n) => write!(f, "Finned Fish size: {n}"),
        }
    }
}
