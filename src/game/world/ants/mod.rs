mod ant;
mod plans;
mod queen;
mod worker;

use super::*;
use crate::game::*;
use crate::window::*;
use cgmath::*;
use plans::*;

pub use ant::Ant;
pub type Worker = Ant<worker::WorkerPlan>;
pub type Queen = Ant<queen::QueenPlan>;
