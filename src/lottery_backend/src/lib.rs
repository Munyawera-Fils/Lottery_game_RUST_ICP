use candid::Principal;
use ic_cdk::api::{caller, time};
use ic_cdk_macros::*;

// Define the smart contract struct
struct LotteryGame {
    participants: Vec<Principal>,
    is_active: bool,
}

impl LotteryGame {
    // Constructor
    fn new() -> Self {
        LotteryGame {
            participants: Vec::new(),
            is_active: true,
        }
    }

    // Method to participate in the lottery
    fn participate(&mut self) {
        if self.is_active {
            let caller_principal = caller();
            self.participants.push(caller_principal);
        }
    }

    // Method to draw a winner
    fn draw_winner(&mut self) -> Option<Principal> {
        if self.is_active && !self.participants.is_empty() {
            self.is_active = false;
            let winner_index = time() as usize % self.participants.len();
            Some(self.participants[winner_index])
        } else {
            None
        }
    }

    // Method to check if the lottery is active
    fn is_active(&self) -> bool {
        self.is_active
    }
}

#[init]
fn init() {
    let _game = LotteryGame::new(); // Assign to a variable to keep the instance alive
}

#[ic_cdk::update]
fn participate() {
    let mut game = LotteryGame::new();
    game.participate();
}

#[ic_cdk::update]
fn draw_winner() -> Option<Principal> {
    let mut game = LotteryGame::new();
    game.draw_winner()
}

#[ic_cdk::query]
fn is_active() -> bool {
    let game = LotteryGame::new();
    game.is_active()
}

ic_cdk::export_candid!();