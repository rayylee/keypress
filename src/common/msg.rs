pub enum Msg {
    UpdateTime,
}

pub enum Key {
    SetText(String),
    SelectProunc(u8),
    SelectLevel(String),
    SelectChapter(usize),
    WordNextPre(String),
    Submit,
}
