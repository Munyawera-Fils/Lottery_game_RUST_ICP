use ic_cdk::export::Principal;
use ic_cdk::api::{caller, time};
use ic_cdk_macros::*;
use ic_cdk::random::Uniform;

struct LotteryGame {
    participants: Vec<Principal>,
    is_active: bool,
}

impl LotteryGame {
    fn new() -> Self {
        LotteryGame {
            participants: Vec::new(),
            is_active: true,
        }
    }

    fn participate(&mut self) {
        if self.is_active {
            let caller_principal = caller();
            self.participants.push(caller_principal);
        }
    }

    fn draw_winner(&mut self) -> Option<Principal> {
        if self.is_active && !self.participants.is_empty() {
            let winner_index = Uniform::new(0, self.participants.len()).sample(&mut ic_cdk::random::thread_rng());
            self.is_active = false;
            Some(self.participants[winner_index])
        } else {
            None
        }
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

#[init]
fn init() {
    let _game = LotteryGame::new();
}

#[update]
fn participate(game: &mut LotteryGame) {
    game.participate();
}

#[update(admin)]
fn draw_winner(game: &mut LotteryGame) -> Option<Principal> {
    game.draw_winner()
}

#[query]
fn is_active(game: &LotteryGame) -> bool {
    game.is_active()
}
