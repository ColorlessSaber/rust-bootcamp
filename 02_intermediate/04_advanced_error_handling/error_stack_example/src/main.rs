#![allow(dead_code)]

use std::{collections::HashMap, io, fmt, error::Error};
use error_stack::{Report, ResultExt}; // The error_stack's Result is deprecated; have to use Result from core.
use core::result::Result;

#[derive(Debug)]
struct ParsePaymentInfoError;

impl fmt::Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Parse payment info error")
    }
}

impl Error for ParsePaymentInfoError {}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, Report<ParsePaymentInfoError>> {
    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| {
            s.parse()
                .attach_with(|| {
                    format!("{s:?} could not be parsed as u32")
                })
        })
        .collect::<Result<Vec<u32>, _>>()
        .change_context(ParsePaymentInfoError)
        .attach(format!("Failed to parse input as numbers. Input: {card}"))?;

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, Report<ParsePaymentInfoError>> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(Report::new(ParsePaymentInfoError)
            .attach(format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}")));
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other,
}

impl fmt::Display for CreditCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for CreditCardError {}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, Report<CreditCardError>> {
    let card_string = credit_cards.get(name).ok_or_else(|| {
        let msg = format!("No credit card was found for {name}.");
        Report::new(CreditCardError::InvalidInput(msg.clone()))
            .attach(msg.clone())
    })?;

    let card = parse_card(card_string)
        .change_context(CreditCardError::Other)
        .attach(format!("{name}'s card could not be parsed."))?;

    Ok(card)
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),
        ("Bob", "1234567 Dec 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());
    match result {
        Ok(card) => {
            println!("\nCredit Card Info: {card:?}");
        },
        Err(err) => {
            match err.current_context() {
                CreditCardError::InvalidInput(msg) => println!("\n{msg}"),
                CreditCardError::Other => println!("\nSomething went wrong! Try again!"),
            }
            log::error!("\n{err:?}");
        }
    }
}