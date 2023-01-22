use crate::human_calcs::technique::Technique;

pub trait TechniqIndex {
    fn tech_index(&self, techs: &[Technique]) -> Option<usize>;
}

impl TechniqIndex for usize {
    fn tech_index(&self, techs: &[Technique]) -> Option<usize> {
        if *self < techs.len() {
            Some(*self)
        } else {
            None
        }
    }
}

impl TechniqIndex for Technique {
    fn tech_index(&self, techs: &[Technique]) -> Option<usize> {
        techs
            .binary_search_by(|a| {
                a.default_difficulty()
                    .partial_cmp(&self.default_difficulty())
                    .unwrap()
            })
            .ok()
    }
}
