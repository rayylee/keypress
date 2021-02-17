pub enum Msg {
    ButtonStart,
    UpdateTime,
}

pub enum Key {
    SetText(String),
    SelectLevel(String),
    SelectChapter(usize),
    WordNextPre(String),
    Submit,
}
