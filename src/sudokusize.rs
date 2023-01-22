use crate::errors::*;

pub(crate) type Res<K> = Result<K, SudError>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum SudokuSize {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl SudokuSize {
    #[inline]
    pub(crate) fn size(&self) -> usize {
        use SudokuSize::*;
        match self {
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
        }
    }

    pub(crate) fn size_from_input_length(len: usize) -> Res<SudokuSize> {
        let sq = |x: usize| x.pow(2).pow(2);

        let size = match len {
            x if x == sq(3) => SudokuSize::Three,
            x if x == sq(4) => SudokuSize::Four,
            x if x == sq(5) => SudokuSize::Five,
            x if x == sq(6) => SudokuSize::Six,
            x if x == sq(7) => SudokuSize::Seven,
            x if x == sq(8) => SudokuSize::Eight,
            x if x == sq(9) => SudokuSize::Nine,
            x if x == sq(10) => SudokuSize::Ten,
            x => return Err(SudError::InputLengthError(x)),
        };
        Ok(size)
    }

    #[inline]
    pub(crate) fn house_size(&self) -> usize {
        self.size() * self.size()
    }

    #[inline]
    pub(crate) fn total(&self) -> usize {
        self.house_size() * self.house_size()
    }
}

impl Default for SudokuSize {
    fn default() -> Self {
        SudokuSize::Three
    }
}
