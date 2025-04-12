use crate::cards::Card;
use std::collections::{HashMap, HashSet};

/// users: Map<uuid, user_info>
/// user_info: { hand: Vec<Card>, balance: int}
/// community_cards: Vec<Card>
/// curr user: string
/// curr big blind: string
/// bets: Map<uuid, int>
struct PokerGame {
    users: Vec<UserInfo>,
    user_map: HashMap<String, usize>,
    curr_user: usize,
    curr_uuid: String,
    curr_blind: usize,
    community_cards: Vec<Card>,
    turn_num: i32,
    current_state: GameState,
    played_cards: HashSet<Card>,
}

struct UserInfo {
    hand: Vec<Card>,
    bet: Bet,
    is_playing: bool,
    is_room_owner: bool,
}
const MAX_SIZE: usize = 10;
struct Bet {
    bet_type: BetType,
    value: i32,
}

enum BetType {
    CHECK,
    BET,
    CALL,
    RAISE,
    FOLD,
    ALLIN,
}

enum GameState {
    INITIAL = 0,
    BLIND = 1,
    DEAL = 2,
    FLOPBETTING = 3,
    FLOP = 4,
    TURNBETTING = 5,
    TURN = 6,
    RIVERBETTING = 7,
    RIVER = 8,
    SHOWDOWN = 9,
    POTDISTRIBUTION = 10,
    TERMINAL = 11,
}

struct ConnectionAction {
    action_type: ConnectionActionType,
    uuid: String,
}
struct BetAction {
    uuid: String,
    bet: Bet,
}

enum ConnectionActionType {
    JOIN,
    QUIT,
    START,
}

pub enum Action {
    Bet(BetAction),
    Connection(ConnectionAction),
}

impl PokerGame {
    /// Once all players join or room owner starts the game
    /// Assign small blind/big blind (deduct automatically from user)
    /// Deal state: deal 2 cards to everyone
    /// reveal flop
    /// Bet on flop - if everyone folds user remaining wins
    /// reveal turn if betting is equalized
    /// bet on turn - if everyone folds, user remaining wins
    /// reveal river if betting is equalized
    /// bet on river - if everyone folds, user remaining wins
    /// showdown - identify winning hand
    /// give chips to winner
    /// Terminate
    fn run(&mut self, action: Action) {
        match self.current_state {
            GameState::INITIAL => {
                if let Action::Connection(connect_action) = action {
                    self.handle_initial(connect_action)
                }
            }
            GameState::BLIND => self.handle_blind(),
            GameState::DEAL => self.handle_deal(),
            GameState::FLOPBETTING => todo!(),
            GameState::FLOP => todo!(),
            GameState::TURNBETTING => todo!(),
            GameState::TURN => todo!(),
            GameState::RIVERBETTING => todo!(),
            GameState::RIVER => todo!(),
            GameState::SHOWDOWN => todo!(),
            GameState::POTDISTRIBUTION => todo!(),
            GameState::TERMINAL => todo!(),
        }
    }
    fn handle_initial(&self, action: ConnectionAction) {}
    fn handle_blind(&self) {}
    fn handle_deal(&mut self) {
        let mut card: Card;
        let mut card2: Card;
        loop {
            card = Card::random();
            card2 = Card::random();

            if !self.played_cards.contains(&card) && !self.played_cards.contains(&card2) {
                break;
            }
        }
        self.played_cards.insert(card);
        let user: &mut UserInfo = self.users.get_mut(self.curr_user).unwrap();
        user.hand.push(card);
        user.hand.push(card2);
    }
    /// Check if its even users turn to bet
    /// If it is, update users bet info
    /// increment curr bet
    fn handle_bet(&mut self, betAction: BetAction) {
        if (self.curr_user == *self.user_map.get(&betAction.uuid).unwrap()) {
            let user_bet: &mut Bet = &mut self.users.get_mut(self.curr_user).unwrap().bet;
            user_bet.bet_type = betAction.bet.bet_type;
            if self.curr_user == MAX_SIZE - 1 {
                self.curr_user = 0
            } else {
                self.curr_user = self.curr_user + 1;
            }
        }
    }
    fn reveal_flop(&self) {}
    fn bet_flop(&self, bet: BetAction) {}
    fn reveal_turn(&self) {}
    fn bet_turn(&self, bet: BetAction) {}
    fn reveal_river(&self) {}
    fn bet_river(&self, bet: BetAction) {}
    fn showdown(&self) {}
    fn terminate(&self) {}
}
