#![allow(dead_code)]
/*
Below is an example of how to handle errors and propagate the errors up to fn main().
 */
use std::{collections::HashMap, io};
use anyhow::Context;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{msg}")] // This is where we write the error message to be displayed
struct ParsePaymentInfoError {
    // We are using a struct in this case for we are looking for contents about the error
    // than what type of error it is.
    source: Option<anyhow::Error>,
    msg: String,
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse().with_context(|| format!("{s:?} could not be parsed as u32"))
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymentInfoError {
            source: Some(e),
            msg: format!("Failed to parse input as numbers. Input: {card}"),
        })?; // The ? propagates the error

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

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?; // .map_err converts one error type to another

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymentInfoError {
            source: None,
            msg: format!(
                "Incorrect number of elements parsed. Expected {expected_len} but get {len}. Elements: {numbers:?}."
            )
        });
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

#[derive(Error, Debug)]
enum CreditCardError {
    #[error("{0}")] // this will print the error inside the InvalidInput
    InvalidInput(String),
    #[error(transparent)] // The transparent keyword and #[from] indicate the anyhow::error will be
    // the source error and the display implementation should be forward to the source error.
    Other(#[from] anyhow::Error),
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or(
        CreditCardError::InvalidInput(format!("No credit card was found for {name}"))
    )?;

    let card = parse_card(card_string)
        .with_context(|| format!("{name}'s card could not be parsed."))
        .map_err(|e|{
            CreditCardError::Other(e)
        })?;

    Ok(card)
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),
        ("Bob", "1234567 Dec 27 123")
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
            match &err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_) => println!("\n Something went wrong! Try again.")
            }
            // log error
            log::error!("\n{err:?}");
            log::warn!("\n{err:?}");
            log::info!("\n{name}");
        }
    }
}
