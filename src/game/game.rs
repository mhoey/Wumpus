use std::convert::TryInto;
use std::convert::TryFrom;

use crate::util::Random;
use super::gameconstants::*;
use super::Actor;
use super::place_actors;
use super::ActorType;

pub struct Game {
    state:GameStates,
    actors:Vec<Actor>,
    bumped_wumpus:bool,
    wumpus_moves:bool,
    super_bat_move:bool,
    pub number_of_arrows:u8,
    pub number_of_arrow_rooms: u8,
    pub current_arrow_room_count: u8,
    pub rooms_to_shoot_arrows_in: Vec<u8>,
    game_over_reason: GameOverReason
}

impl Game {
    pub fn new() -> Self { 
        Game {
            state: GameStates::GameStart,
            actors: Vec::new(),
            bumped_wumpus: false,
            wumpus_moves: false,
            super_bat_move: false,
            number_of_arrows: 5,
            number_of_arrow_rooms: 0,
            current_arrow_room_count: 0,
            rooms_to_shoot_arrows_in: Vec::new(),
            game_over_reason: GameOverReason::NotDeadYet,
        }
    }

    pub fn clear_state(&mut self) {
        self.state = GameStates::GameStart;
        self.bumped_wumpus = false;
        self.wumpus_moves = false;
        self.super_bat_move = false;
        self.number_of_arrows = 5;
        self.number_of_arrow_rooms = 0;
        self.current_arrow_room_count = 0;
        self.rooms_to_shoot_arrows_in = Vec::new();
        self.game_over_reason = GameOverReason::NotDeadYet;
    }

    pub fn clear_actors(&mut self) {
        self.actors = Vec::new();
    }

    pub fn get_game_state(&self) -> GameStates {
        self.state
    }


    pub fn get_actors(&self) -> Vec<Actor> {
        self.actors.clone()
    }

    pub fn get_game_over_reason(&self) -> GameOverReason {
        self.game_over_reason
    }

    pub fn moved_by_bat(&self) -> bool {
        self.super_bat_move
    }

    pub fn bumped_wumpus(&self) -> bool {
        self.bumped_wumpus
    }

    pub fn is_all_rooms_entered(&self) -> bool {
        self.current_arrow_room_count == self.number_of_arrow_rooms
    }

    pub fn game_action(&mut self, action: GameAction, action_count: Option<u8>, random: &impl Random) {
        match action {
            GameAction::StartGame => {
                self.start_game(random);
            }
            GameAction::WaitNextInstruction => {
                self.state = GameStates::WaitInstructions
            }
            GameAction::MoveOrShoot => {
                self.state = GameStates::DoYouWantToMoveOrShoot
            }
            GameAction::Move => {
                self.state = GameStates::MoveEnterRoomNumber
            }
            GameAction::MoveToRoom => {
                match action_count {
                    Some(room_number) => {
                        self.move_you(room_number, random);
                        if self.state != GameStates::GameOver {
                            self.state = GameStates::DoYouWantToMoveOrShoot;
                        }
                    }
                    None => {
                        self.state = GameStates::MoveEnterRoomNumber;                        
                    }
                    // Perform move
                }
            }
            GameAction::Shoot => {
                self.state = GameStates::ShootEnterNumberOfRooms;
                self.current_arrow_room_count = 0;
                self.rooms_to_shoot_arrows_in = Vec::new();
            }
            GameAction::ShootNumberOfRooms => {
                match action_count {
                    Some(number_of_rooms) => {
                        if number_of_rooms > 5 {
                            self.number_of_arrow_rooms = 5;                           
                        } else {
                            self.number_of_arrow_rooms = number_of_rooms;                           
                        }
                        self.state = GameStates::ShootEnterRoomNumber;
        
                    }
                    None => {
                        
                    }
                }
            }
            GameAction::ShootRoomNumber => {
                match action_count {
                    Some(room_number) => {
                        self.current_arrow_room_count += 1;
                        if self.current_arrow_room_count < self.number_of_arrow_rooms {
                            self.rooms_to_shoot_arrows_in.push(room_number);        
                        } else {
                            self.rooms_to_shoot_arrows_in.push(room_number);
                            // Make a copy be get ownership        
                            let mut rooms: Vec<u8> = Vec::new();
                            self.rooms_to_shoot_arrows_in.iter().for_each(|x|  {
                                rooms.push(*x);
                            });
                            self.shoot(rooms, random);
                            if self.state != GameStates::GameOver {
                                self.state = GameStates::DoYouWantToMoveOrShoot;
                            }
                        }        
                    }
                    None => {

                    }
                }
            }
            GameAction::Restart => {
                self.state = GameStates::Restart
            }
            GameAction::RestartClearAll => {
                self.clear_state();
                self.clear_actors();
                self.actors = place_actors(random);
                self.state = GameStates::DoYouWantToMoveOrShoot;
            }
            GameAction::RestartSameSetup => {
                self.clear_state();
                self.state = GameStates::DoYouWantToMoveOrShoot;
            }
        };
    }

    fn start_game(&mut self, random: &impl Random) {
        self.actors = place_actors(random);
        self.state = GameStates::DoYouWantInstructions
    }

