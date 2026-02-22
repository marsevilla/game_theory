use crate::{Strategy, strategy::Action};

pub struct AlwaysCooperate;

impl AlwaysCooperate {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for AlwaysCooperate {

}
