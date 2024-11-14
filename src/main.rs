use log::info;
use std::fmt;
use std::hash::Hash;
use std::iter::{IntoIterator, Iterator};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct SearchBoxProps {
    on_input: Callback<String>,
}

#[function_component(SearchBox)]
fn search_box(SearchBoxProps { on_input }: &SearchBoxProps) -> Html {
    let on_input = on_input.clone();
    let name = use_state(|| String::new());
    let oninput = Callback::from({
        let name = name.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            name.set(target.value());
            
            on_input.emit(target.value())
        }
    });
    html! {
        <input type="search" oninput={oninput} id="name" data-lpignore="true" autocomplete="off" name="name" minlength="0" maxlength="20" size="20"/>
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Card {
    id: usize,
    name: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Properties, PartialEq)]
struct CardsListProps {
    cards: Vec<Card>,
}

#[function_component(CardsList)]
fn cards_list(CardsListProps { cards }: &CardsListProps) -> Html {
    cards
        .iter()
        .map(|card| {
            html! {
                <p key={card.id}>{format!("{}", card.name)}</p>
            }
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let cards = vec![
        Card {
            id: 0,
            name: "Alexis".to_string(),
        },
        Card {
            id: 1,
            name: "Camille".to_string(),
        },
        Card {
            id: 2,
            name: "Jérémy".to_string(),
        },
    ];

    let typed = use_state(|| None);
    let mut filtered_cards = use_state(|| None);

    let on_type = {
        let typed = typed.clone();
        let mut filtered_cards = filtered_cards.clone();
        let cards = cards.clone();
        Callback::from(move |text: String| {
            let cards_found: Vec<Card> = cards
                .clone()
                .into_iter()
                .filter(|card| card.name.to_lowercase().contains(&text))
                .collect();
            typed.set(Some(text));
            filtered_cards.set(Some(cards_found));
        })
    };

    let typed_details = typed.as_ref().map(|i| {
        html! {
            <p>{ format!("Searching for : {}" ,i) }</p>
        }
    });

    let items = filtered_cards.as_ref().unwrap_or(&cards);

    let cards_details = html! {
        <div id="cards">
            {
                items.into_iter().map(|card| {
                    html!{<div key={card.id}>{ format!("{}",card.name) }</div>}
                }).collect::<Html>()
            }
        </div>
    };

    html! {
        <>
            <div>
                <SearchBox on_input={on_type.clone()} />
            </div>
            { for typed_details }
            { cards_details }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
