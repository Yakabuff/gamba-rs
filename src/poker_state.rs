use crate::cards::Card;
use std::collections::HashMap;

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
    deck: Vec<Card>,
    pot: i32,
    betting_stage: BettingStage,
    bet_amount: i32,
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
#[derive(PartialEq)]
enum BetType {
    NONE,
    CHECK,
    BET,
    CALL,
    RAISE,
    FOLD,
    ALLIN,
}

/// NOBETS -> allows check/bet
/// BETPLACED -> allows raise/call
/// RAISED -> allows call
/// FINISHED -> everyone called
#[derive(PartialEq)]
enum BettingStage {
    NOBETS,
    BETPLACED,
    RAISED,
    CALLED,
    FINISHED,
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
    TERMINAL = 10,
}

impl GameState {
    fn next(self) -> Self {
        match self {
            GameState::INITIAL => GameState::BLIND,
            GameState::BLIND => GameState::DEAL,
            GameState::DEAL => GameState::FLOPBETTING,
            GameState::FLOPBETTING => GameState::FLOP,
            GameState::FLOP => GameState::TURNBETTING,
            GameState::TURNBETTING => GameState::TURN,
            GameState::TURN => GameState::RIVERBETTING,
            GameState::RIVERBETTING => GameState::RIVER,
            GameState::RIVER => GameState::SHOWDOWN,
            GameState::SHOWDOWN => GameState::TERMINAL,
            GameState::TERMINAL => self,
        }
    }
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
    fn new() -> PokerGame {
        let mut deck = Card::newDeck();
        Card::shuffle_deck(&mut deck);
        PokerGame {
            users: Vec::new(),
            user_map: HashMap::new(),
            curr_user: 0,
            curr_uuid: String::new(),
            curr_blind: 0,
            community_cards: Vec::new(),
            turn_num: 0,
            current_state: GameState::INITIAL,
            deck,
            pot: 0,
            betting_stage: BettingStage::NOBETS,
            bet_amount: 0,
        }
    }
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
    fn run(&mut self, action: Action) -> Result<(), String> {
        match self.current_state {
            GameState::INITIAL => {
                if let Action::Connection(connect_action) = action {
                    self.handle_initial(connect_action)
                }
            }
            GameState::BLIND => {
                self.handle_blind();
            }
            GameState::DEAL => {
                self.handle_deal();
            }
            GameState::FLOPBETTING => {
                if let Action::Bet(bet_action) = action {
                    self.betting_stage(bet_action, GameState::FLOPBETTING)?;
                }
            }
            GameState::FLOP => {
                self.reveal_flop();
            }
            GameState::TURNBETTING => todo!(),
            GameState::TURN => {
                self.reveal_turn();
            }
            GameState::RIVERBETTING => todo!(),
            GameState::RIVER => {
                self.reveal_river();
            }
            GameState::SHOWDOWN => todo!(),
            GameState::TERMINAL => todo!(),
        }
        Ok(())
    }
    fn handle_initial(&self, action: ConnectionAction) {}
    fn handle_blind(&self) {}
    fn handle_deal(&mut self) {
        let user: &mut UserInfo = self.users.get_mut(self.curr_user).unwrap();
        user.hand.push(self.deck.pop().unwrap());
        user.hand.push(self.deck.pop().unwrap());
    }
    /// Check if its even users turn to bet
    /// If it is your turn to bet, check if user hasn't folded
    /// If it is, update users bet info
    /// increment curr bet
    fn betting_stage(&mut self, betAction: BetAction, game_state: GameState) -> Result<(), String> {
        if self.curr_user == *self.user_map.get(&betAction.uuid).unwrap() {
            if self.users.get(self.curr_user).unwrap().bet.bet_type == BetType::FOLD {
                return Err("User already folded. Invalid move".to_string());
            }
            if self.users.get(self.curr_user).unwrap().is_playing == false {
                return Err("Game is in progress, user is not playing".to_string());
            }
            match betAction.bet.bet_type {
                BetType::CHECK => self.handle_check()?,
                BetType::BET => self.handle_bet(betAction.bet.value)?,
                // If previous bet is call, raise, fold
                // If call, check if everyone called after.  If everyone called, advance game state
                // Reset all bets
                BetType::CALL => self.handle_call()?,
                BetType::RAISE => self.handle_raise(betAction.bet.value)?,
                BetType::FOLD => {
                    let curr_bet: &mut Bet = &mut self.users.get_mut(self.curr_user).unwrap().bet;
                    curr_bet.bet_type = betAction.bet.bet_type;
                    curr_bet.value = betAction.bet.value;
                }
                BetType::ALLIN => todo!(),
                BetType::NONE => todo!(),
            }
            // Increment curr user if bet is succesful
            if self.curr_user == MAX_SIZE - 1 {
                self.curr_user = 0
            } else {
                self.curr_user = self.curr_user + 1;
            }
        } else {
            return Err("Not your turn".to_string());
        }
        self.current_state = game_state.next();
        return Ok(());
    }

