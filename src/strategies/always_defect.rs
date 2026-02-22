use crate::{Strategy, strategy::Action};

pub struct AlwaysDefect;

impl AlwaysDefect {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for AlwaysDefect {

}
