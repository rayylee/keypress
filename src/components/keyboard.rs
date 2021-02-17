use std::collections::HashMap;

use yew::{html, Bridge, Component, ComponentLink, Html, ShouldRender};
use yew::services::{ConsoleService};
use yew::agent::Bridged;
use web_sys::{HtmlAudioElement, AudioBuffer, AudioContext, AudioDestinationNode};
use wasm_bindgen::{JsCast, prelude::Closure};

use crate::common::msg::Key;
use crate::common::event_bus::{EventBus};

static AUDIO_URL: &str = "http://dict.youdao.com/dictvoice?type=0&audio=";

const SOUND_CLICK: &[u8] = include_bytes!("../content/sound/click.wav");

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
    start_style: String,
    dict: serde_json::Value,
    nr_word: usize,
    cur_index: usize,
    cur_level: String,
    cur_chaper: usize,
    inputs: String,
    _producer: Box<dyn Bridge<EventBus>>,
    link: ComponentLink<Self>,
}

impl Component for Keyboard {
    type Message = Key;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let cur_level = DICT_INDEX[0].to_string();
        let dict: serde_json::Value =
            serde_json::from_str(DICT_MAP[&cur_level]).unwrap();

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
            start_style: String::from("background-color:#F5F5F5"),
            _producer: EventBus::bridge(link.callback(Key::SetText)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
	match msg {
	    Key::SetText(text) => {
                if self.start_status != String::from("Pause") || text.len() != 1 {
                    return true;
                }

                let array_u8: js_sys::Uint8Array = js_sys::Uint8Array::from(SOUND_CLICK);

                let array_buf: js_sys::ArrayBuffer = array_u8.buffer();

                let audio_ctx = AudioContext::new().unwrap();

                let song = audio_ctx.create_buffer_source().unwrap();
                let destination: AudioDestinationNode = audio_ctx.destination();

                let handler = move |buf: AudioBuffer| {
                    let buffer: Option<&AudioBuffer> = Some(&buf);

                    song.set_buffer(buffer);
                    song.connect_with_audio_node(destination.as_ref()).unwrap();
                    song.start().unwrap();
                };

                let handle: Box<dyn FnMut(_) + 'static> = Box::new(handler) as Box<dyn FnMut(_)>;

                let cb = Closure::wrap(handle);

                audio_ctx.decode_audio_data_with_success_callback(&array_buf,
                    cb.as_ref().unchecked_ref()).unwrap();

                cb.forget();

                let b = text.as_bytes()[0];
                let c: char = b as char;
                self.inputs.push(c);

                let word = self.dict[self.cur_index]["name"].as_str().unwrap();

                let mut need_play = false;
                if word.starts_with(&self.inputs) {
                    if word.len() == self.inputs.len() {
                        self.inputs.clear();
                        self.cur_index = self.cur_index + 1;
                        if self.cur_index >= self.nr_word {
                            self.cur_index = 0;
                        }
                        self.cur_chaper = self.cur_index / 20 + 1;
                        need_play = true;

                    }
                } else {
                    self.inputs.clear();
                }

                if need_play {
                    let word = self.dict[self.cur_index]["name"].as_str().unwrap();
                    let word_url = AUDIO_URL.to_string() + &word.to_string();
                    let audio = HtmlAudioElement::new_with_src(word_url.as_str()).unwrap();
                    audio.play().unwrap();
                }

                let word = self.dict[self.cur_index]["name"].as_str().unwrap();
		let msg = format!("> key:{} for:{} inputs:{}, level:{}, chaper:{}, words:{}.",
		    text, word, self.inputs, self.cur_level, self.cur_chaper, self.nr_word);
                ConsoleService::info(&msg);
	    }
	    Key::SelectLevel(level) => {
                self.cur_level = level;
                let msg = format!("> select level: {}.", self.cur_level);
                ConsoleService::info(&msg);

                self.dict = serde_json::from_str(DICT_MAP[&self.cur_level]).unwrap();
                self.nr_word = self.dict.as_array().unwrap().len();
                self.inputs.clear();
                self.cur_index = 0;
                self.cur_chaper = self.cur_index / 20 + 1;
            }
	    Key::SelectChapter(chaper) => {
                self.cur_index = (chaper - 1 ) * 20;
                let msg = format!("> select chaper: {}.", chaper);
                ConsoleService::info(&msg);
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
                        self.cur_index = self.cur_index -1;
                    }
                }
                self.cur_chaper = self.cur_index / 20 + 1;

                let word = self.dict[self.cur_index]["name"].as_str().unwrap();
                let word_url = AUDIO_URL.to_string() + &word.to_string();
                let audio = HtmlAudioElement::new_with_src(word_url.as_str()).unwrap();
                audio.play().unwrap();

                let msg = format!("> level:{}, chaper:{}, index:{}, words:{}.",
                    self.cur_level, self.cur_chaper, self.cur_index, self.nr_word);
                ConsoleService::info(&msg);
            }
	    Key::Submit => {
                if self.start_status == String::from("Start") {
                    self.start_status = String::from("Pause");
                    self.start_style = String::from("background-color:#008f53");

                    let word = self.dict[self.cur_index]["name"].as_str().unwrap();
                    let word_url = AUDIO_URL.to_string() + &word.to_string();
                    let audio = HtmlAudioElement::new_with_src(word_url.as_str()).unwrap();
                    audio.play().unwrap();
                } else {
                    self.start_status = String::from("Start");
                    self.start_style = String::from("background-color:#F5F5F5");
                }
                ConsoleService::info("> window key [enter] pressed.");
	    }
	}

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let word = self.dict.get(self.cur_index).unwrap();
        let word_name: &str = word["name"].as_str().unwrap();
        let word_trans: &str = word["trans"][0].as_str().unwrap();
        let name_byte = word_name.as_bytes();
        let inputs_byte = self.inputs.as_bytes();
        let name_byte_last = &name_byte[inputs_byte.len()..name_byte.len()];

        let chapers: Vec<usize> = (1..(self.nr_word / 20 + 1)).collect();

        html! {
            <>
               <div id="buttons">
                   <select onchange=self.link.callback(| v:html::ChangeData | {
                           match v {
                               html::ChangeData::Select(ele) => {
                                   Key::SelectLevel(ele.value())
                               }
                               _ => {
                                   Key::SelectLevel(DICT_INDEX[0].to_string())
                               }
                           }
                       } )>
                       { for DICT_INDEX.iter().map(|b| html! { <option value=b>{ b }</option> }) }
                   </select>
                   <select onchange=self.link.callback(| v:html::ChangeData | {
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
                                   html! { <option value=b selected=true>{ format!("Charper {}", b) }</option> }
                               } else {
                                   html! { <option value=b>{ format!("Charper {}", b) }</option> }
                               }
                           })
                       }
                   </select>
                   <button onclick=self.link.callback(|_| Key::Submit) style=self.start_style>
                       { &self.start_status }
                   </button>
               </div>
                <div id="word">
                   { for inputs_byte.iter().map(|b| html! { <font color="red">{ *b as char }</font> }) }
                   { for name_byte_last.iter().map(|b| html! { <font color="white">{ *b as char }</font> }) }
                </div>
                <div id="trans">
                   <p> { &word_trans } </p>
                </div>
                <div>
                   <button style="float:left;" onclick=self.link.callback(|_| Key::WordNextPre(String::from("prev")))>
                       { "Prev" }
                   </button>
                   <button style="float:right;" onclick=self.link.callback(|_| Key::WordNextPre(String::from("next")))>
                       { "Next" }
                   </button>
                </div>
            </>
        }
    }
}

