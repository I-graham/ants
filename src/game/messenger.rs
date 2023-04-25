use cgmath::*;
use std::{
    ops::Add,
    time::{Duration, Instant},
};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumDiscriminants};

#[derive(Clone, Copy, EnumDiscriminants, EnumCount)]
#[strum_discriminants(name(MessageTypes))]
pub enum Message {
    Pheromones
}

type Buckets = [Vec<Dispatch>; Message::COUNT];

pub struct Messenger {
    buckets: Buckets,
}

impl Messenger {
    pub fn new() -> Self {
        Self {
            buckets: Buckets::default(),
        }
    }

    pub fn receive<'a>(&'a self, types: &'a [MessageTypes]) -> impl Iterator<Item = Message> + 'a {
        types
            .iter()
            .flat_map(|&ty| self.buckets[ty as usize].iter())
            .map(|dispatch| dispatch.message)
    }

    pub fn dispatch(&mut self, message: Message, delay: f32) {
        let ty = MessageTypes::from(message);
        self.buckets[ty as usize].push(Dispatch {
            message,
            time: Instant::now().add(Duration::from_secs_f32(delay)),
        })
    }

    pub fn cleanup(&mut self) {
        let now = Instant::now();
        for bucket in &mut self.buckets {
            bucket.retain(|dispatch| now < dispatch.time)
        }
    }
}

struct Dispatch {
    message: Message,
    time: Instant,
}
