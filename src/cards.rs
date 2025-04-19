use rand::Rng;
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Card {
    rank: i32,
    suite: Suite,
}
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Suite {
    Diamond,
    Club,
    Heart,
    Spade,
}

impl Card {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        // Generate random rank (1-13, where 1 is Ace, 11 is Jack, etc.)
        let rank = rng.gen_range(1..=13);

        // Generate random suite (0-3)
        let suite = match rng.gen_range(0..4) {
            0 => Suite::Diamond,
            1 => Suite::Club,
            2 => Suite::Heart,
            _ => Suite::Spade,
        };

        Card { rank, suite }
    }
    pub fn newDeck() -> Vec<Card> {
        let mut vec = Vec::new();
        for i in 1..=13 {
            for j in 0..=3 {
                // Generate random suite (0-3)
                let suite = match j {
                    0 => Suite::Diamond,
                    1 => Suite::Club,
                    2 => Suite::Heart,
                    _ => Suite::Spade,
                };
                vec.push(Card {
                    rank: i,
                    suite: suite,
                });
            }
        }
        return vec;
    }

    pub fn shuffle_deck(deck: &mut Vec<Card>) {
        let mut rng = rand::thread_rng();
        for i in (1..deck.len()).rev() {
            let j = rng.gen_range(0..=i);
            deck.swap(i, j);
        }
    }
}
