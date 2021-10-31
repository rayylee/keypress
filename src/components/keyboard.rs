use std::collections::HashMap;

use yew::agent::Bridged;
use yew::services::ConsoleService;
use yew::{html, Bridge, Component, ComponentLink, Html, ShouldRender};

use crate::common::event_bus::EventBus;
use crate::common::msg::Key;
use crate::components::audioplayer::{AudioPlayer, Pronunc};

const DICT_PROGRAMMER: &str = include_str!("../content/dicts/it-words.json");
const DICT_CET4: &str = include_str!("../content/dicts/CET4_T.json");
const DICT_CET6: &str = include_str!("../content/dicts/CET6_T.json");
const DICT_TOEFL: &str = include_str!("../content/dicts/TOEFL_T.json");

lazy_static::lazy_static! {
    static ref DICT_INDEX: Vec<&'static str> = vec![
        "Programmer",
        "CET4",
        "CET6",
        "TOEFL",
    ];

    static ref DICT_MAP: HashMap<String, &'static str> =
    {
        let mut map = HashMap::new();
        map.insert(DICT_INDEX[0].to_string(), DICT_PROGRAMMER);
        map.insert(DICT_INDEX[1].to_string(), DICT_CET4);
        map.insert(DICT_INDEX[2].to_string(), DICT_CET6);
        map.insert(DICT_INDEX[3].to_string(), DICT_TOEFL);
        map
    };
}

pub struct Keyboard {
    start_status: String,
    start_class: String,
    dict: serde_json::Value,
    nr_word: usize,
    cur_index: usize,
    cur_level: String,
    cur_chaper: usize,
    inputs: String,
    _producer: Box<dyn Bridge<EventBus>>,
    link: ComponentLink<Self>,
    audio: AudioPlayer,
}

impl Keyboard {
    fn view_select_button(&self) -> Html {
        let chapers: Vec<usize> = (1..(self.nr_word / 20 + 1)).collect();

        html! {
            <>
               <div class="row justify-content-end">
                    <div class= "col-1">
                        <select class="form-control form-control-sm" id="exampleFormControlSelect2"
                        onchange=self.link.callback(| v:html::ChangeData | {
                            match v {
                                html::ChangeData::Select(ele) => {
                                    Key::SelectProunc(ele.value().parse::<u8>().unwrap_or(0))
                                }
                                _ => Key::SelectProunc(Pronunc::AmE as u8)
                            }
                        })>
                        {for Pronunc::iterator().map(|o| html!{<option value=(*o as u8).to_string()>{o}</option>} )}
                        </select>
                   </div>
                   <div class="col-6"></div>
                   <div class="col-2">
                   <select class="form-control form-control-sm" id="exampleFormControlSelect2"
                       onchange=self.link.callback(| v:html::ChangeData | {
                           match v {
                               html::ChangeData::Select(ele) => {
                                   Key::SelectLevel(ele.value())
                               }
                               _ => {
                                   Key::SelectLevel(DICT_INDEX[0].to_string())
                               }
                           }
                       } )>
                       { for DICT_INDEX.iter().map(|b| html! { <option value=*b>{ b }</option> }) }
                   </select>
                   </div>
                   <div class="col-2">
                   <select class="form-control form-control-sm" id="exampleFormControlSelect2"
                        onchange=self.link.callback(| v:html::ChangeData | {
                           match v {
                               html::ChangeData::Select(chp) => {
                                   Key::SelectChapter(chp.value().parse::<usize>().unwrap())
                               }
                               _ => {
                                   Key::SelectChapter(1)
                               }
                           }
                       } )>
                       {
                           for chapers.iter().map(|b| {
                               if *b == self.cur_chaper {
                                   html! { <option value=b.to_string() selected=true>{ format!("Charper {}", b) }</option> }
                               } else {
                                   html! { <option value=b.to_string()>{ format!("Charper {}", b) }</option> }
                               }
                           })
                       }
                   </select>
                   </div>
                   <div class="col-1">
                   <button onclick=self.link.callback(|_| Key::Submit) type="button" class=&self.start_class>
                       { &self.start_status }
                   </button>
                   </div>
                </div>
            </>
        }
    }

    fn view_word(&self) -> Html {
        let word = self.dict.get(self.cur_index).unwrap();
        let word_name: &str = word["name"].as_str().unwrap();
        let word_trans: &str = word["trans"][0].as_str().unwrap();
        let name_byte = word_name.as_bytes();
        let inputs_byte = self.inputs.as_bytes();
        let name_byte_last = &name_byte[inputs_byte.len()..name_byte.len()];

        html! {
            <>
                <div id="word">
                   { for inputs_byte.iter().map(|b| html! { <font color="#059669">{ *b as char }</font> }) }
                   { for name_byte_last.iter().map(|b| html! { <font color="#4B5563">{ *b as char }</font> }) }
                </div>
                <div id="trans">
                   <p> { &word_trans } </p>
                </div>
            </>
        }
    }

