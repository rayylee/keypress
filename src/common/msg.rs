pub enum Msg {
    UpdateTime,
}

pub enum Key {
    SetText(String),
    SelectLevel(String),
    SelectChapter(usize),
    WordNextPre(String),
    Submit,
}
