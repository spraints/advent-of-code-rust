//use proc_macro::{Ident, Punct, TokenTree};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn aoc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // let mut year = None;
    // let mut day = None;
    // let mut part = None;
    // let mut label = None;
    // //let mut fnname = None;
    // #[derive(Debug)]
    // enum State {
    //     Idle,
    //     EqAfterYear,
    //     YearValue,
    //     EqAfterDay,
    //     DayValue,
    //     EqAfterPart,
    //     PartValue,
    //     EqAfterLabel,
    //     LabelValue,
    // }
    // let mut st = State::Idle;
    // for t in attr {
    //     st = match (st, t) {
    //         (State::Idle, TokenTree::Ident(i)) if i.to_string() == "year" => State::EqAfterYear,
    //         (State::Idle, TokenTree::Ident(i)) if i.to_string() == "day" => State::EqAfterDay,
    //         (State::Idle, TokenTree::Ident(i)) if i.to_string() == "part" => State::EqAfterPart,
    //         (State::Idle, TokenTree::Ident(i)) if i.to_string() == "label" => State::EqAfterLabel,
    //         (State::Idle, TokenTree::Punct(p)) if p.as_char() == ',' => State::Idle,
    //         (State::EqAfterYear, TokenTree::Punct(p)) if p.as_char() == '=' => State::YearValue,
    //         (State::EqAfterDay, TokenTree::Punct(p)) if p.as_char() == '=' => State::DayValue,
    //         (State::EqAfterPart, TokenTree::Punct(p)) if p.as_char() == '=' => State::PartValue,
    //         (State::EqAfterLabel, TokenTree::Punct(p)) if p.as_char() == '=' => State::LabelValue,
    //         (State::YearValue, TokenTree::Literal(l)) => {
    //             year = Some(l);
    //             State::Idle
    //         }
    //         (State::DayValue, TokenTree::Literal(l)) => {
    //             day = Some(l);
    //             State::Idle
    //         }
    //         (State::PartValue, TokenTree::Literal(l)) => {
    //             part = Some(l);
    //             State::Idle
    //         }
    //         (State::LabelValue, TokenTree::Literal(l)) => {
    //             label = Some(l);
    //             State::Idle
    //         }
    //         (st, t) => panic!("Bad syntax: st={:?} token={:?}", st, t),
    //     };
    // }
    // // Make a 'register' call.
    item
}
