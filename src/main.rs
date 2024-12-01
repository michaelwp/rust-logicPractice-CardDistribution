/*
   Card distribution logic:
   - Number of cards in a deck is 52.
   - Card consist of 4 types of card (Hearts, Diamonds, Spades and Clubs).
   - Every type of card consist of 13 cards (A, 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K).
   - The cards will be distributed to "n" players.
   - The cards will be distributed in round-robin way.
   - Before distributed to players, the cards will be shuffled.
*/

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;
#[derive(Debug, Clone)]
struct Card {
    c_number: u32,
    c_type: u32,
}

#[derive(Debug)]
struct Player {
    p_name: String,
    p_card: Vec<Card>,
}

static DECK_OF_CARD: Lazy<Mutex<Vec<Card>>> = Lazy::new(|| Mutex::new(Vec::new()));
static CARD_PLAYERS: Lazy<Mutex<Vec<Player>>> = Lazy::new(|| Mutex::new(Vec::new()));

lazy_static! {
    static ref CARD_TYPE: HashMap<u32, &'static str> = {
        let mut map: HashMap<u32, &str> = HashMap::new();
        map.insert(1, "Heart");
        map.insert(2, "Spade");
        map.insert(3, "Diamond");
        map.insert(4, "Club");
        map
    };
}

lazy_static! {
    static ref CARD_NUMBER: HashMap<u32, &'static str> = {
        let mut map: HashMap<u32, &str> = HashMap::new();
        map.insert(1, "Ace");
        map.insert(2, "Two");
        map.insert(3, "Three");
        map.insert(4, "Four");
        map.insert(5, "Five");
        map.insert(6, "Six");
        map.insert(7, "Seven");
        map.insert(8, "Eight");
        map.insert(9, "Nine");
        map.insert(10, "Ten");
        map.insert(11, "Jack");
        map.insert(12, "Queen");
        map.insert(13, "King");
        map
    };
}

fn main() {
    // set up the deck of card
    setup_deck_of_card();

    // shuffle the card
    shuffle_the_card();

    // set up the players
    setup_the_player(4);

    // distribute the card
    distribute_the_card();

    // print the deck of card
    let deck = DECK_OF_CARD.lock().unwrap();
    for card in deck.iter() {
        println!("{} {}", CARD_NUMBER[&card.c_number], CARD_TYPE[&card.c_type]);
    }

    // print the player
    {
        let players = CARD_PLAYERS.lock().unwrap();
        for player in players.iter() {
            println!("=====================");
            println!("{} :", player.p_name);

            // print the player's card
            for card in player.p_card.iter() {
                println!("{} {}", CARD_NUMBER[&card.c_number], CARD_TYPE[&card.c_type]);
            }
        }
    }
}

fn setup_deck_of_card() {
    let mut deck = DECK_OF_CARD.lock().unwrap();
    for c_type in CARD_TYPE.keys() {
        for c_number in 1..14 {
            deck.push(Card {
                c_number: c_number,
                c_type: *c_type,
            });
        }
    }
}

fn shuffle_the_card() {
    let mut deck = DECK_OF_CARD.lock().unwrap();
    let mut rng = rand::thread_rng();

    for idx in 0..deck.len() {
        // random number 0 to deck.len() exclusive range
        let random_number = rng.gen_range(0..deck.len());

        // swap the card current position with the random number
        deck.swap(idx, random_number);
    }
}

fn setup_the_player(player_number:usize) {
    let mut players = CARD_PLAYERS.lock().unwrap();
    for p_number in 1..=player_number {
        players.push(Player{
            p_name: format!("Player {}", p_number),
            p_card: Vec::new(),
        });
    }
}

fn distribute_the_card() {
    if CARD_PLAYERS.lock().unwrap().len() <= 0 {
        println!("There are no players to distribute !!");
        return
    }

    let deck = DECK_OF_CARD.lock().unwrap();
    let mut players = CARD_PLAYERS.lock().unwrap();

    for idx in 0..deck.len() {
        let player_index = idx % players.len();

        players[player_index].p_card.push(deck[idx].clone());
    }
}