    /// Make sure no bets. Previous bets must be either check, none or fold
    fn handle_check(&mut self) -> Result<(), String> {
        if self.betting_stage == BettingStage::NOBETS {
            let curr_bet: &mut Bet = &mut self.users.get_mut(self.curr_user).unwrap().bet;
            curr_bet.bet_type = BetType::CHECK;
            curr_bet.value = 0;
            return Ok(());
        }
        Err("Could not check as bet has already been placed".to_string())
    }

    /// Make sure no bets
    fn handle_bet(&mut self, value: i32) -> Result<(), String> {
        if self.betting_stage == BettingStage::NOBETS {
            let curr_bet: &mut Bet = &mut self.users.get_mut(self.curr_user).unwrap().bet;
            curr_bet.bet_type = BetType::BET;
            curr_bet.value = value;
            self.betting_stage = BettingStage::BETPLACED;
            self.pot = self.pot + value;
            return Ok(());
        }
        Err("Could not bet as bet has already been placed".to_string())
    }

    /// Make sure there is a bet
    /// Make sure raise is greater than current bet * 2
    fn handle_raise(&mut self, value: i32) -> Result<(), String> {
        if self.betting_stage == BettingStage::BETPLACED && value > self.bet_amount * 2 {
            let curr_bet: &mut Bet = &mut self.users.get_mut(self.curr_user).unwrap().bet;
            curr_bet.bet_type = BetType::RAISE;
            curr_bet.value = value;
            self.betting_stage = BettingStage::RAISED;
            self.pot = self.pot + value;
            self.bet_amount = self.bet_amount + value;
            return Ok(());
        }
        if self.betting_stage != BettingStage::BETPLACED {
            return Err("Cannot raise as bet has not been placed yet".to_string());
        }
        Err("Raise amount too small".to_string())
    }

    /// Make sure someone has raised
    fn handle_call(&mut self) -> Result<(), String> {
        if self.betting_stage == BettingStage::RAISED {
            let curr_bet: &mut Bet = &mut self.users.get_mut(self.curr_user).unwrap().bet;
            curr_bet.bet_type = BetType::CALL;
            curr_bet.value = self.bet_amount;
            self.betting_stage = BettingStage::CALLED;
            self.pot = self.pot + self.bet_amount;
            return Ok(());
        }
        Err("Cannot call as bet has not been raised".to_string())
    }

    /// Check if everyone has either
    /// 1) Everyone checks - NOBET stage
    /// 2) Someone bets, everyone else calls - CALLED stage
    /// Someone bets, everyone folds - BETPLACED stage
    /// 3) Someone raises, everyone calls - CALLED stage
    /// Someone raises everyone folds - RAISED stage
    /// Somone raises, everyone calls OR folds
    /// 4) Everyone folds except 1 person - any stage
    /// 5) Someone all ins, all other players called highest bet possible except person who all ined
    fn check_betting_finished_nobet(&mut self) -> bool {
        // Check if everyone folded or checked
        let mut res: bool = false;
        for i in &self.users {
            if i.is_playing && (i.bet.bet_type == BetType::FOLD || i.bet.bet_type == BetType::CHECK)
            {
                res = res || true;
            }
        }
        return res;
    }

    fn reset_bets(&mut self) {
        for i in &mut self.users {
            i.bet.bet_type = BetType::NONE;
            i.bet.value = 0;
        }
    }

    fn reveal_flop(&mut self) {
        let cards = self.drawNCards(3);
        self.community_cards = cards
    }
    fn bet(&mut self, bet: BetAction) {}
    fn reveal_turn(&mut self) {
        let mut cards = self.drawNCards(1);
        self.community_cards.append(&mut cards)
    }
    fn bet_turn(&self, bet: BetAction) {}
    fn reveal_river(&mut self) {
        let mut cards = self.drawNCards(1);
        self.community_cards.append(&mut cards)
    }
    fn bet_river(&self, bet: BetAction) {}
    fn showdown(&self) {}
    fn terminate(&self) {}

    fn drawNCards(&mut self, num: i32) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        for i in 0..num {
            cards.push(self.deck.pop().unwrap());
        }
        return cards;
    }
}
