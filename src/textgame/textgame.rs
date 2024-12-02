use crate::game::*;
use crate::util::Random;

type TextOuputFunction=fn(String);

const INSTRUCTIONS: [&'static str;48] = [
    "THE WUMPUS LIVES IN A CAVE OF 20 ROOMS. EACH ROOM",
    "HAS 3 TUNNELS LEADING TO OTHER ROOMS. (LOOK AT A",
    "DODECAHEDRON TO SEE HOW THIS WORKS - IF YOU DONT'T",
    "KNOW WHAT A DEDECHARON IS, ASK SOMEONE)",
    "",
    "     HAZARDS",
    "BOTTOMLESS PITS - TWO ROOMS HAVE BOTTOMLESS PITS IN THEM",
    "     IF YOU GO THERE YOU FALL INTO THE PIT (& LOSE!)",
    "SUPERBATS - TWO OTHER ROOMS HAVE SUPER BATS. IF YOU",
    "     GO THERE, A BAT GRABS YOU AND TAKES YOU TO SOME OTHER",
    "     ROOM AT RANDOM (WHICH MIGHT BE TOUBLESOME)",
    "",
    "                  PRESS ENTER TO CONTINUE",
    "     WUMPUS",
    "THE WUMPUS IS NOT BOTHERED BY THE HAZARDS (HE HAS SUCKER",
    "FEET AND IS TO BIG FOR A BAT TO LIFT). USUALLY",
    "HE IS ASLEEP. TWO THINGS THAT CAN WAKE HIM UP: YOUR ENTERING",
    "HIS ROOM OR YOUR SHOOTING AN ARROW",
    "   IF THE WUMPUS WAKES, HE MOVES (P=75) ONE ROOM",
    "OR STAYS STILL (P=25). AFTER THAT, IF HE IS WHERE YOU",
    "ARE, HE EATS YOU UP (& YOU LOOSE)",
    "",
    "                  PRESS ENTER TO CONTINUE",
    "     YOU",
    "EACH TURN YOU MAY MOVE OR SHOOT A CROOKED ARROW",
    "",
    "     MOVING: YOU CAN GO ONE ROOM (THRU ONE TUNNEL)",
    "     ARROWS: YOU HAVE 5 ARROWS. YOU LOSE WHEN YOU RUN OUT.",
    "     EACH ARROW CAN GO FROM 1 TO 5 ROOMS. YOU AIM BY TELLING",
    "     THE COMPUTER THE ROOMS YOU WANT THE ARROW TO GO TO.",
    "     IF THE ARROW CAN´T GO THAT WAY (I.E, NO TUNNEL) IT MOVES",
    "     AT RANDOM TO THE NEXT ROOM",
    "",
    "       IF THE ARROW HITS THE WUMPUS, YOU WIN.",
    "       IF THE ARROW HITS YOU, YOU LOSE",
    "",
    "                  PRESS ENTER TO CONTINUE",
    "     WARNINGS:",
    "     WHEN YOU ARE ONE ROOM AWAY FROM WUMPUS OR HAZARD,",
    "     THE COMPUTER SAYS:",
    "",
    "WUMPUS-    'I SMELL A WUMPUS'",
    "BAT   -    'BATS NEARBY'",
    "PIT   -    'I FEEL A DRAFT",
    "",
    "",
    "                       HUNT THE WUMPUS",
    ""
];

pub struct TextGame {
    instruction_index:usize,
    game:Game,
    out:TextOuputFunction,
}

impl TextGame {
    pub fn new(text_out:TextOuputFunction) -> Self {
        TextGame {
            instruction_index: 0,
            out: text_out, 
            game: Game::new()
        }
    }

    pub fn start(&mut self, random: &impl Random) {
        (self.out)("COPYRIGHT 1979 CREATIVE COMPUTING MORRISTOWN, NJ".to_string());
        (self.out)("DO YOU NEED INSTRUCTIONS?".to_string());
        self.game.game_action(GameAction::StartGame, None, random);
    }

    pub fn next_instruction_text(&mut self) -> bool {
        let mut complete = false;
        loop {
            if self.instruction_index > INSTRUCTIONS.len()-1 {
                self.instruction_index = 0;
                complete = true;
                break;
            }
            let current_line = INSTRUCTIONS[self.instruction_index].to_string();
            let test_current_line = current_line.clone();
            self.instruction_index += 1;
            (self.out)(current_line);
            if test_current_line.contains("PRESS ENTER") {
                break;
            }
        }
        return complete; 
    }

