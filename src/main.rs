use hiragana::{Hiragana, picker::{pick_hiragana, pick_hiragana_exclude_existing}};
use rand::{prelude::ThreadRng, seq::IteratorRandom, thread_rng};
use web_sys::{FocusEvent, Url};
use yew::{html, Component, InputData, Properties};

mod hiragana;

struct Answer {
    hiragana: &'static Hiragana<'static>,
    answer: String,
}

enum Msg {
    UpdateAnswer(String),
    Answer,
}

struct Model {
    previous_answer: Option<Answer>,
    current_hiragana: &'static Hiragana<'static>,
    hiragana: Vec<&'static Hiragana<'static>>,
    rng: ThreadRng,
    link: yew::ComponentLink<Self>,
    answer_text: String,
}

#[derive(Properties, Clone)]
struct Props {
    hiragana: Vec<&'static Hiragana<'static>>,
    rng: ThreadRng,
}

impl<'a> Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(mut props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            current_hiragana: pick_hiragana(&props.hiragana, &mut props.rng),
            hiragana: props.hiragana,
            previous_answer: None,
            rng: props.rng,
            link,
            answer_text: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::UpdateAnswer(answer) => {
                self.answer_text = answer;
                false
            }
            Msg::Answer => {
                let prev_answer = std::mem::replace(&mut self.answer_text, "".to_string());
                self.previous_answer = Some(Answer {
                    hiragana: self.current_hiragana,
                    answer: prev_answer,
                });
                self.current_hiragana = pick_hiragana_exclude_existing(&self.hiragana, &mut self.rng, self.current_hiragana);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.hiragana = props.hiragana;
        self.current_hiragana = pick_hiragana(&self.hiragana, &mut self.rng);
        self.answer_text = "".to_string();
        true
    }

    fn view(&self) -> yew::Html {
        let form_submit_handler = self.link.batch_callback(|e: FocusEvent| {
            e.stop_propagation();
            e.prevent_default();
            None
        });
        let answer = self
            .previous_answer
            .as_ref()
            .map(|previous_answer| {
                format!(
                    "{}: {} (correct answer: {})",
                    previous_answer.hiragana.jpn,
                    previous_answer.answer,
                    previous_answer.hiragana.eng
                )
            })
            .unwrap_or("".to_string());
        html! {
            <div>
                <div>
                    {answer}
                </div>
                <form onsubmit=form_submit_handler>
                    {self.current_hiragana.jpn}
                    <input type="text" value={self.answer_text.clone()} oninput=self.link.callback(|e: InputData| Msg::UpdateAnswer(e.value))/>
                    <input type="submit" onclick=self.link.callback(|_|Msg::Answer)/>
                </form>
            </div>
        }
    }
}

fn main() {
    let size = Url::new(&web_sys::window().unwrap().location().href().unwrap())
        .unwrap()
        .search_params()
        .get("count")
        .map(|count| {
            count
                .parse::<usize>()
                .expect("Count param should be a positive number")
        })
        .unwrap_or(5);
    let single_char_hiragana = hiragana::data::SYMBOLS
        .iter()
        .filter(|h| h.jpn.chars().count() < 2);
    let mut rng = thread_rng();
    yew::start_app_with_props::<Model>(Props {
        hiragana: single_char_hiragana.choose_multiple(&mut rng, size),
        rng,
    });
}
