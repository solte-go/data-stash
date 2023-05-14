use serde::{Deserialize, Serialize};
use crate::domain::time::Time;
use derive_more::Constructor;


#[derive(Constructor, Clone, Debug, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
    fn into_inner(self) -> Time {
        self.0
    }
}