    pub fn text_action(&mut self, text:String, random: &impl Random) {
        match self.game.get_game_state() {
            GameStates::DoYouWantInstructions => {
                match text.to_uppercase().trim() {
                    "Y" => {
                            self.next_instruction_text(); 
                            self.game.game_action(GameAction::WaitNextInstruction, None, random); 
                        }
                    "N" => { 
                            self.show_your_location();
                            self.show_tunnels();
                            self.show_dangers_nearby();
                            self.show_move_or_shoot();
                            self.game.game_action(GameAction::MoveOrShoot, None, random); 
                        }
                    _ =>   { 
                            (self.out)("Please answer (Y/N)".to_string()); 
                        }
                }
            },
            GameStates::WaitInstructions => {
                let complete = self.next_instruction_text();
                if complete {
                    self.show_your_location();
                    self.show_tunnels();
                    self.show_dangers_nearby();
                    self.show_move_or_shoot();
                    self.game.game_action(GameAction::MoveOrShoot, None, random); 
                }
            }
            GameStates::DoYouWantToMoveOrShoot => {
                match text.to_uppercase().trim()  {
                    "M" => { 
                        self.game.game_action(GameAction::Move, None, random);
                        (self.out)("WHERE TO:".to_string());
                    }
                    "S" => { 
                        self.game.game_action(GameAction::Shoot, None, random);
                        (self.out)("NO. OF ROOMS(1-5)".to_string()) 
                    }
                    _ =>   { (self.out)("PLEASE ENTER M OR S".to_string()); }
                }
            }
            GameStates::MoveEnterRoomNumber => {
                let room_number_parse_result = self.convert_to_u8(text);
                match room_number_parse_result {
                    Ok(room_number) => {
                        self.game.game_action(GameAction::MoveToRoom, Some(room_number), random);
                    }
                    Err(error) => {
                        (self.out)(error.to_string())
                    }
                }
                if self.game.get_game_state()!=GameStates::GameOver {
                    self.check_bat_move();
                    self.check_bumped_wumpus();
                    self.show_your_location();
                    self.show_dangers_nearby();
                    self.show_tunnels();
                    self.show_move_or_shoot();
                } else {
                    self.check_bat_move();
                    self.show_game_over();
                }

            }
            GameStates::ShootEnterNumberOfRooms => {
                let number_of_rooms_parse_result = self.convert_to_u8(text);
                match number_of_rooms_parse_result {
                    Ok(number_of_rooms) => {
                        self.game.game_action(GameAction::ShootNumberOfRooms, Some(number_of_rooms), random);
                        (self.out)("ROOM#".to_string())
                    }
                    Err(error) => {
                        (self.out)(error.to_string())
                    }
                }

            }
            GameStates::ShootEnterRoomNumber => {
                let room_number_parse_result = self.convert_to_u8(text);
                match room_number_parse_result {
                    Ok(room_number) => {
                        self.game.game_action(GameAction::ShootRoomNumber, Some(room_number), random);
                        if self.game.get_game_state() == GameStates::GameOver {
                            self.show_game_over();
                            self.game.game_action(GameAction::Restart, None, random);
                        }
                        else if self.game.is_all_rooms_entered() {
                            (self.out)("MISSED".to_string());
                            self.show_your_location();
                            self.show_dangers_nearby();
                            self.show_tunnels();
                            self.show_move_or_shoot();
                        } 
                        else if !self.game.is_all_rooms_entered() {
                            (self.out)("ROOM#".to_string())
                        }
                    }
                    Err(error) => {
                        (self.out)(error.to_string())
                    }
                }

            }
            GameStates::GameOver => {
                self.show_game_over();
                self.game.game_action(GameAction::Restart, None, random)
            }
            GameStates::Restart => {
                self.show_restart();
                match text.to_uppercase().trim() {
                    "Y" => {
                        self.game.game_action(GameAction::RestartSameSetup, None, random);
                        self.show_your_location();
                        self.show_dangers_nearby();
                        self.show_tunnels();
                        self.show_move_or_shoot();

                    }
                    "N" => {
                        self.game.game_action(GameAction::RestartClearAll, None, random);
                        self.show_your_location();
                        self.show_dangers_nearby();
                        self.show_tunnels();
                        self.show_move_or_shoot();
                    }
                    "Q" => {

                    }
                    _ => {
                        (self.out)("PLEASE ANSWER Y,N OR Q".to_string());
                        self.game.game_action(GameAction::Restart, None, random)
                    }
                } 

            }
            _ => {}
        }

    }

    pub fn is_game_over(&self) -> bool {
        match self.game.get_game_state() {
            GameStates::GameOver => return true,
            _ => return false
        }
    }

