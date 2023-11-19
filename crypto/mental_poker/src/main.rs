use std::collections::HashMap;

use enc::util::{gcd, num_inv_by_mod};
use num::One;
use num_primes::{Generator, BigUint};
use rand::{thread_rng, seq::SliceRandom};

extern crate enc;

static SUITS: &'static [&str] = &[
    "spades", "hearts", "diamonds", "clubs"
];
static CARDS: &'static [&str] = &[
    "2", "3", "4", "5", "6", "7", "8",
    "9", "10", "jack", "queen", "king", "ace"
];
static DECK_LEN: usize = CARDS.len()*SUITS.len();

#[derive(Clone, Debug)]
struct Player<'a> {
    p: &'a BigUint,
    c: BigUint,
    d: BigUint,
    deck: &'a Vec<BigUint>,
    map: &'a HashMap<BigUint, (String, String)>,
    hand: Vec<BigUint>,
    decoded_cards: Vec<BigUint>
}

impl Player<'_> {
    pub fn enc_deck(&self, deck: Vec<BigUint>) -> Vec<BigUint> {
        let mut ret = Vec::new();
        for card in deck {
            ret.push(
                card.modpow(&self.c, &self.p)
            );
        }
        ret
    }
    pub fn decode_deck(&self, deck: Vec<BigUint>) -> Vec<BigUint> {
        let mut ret = Vec::new();
        for card in deck {
            ret.push(
                card.modpow(&self.d, &self.p)
            );
        };
        ret
    }
}

fn main() {
    let players_num = 2;
    let cards_in_hand = 2;
    let cards_on_table = 5;
    let mut table = Vec::new();

    let mut players = Vec::with_capacity(4);
    let shared_p = Generator::new_prime(1024);
    let shared_p_neg1 = &shared_p - BigUint::one();
    let mut deck: Vec<BigUint> = Vec::with_capacity(52);
    let mut map: HashMap<BigUint, (String, String)> = HashMap::new();
    for card in CARDS {
        for suit in SUITS {
            let card_code = loop {
                let card = Generator::new_uint(1024);
                if card < shared_p_neg1 {
                    break card;
                }
            };
            map.insert(card_code.clone(), (card.to_string(), suit.to_string()));
            deck.push(card_code);
        }
    }
    for _ in 1..players_num {
        let c = loop {
            let c = Generator::new_uint(1024);
            if gcd(&c, &shared_p_neg1) == BigUint::one() {
                break c;
            }
        };
        let d = num_inv_by_mod(&c, &(&shared_p - BigUint::one()));
        if ((&c % &shared_p_neg1) * (&d % &shared_p_neg1)) % (&shared_p - BigUint::one()) != BigUint::one() {
            todo!("panic");
        }
        players.push(Player {
            p: &shared_p,
            c: c,
            d: d,
            deck: &deck,
            hand: Vec::new(),
            map: &map,
            decoded_cards: Vec::new()
        })
    }

    let mut enc_deck = players[0].enc_deck(deck.clone());
    enc_deck.shuffle(&mut thread_rng());
    for i in 1..players_num-1 {
        enc_deck = players[i].enc_deck(enc_deck);
        enc_deck.shuffle(&mut thread_rng());
    }

    for player in players.iter_mut() {
        for _ in 1..cards_in_hand {
            player.hand.push(enc_deck.pop().unwrap());
        }
    }
    for _ in 1..cards_on_table {
        table.push(enc_deck.pop().unwrap());
    }

    // for (idx, player) in players.iter().enumerate() {
    //     println!("{idx}: {:#?}", player.hand);
    // }
    // println!("Table: {:#?}\n\n", table);

    let players2 = players.clone();
    for player in players.iter_mut() {
        let mut others = players2.clone();
        others.retain(|pl| !std::ptr::eq(pl, player));
        for other in others {
            player.decoded_cards = other.decode_deck(player.decode_deck(player.decoded_cards.clone()));
        }
        player.decoded_cards = player.decode_deck(player.decoded_cards.clone());
        println!("{:#?}", player.hand);
    }
    table = players[players.len()-1].decode_deck(table);

    for (idx, player) in players.iter().enumerate() {
        for code in &player.decoded_cards {
            let description = player.map.get(code).unwrap();
            let description = format!("{}-{}", description.0, description.1);
            println!("{idx}: {description}");
        }
    }
    // for code in &table {
    //     let description =  players[0].map.get(code).unwrap();
    //     let description = format!("{}-{}", description.0, description.1);
    //     println!("Table: {description}");
    // }
}
