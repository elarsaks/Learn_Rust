#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let values = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];

    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck: Deck = Deck { cards };

    println!("Hello, world! {:#?}", deck);
}
