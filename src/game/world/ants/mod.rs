mod ant;
mod plans;
mod queen;
mod worker;

use super::*;
use crate::game::*;
use crate::window::*;
use cgmath::*;

pub use ant::Ant;
pub use plans::AntPlan;
pub use queen::QueenPlan;
pub use worker::WorkerPlan;
pub type Worker = Ant<WorkerPlan>;
pub type Queen = Ant<QueenPlan>;
