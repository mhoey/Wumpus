pub const MAX_ROOMS: u8 = 20;

pub const MAZE: [[u8;3];20] = 
    [
        [2,5,6], // 1
        [1,3,8], // 2 
        [2,4,10], // 3
        [3,5,12], // 4
        [4,1,14], // 5
        [1,7,15], // 6
        [6,8,16], // 7
        [2,7,9], // 8
        [8,10,17], // 9
        [3,9,11], // 10
        [10,12,18], // 11
        [4,11,13], // 12
        [12,14,19], // 13
        [5,13,15], // 14
        [6,14,20], // 15
        [7,17,20], // 16
        [9,16,18], // 17
        [11,17,19], // 18
        [12,18,20], // 19
        [15,16,19], // 20
];

#[derive(Clone, Copy, PartialEq)]
pub enum GameStates {
    GameStart,
    DoYouWantInstructions,
    WaitInstructions,
    DoYouWantToMoveOrShoot,
    MoveEnterRoomNumber,
    ShootEnterNumberOfRooms,
    ShootEnterRoomNumber,
    GameOver,
    Restart
}

pub enum GameAction {
    StartGame,
    WaitNextInstruction,
    MoveOrShoot,
    Move,
    Shoot,
    ShootNumberOfRooms,
    ShootRoomNumber,
    MoveToRoom,
    Restart,
    RestartClearAll,
    RestartSameSetup,
}

#[derive(Clone, Copy)]
pub enum GameOverReason {
    NotDeadYet,
    YouFellIntoPit,
    YouShotYourself,
    YouShotWumpus,
    YouAreOutOfArrows,
    WumpusGotYou
}