    fn view_bottom_button(&self) -> Html {
        html! {
            <>
                <div class="row">
                   <div class="col-2"></div>
                   <div class="col-2">
                   <button type="button" class="btn btn-outline-info"
                        onclick=self.link.callback(|_| Key::WordNextPre(String::from("prev")))>
                        { "Prev" }
                   </button>
                   </div>
                   <div class="col-6"></div>
                   <div class="col-2">
                   <button type="button" class="btn btn-outline-info"
                        onclick=self.link.callback(|_| Key::WordNextPre(String::from("next")))>
                        { "Next" }
                   </button>
                   </div>
                </div>
            </>
        }
    }
}

impl Component for Keyboard {
    type Message = Key;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cur_level = DICT_INDEX[0].to_string();
        let dict: serde_json::Value = serde_json::from_str(DICT_MAP[&cur_level]).unwrap();

        let nr_word: usize = dict.as_array().unwrap().len();
        let cur_index = 0;
        let cur_chaper = cur_index / 20 + 1;

        Self {
            dict: dict,
            nr_word: nr_word,
            cur_index: cur_index,
            cur_level: cur_level,
            cur_chaper: cur_chaper,
            inputs: String::with_capacity(100),
            start_status: String::from("Start"),
            start_class: String::from("btn btn-primary btn-sm"),
            _producer: EventBus::bridge(link.callback(Key::SetText)),
            link,
            audio: AudioPlayer::new(Pronunc::AmE),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Key::SetText(text) => {
                if self.start_status != String::from("Pause") || text.len() != 1 {
                    return true;
                }

                self.audio.play_click();

                let chr: char = text.as_bytes()[0] as char;
                self.inputs.push(chr);

                let word = self.dict[self.cur_index]["name"].as_str().unwrap();

                if word.starts_with(&self.inputs) {
                    if word.len() == self.inputs.len() {
                        self.inputs.clear();
                        self.cur_index = self.cur_index + 1;
                        if self.cur_index >= self.nr_word {
                            self.cur_index = 0;
                        }
                        self.cur_chaper = self.cur_index / 20 + 1;
                        self.audio.play_correct();
                        self.audio
                            .play_word(self.dict[self.cur_index]["name"].as_str().unwrap());
                    }
                } else {
                    self.audio.play_wrong();
                    self.inputs.clear();
                }
            }
            Key::SelectProunc(prounc) => {
                self.audio.set_prounc(prounc.into());
                let msg = format!("> select audio type: {}.", self.audio.prounc());
                ConsoleService::debug(&msg);
                self.audio
                    .play_word(self.dict[self.cur_index]["name"].as_str().unwrap());
            }
            Key::SelectLevel(level) => {
                self.cur_level = level;
                let msg = format!("> select level: {}.", self.cur_level);
                ConsoleService::debug(&msg);

                self.dict = serde_json::from_str(DICT_MAP[&self.cur_level]).unwrap();
                self.nr_word = self.dict.as_array().unwrap().len();
                self.inputs.clear();
                self.cur_index = 0;
                self.cur_chaper = self.cur_index / 20 + 1;
            }
            Key::SelectChapter(chaper) => {
                self.cur_index = (chaper - 1) * 20;
                let msg = format!("> select chaper: {}.", chaper);
                ConsoleService::debug(&msg);
            }
            Key::WordNextPre(text) => {
                if self.start_status == String::from("Start") {
                    return true;
                }
                self.inputs.clear();
                if text == String::from("next") {
                    self.cur_index = self.cur_index + 1;
                    if self.cur_index >= self.nr_word {
                        self.cur_index = 0;
                    }
                } else {
                    if self.cur_index <= 0 {
                        self.cur_index = self.nr_word - 1;
                    } else {
                        self.cur_index = self.cur_index - 1;
                    }
                }
                self.cur_chaper = self.cur_index / 20 + 1;

                self.audio
                    .play_word(self.dict[self.cur_index]["name"].as_str().unwrap());
            }
            Key::Submit => {
                if self.start_status == String::from("Start") {
                    self.start_status = String::from("Pause");
                    self.start_class = String::from("btn btn-secondary btn-sm");

                    self.audio
                        .play_word(self.dict[self.cur_index]["name"].as_str().unwrap());
                } else {
                    self.start_status = String::from("Start");
                    self.start_class = String::from("btn btn-primary btn-sm");
                }
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="container-fluid">
                    { self.view_select_button() }
                </div>
                { self.view_word() }
                <div class="container-fluid">
                    { self.view_bottom_button() }
                </div>
            </>
        }
    }
}
