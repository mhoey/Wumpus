use crate::util::Random;

use super::MAX_ROOMS;

#[derive(Copy, Clone, PartialEq)]
pub enum ActorType {
    You,
    Wumpus,
    Bat,
    Pit,
}

#[derive(Copy, Clone)]
pub struct Actor {
    pub actor_type: ActorType,
    pub room: u8,
}

pub fn place_actors(random: &impl Random) -> Vec<Actor> {
    let mut rooms:Vec<u8> = Vec::new();
    for rn in 1..MAX_ROOMS {
        rooms.push(rn);
    }

    rooms = random.shuffle( rooms);

    let mut actors: Vec<Actor> = Vec::new();

    actors.push(Actor {
        actor_type: ActorType::You,
        room: rooms[0],
    });

    actors.push(Actor {
        actor_type: ActorType::Pit,
        room: rooms[1],
    });

    actors.push(Actor {
        actor_type: ActorType::Pit,
        room: rooms[2],
    });

    actors.push(Actor {
        actor_type: ActorType::Bat,
        room: rooms[3],
    });
    actors.push(Actor {
        actor_type: ActorType::Bat,
        room: rooms[4],
    });

    actors.push(Actor {
        actor_type: ActorType::Wumpus,
        room: rooms[5],
    });

    return actors;
}    
