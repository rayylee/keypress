use std::{
    fmt::{Display, Formatter, Result},
    slice::Iter,
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{
    AudioBuffer, AudioContext, AudioDestinationNode, HtmlAudioElement,
};


const SOUND_CLICK: &[u8] = include_bytes!("../content/sound/click.wav");
const SOUND_CORRECT: &[u8] = include_bytes!("../content/sound/correct.mp3");
const SOUND_WRONG: &[u8] = include_bytes!("../content/sound/wrong.mp3");

macro_rules! audio_url{
    ($($arg:tt)*) => {
        format!("http://dict.youdao.com/dictvoice?type={prounc}&audio={word}", $($arg)*)
    };
}

#[derive(Clone, Copy)]
pub enum Pronunc {
    AmE = 0,
    BrE = 1,
}

impl Pronunc {
    pub fn iterator() -> Iter<'static, Pronunc> {
        static PRONUNC_S: [Pronunc; 2] = [Pronunc::AmE, Pronunc::BrE];
        PRONUNC_S.iter()
    }
}

impl Display for Pronunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Pronunc::AmE => write!(f, "Amercan pronunciations"),
            Pronunc::BrE => write!(f, "British pronunciations"),
        }
    }
}

impl From<u8> for Pronunc {
    fn from(v: u8) -> Pronunc {
        match v {
            0 => Pronunc::AmE,
            _ => Pronunc::BrE,
        }
    }
}

pub struct AudioPlayer {
    prounc: Pronunc, // Switch Amercan English and British English
    audio_ctx: AudioContext,
}

impl AudioPlayer {
    fn play_audio_from_url(&self, url: &str) {
        // https://docs.rs/web-sys/0.3.19/web_sys/struct.HtmlAudioElement.html
        let audio = HtmlAudioElement::new_with_src(url).unwrap();
        let _ = audio.play().unwrap();
    }

    fn play_audio_from_array(&self, array: &'static [u8]) {
        let array_u8: js_sys::Uint8Array = js_sys::Uint8Array::from(array);
        let array_buf: js_sys::ArrayBuffer = array_u8.buffer();

        // https://docs.rs/web-sys/0.3.19/web_sys/struct.AudioContext.html

        let destination: AudioDestinationNode = self.audio_ctx.destination();
        let song = self.audio_ctx.create_buffer_source().unwrap();

        let handler = move |buf: AudioBuffer| {
            let buffer: Option<&AudioBuffer> = Some(&buf);

            song.set_buffer(buffer);
            song.connect_with_audio_node(destination.as_ref()).unwrap();
            song.start().unwrap();
        };

        // shoud be 'static
        let handle: Box<dyn FnMut(_) + 'static> = Box::new(handler) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handle);

        let _ = self
            .audio_ctx
            .decode_audio_data_with_success_callback(&array_buf, cb.as_ref().unchecked_ref())
            .unwrap();

        // don't forget
        cb.forget();
    }
}

impl AudioPlayer {
    pub fn new(prounc: Pronunc) -> Self {
        let audio_ctx = {
            let audio_ctx = AudioContext::new().unwrap();
            audio_ctx
        };
        AudioPlayer { prounc, audio_ctx }
    }

    pub fn prounc(&self) -> &Pronunc {
        &self.prounc
    }

    pub fn set_prounc(&mut self, prounc: Pronunc) {
        self.prounc = prounc;
    }

    pub fn play_correct(&self) {
        self.play_audio_from_array(SOUND_CORRECT);
    }

    pub fn play_wrong(&self) {
        self.play_audio_from_array(SOUND_WRONG);
    }

    pub fn play_word(&self, word: &str) {
        let word_url = audio_url!(prounc = self.prounc as u8, word = word);
        self.play_audio_from_url(&word_url);
    }

    pub fn play_click(&self) {
        self.play_audio_from_array(SOUND_CLICK);
    }
}