    fn show_instructions(&mut self) {
        INSTRUCTIONS.iter().for_each(|text| {
            (self.out)(text.to_string());
        })
    }

    fn show_move_or_shoot(&mut self) {
        (self.out)("SHOOT OR MOVE (S-M)?".to_string());
    }

    fn show_your_location(&mut self) {
        let you = self.game.get_actor(ActorType::You);
        (self.out)(format!("YOU ARE IN ROOM {}", you.room)); 
        // let wumpus = self.game.get_actor(ActorType::Wumpus);
        // (self.out)(format!("WUMPUS ARE IN ROOM {}", wumpus.room)); 
    }

    fn show_tunnels(&mut self) {
        let actor = self.game.get_actor(ActorType::You);
        let tunnels = self.game.get_tunnels(actor.room);
        let tunnel_list = tunnels.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
        (self.out)(format!("TUNNELS LEAD TO {}", tunnel_list));
    }

    fn check_bat_move(&mut self) {
        if self.game.moved_by_bat() {
            (self.out)("ZAP--SUPER BAT SNATCH! ELSEWHERE FOR YOU".to_string())
        }
    }

    fn check_bumped_wumpus(&mut self) {
        if self.game.bumped_wumpus() {
            (self.out)("... OOPS BUMPED A WUMPUS".to_string())
        }
    }

    fn show_dangers_nearby(&mut self) {
        let danger_actors = self.game.dangers_nearby();
        let danger_actors_texts = self.dangers_nearby_to_string(danger_actors);

        danger_actors_texts.iter().for_each(|text| {
            (self.out)(text.to_string());
        })
    }

    fn show_game_over(&mut self) {
        let reason = self.game.get_game_over_reason();
        let reason_text = match reason {
            GameOverReason::NotDeadYet => "NOT DEAD YET".to_string(),
            GameOverReason::WumpusGotYou => "WUMPUS GOT YOU".to_string(),
            GameOverReason::YouAreOutOfArrows => "YOU ARE OUT OF ARROWS".to_string(),
            GameOverReason::YouFellIntoPit => "YOU FELL IN A PIT".to_string(),
            GameOverReason::YouShotWumpus => "AHA! YOU GOT THE WUMPUS".to_string(),
            GameOverReason::YouShotYourself => "YOU SHOT YOURSELF".to_string()
        };
        (self.out)(reason_text);
        match reason {
            GameOverReason::YouShotWumpus => {
                (self.out)("HEE HEE HEE - THE WUMPUS'LL GETCHA NEXT TIME!!".to_string())
            }
            _ => {
                (self.out)("HA HA HA - YOU LOSE!".to_string());
            }
        } 
    }

    fn show_restart(&self) {
        (self.out)("SAME SET-UP (Y-N) OR 'Q' TO QUIT".to_string());       
    }

    // fn actor_to_string(&self, actor: &Actor) -> String {
    //     match actor.actor_type {
    //         ActorType::You => "You".to_string(),
    //         ActorType::Wumpus => "Wumpus".to_string(),
    //         ActorType::Pit => "Pit".to_string(),
    //         ActorType::Bat => "Bat".to_string(),
    //     }
    // }

    // fn game_state_to_string(&self) -> String {
    //     match self.game.get_game_state() {
    //         GameStates::DoYouWantInstructions => "Instructions".to_string(),
    //         GameStates::DoYouWantToMoveOrShoot => "MoveOrShoot".to_string(),
    //         GameStates::GameOver => "GameOver".to_string(),
    //         GameStates::GameStart => "GameStart".to_string(),
    //         GameStates::MoveEnterRoomNumber => "MoveEnterRoom".to_string(),
    //         GameStates::ShootEnterNumberOfRooms => "ShootEnterNumberOfRooms".to_string(),
    //         GameStates::ShootEnterRoomNumber => "ShootEnterRoomNumber".to_string(),
    //         GameStates::Restart => "RestartGame".to_string()
    //     }
    // }

    fn dangers_nearby_to_string(&self, actors: Vec<&Actor>) -> Vec<String> {
        let mut danger_list: Vec<String> = Vec::new();
        for actor in actors {
            match actor.actor_type {
                ActorType::Wumpus => danger_list.push("I SMELL A WUMPUS!".to_string()),
                ActorType::Bat => danger_list.push("BATS NEARBY!".to_string()),
                ActorType::Pit => danger_list.push("I FEEL A DRAFT".to_string()),
                ActorType::You => {}
            }
        }
        danger_list
    }


    fn convert_to_u8(&mut self, input_text:String) -> Result<u8, std::num::ParseIntError> {
        input_text.trim().parse::<u8>()
    }

}