    pub fn get_actor(&self, actor_type: ActorType) -> &Actor {
        return self.actors.iter().find(|x| x.actor_type == actor_type).unwrap();
    }

    pub fn get_tunnels_for_actor(&self, actor_type: ActorType) -> [u8;3] {
        let your_location = self.get_actor(actor_type).room;
        return self.get_tunnels(your_location);
    }

    pub fn get_tunnels(&self, room: u8) -> [u8;3] {
        let room_value: Result<usize, _> = room.try_into();
        match room_value {
            Ok(value) => {
                return MAZE[value - 1];

            },
            Err(_err) => {
                return [0,0,0];
            }
        }
    }

    pub fn move_you(&mut self, mut new_room: u8, random: &impl Random) {
        // Check if move is legal
        self.bumped_wumpus = false;
        self.super_bat_move = false;
        let tunnels = self.get_tunnels_for_actor(ActorType::You);
        let move_valid = tunnels.iter().any(|x| *x == new_room);
        if move_valid {

            // Check for dangers
            let bats = self.is_actor_in_room(ActorType::Bat, new_room);
            if bats {
                new_room = random.get_random(1..20);
                self.super_bat_move = true;
            } 

            let pits = self.is_actor_in_room(ActorType::Pit, new_room);
            if pits {
                self.state = GameStates::GameOver;
                self.game_over_reason = GameOverReason::YouFellIntoPit;
            }

            let wumpus = self.is_actor_in_room(ActorType::Wumpus, new_room);
            if wumpus && !self.wumpus_moves {
                self.wumpus_moves = true;
                self.bumped_wumpus = true;
            } else if wumpus && self.wumpus_moves {
                self.state = GameStates::GameOver;
                self.game_over_reason = GameOverReason::WumpusGotYou;
                return;
            }

            // All good, move into room
            let you_index = self.actors.iter().position(|x| x.actor_type == ActorType::You).unwrap();
            self.actors[you_index].room = new_room;
            
        }
    }

    pub fn move_wumpus(&mut self, random: &impl Random) {
        // Determine if wumpus stays or moves (1/4 stay, 3/4 move)
        let properbility = random.get_random(1..100);
        let do_move = properbility > 25;
        if do_move {
            // Get tunnels where wumpus can move
            let tunnels = self.get_tunnels_for_actor(ActorType::Wumpus);
            // Select a random tunnel
            let random_tunnel_index = random.get_random(0..2); 
            let tunnel_index:usize = usize::from(random_tunnel_index);
            let new_room = tunnels[tunnel_index];

            let you = self.is_actor_in_room(ActorType::You, new_room);

            if you {
                self.state = GameStates::GameOver;
                self.game_over_reason = GameOverReason::WumpusGotYou;
            } else {
                // Did not get You, wumpus moves
                let wumpus_index = self.actors.iter().position(|x| x.actor_type == ActorType::Wumpus).unwrap();
                self.actors[wumpus_index].room = new_room;
            }
        }
    }

    pub fn shoot(&mut self, rooms: Vec<u8>, random: &impl Random) {
        // Check ammo
        self.number_of_arrows -= 1;
        if self.number_of_arrows == 0 {
            self.state = GameStates::GameOver;
            self.game_over_reason = GameOverReason::YouAreOutOfArrows;
            return;
        }
        
        let your_location = self.get_actor(ActorType::You).room;
        let wumpus_location = self.get_actor(ActorType::Wumpus).room;

        let mut arrow_location = self.get_actor(ActorType::You).room;

        for room in rooms {
            if room == your_location {
                    self.state = GameStates::GameOver;
                    self.game_over_reason = GameOverReason::YouShotYourself;
                    break;
            } else if room == wumpus_location {
                    self.state = GameStates::GameOver;
                    self.game_over_reason = GameOverReason::YouShotWumpus;
                    break;
            } else {
                let tunnels = self.get_tunnels(arrow_location);
                if tunnels.contains(&room) {
                    arrow_location = room;
                } else {
                    let random_tunnel_index = random.get_random(0..2);
                    let tunnel_index = usize::from(random_tunnel_index);
                    arrow_location = tunnels[tunnel_index]; 
                }
                
            }
        }
        // When You have fired a shot Wumpus starts moving
        self.move_wumpus(random);
    }
   
    pub fn dangers_nearby(&self) -> Vec<&Actor> {
        let your_location = self.get_actor(ActorType::You).room;
        let current_room = usize::try_from(your_location).unwrap();
        let tunnels: [u8; 3] = MAZE[current_room-1];
        let dangerous_actors = 
        self.actors.iter().filter(|x|
        {
            let found = tunnels.iter().any(|&y| 
                y == x.room &&
                (x.actor_type == ActorType::Bat ||
                 x.actor_type == ActorType::Pit ||
                 x.actor_type == ActorType::Wumpus));
            return found;     
        });
        return dangerous_actors.collect();
    }        
    
    fn is_actor_in_room(&self, actor_type: ActorType, room: u8) -> bool {
        if self.actors.iter().any(|x| x.actor_type == actor_type && x.room == room) {
            return true;
        }
        return false;
    }
}