use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ReqStatus<T>{
    ok: bool,
    result: T
}

#[derive(Serialize, Deserialize)]
struct BotInfo{
    id: u64,
    is_bot: bool,
    first_name: String,
    username: String,
    can_join_groups: bool,
    can_read_all_group_messages: bool,
    supports_inline_queries: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BotUpdate{
    update_id: u64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<Message>,
    pub inline_query: Option<InlineQuery>,
    pub callback_query: Option<CallBackQuery>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message{
    message_id: u64,
    date: u64,
    pub chat: Chat,
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat{
    pub id: u64,
    #[serde(rename(deserialize = "type"))]
    chat_type: String,

}

#[derive(Serialize, Deserialize, Clone)]
pub struct User{
    pub id: u64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InlineQuery{
    pub id: String,
    pub from: User,
    pub query: String,
    offset: String,
    pub chat_type: Option<String>
}

#[derive(Serialize)]
struct AnswerInlineQuery{
    inline_query_id: String,
    results: Vec<InlineQueryResult>
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum InlineQueryResult{
    Article(InlineQueryResultArticle),
    Voice(InlineQueryResultVoice)
}

#[derive(Serialize)]
pub struct InlineQueryResultArticle{
    #[serde(rename(serialize = "type"))]
    pub query_type: String,
    id: u8,
    title: String,
    input_message_content: InputMessageContent
}

#[derive(Serialize)]
pub struct InlineQueryResultVoice{
    #[serde(rename(serialize = "type"))]
    pub query_type: String,
    id: u8,
    voice_url: String,
    title: String, // name of recording
    #[serde(skip_serializing_if="Option::is_none")]
    caption: Option<String>,
    // #[serde(skip_serializing_if="Option::is_none")]
    // parse_mode: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallBackQuery{
    id: String,
    from: User,
    pub message: Option<Message>,
    inline_message_id: Option<String>,
    chat_instance: String,
    pub data: Option<String>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum InputMessageContent{
    Text(InputTextMessageContent),
    Location,
    Venue,
    Contact,
    Invoice
}

#[derive(Serialize)]
pub struct InputTextMessageContent{
    message_text: String,
    // parse_mode: String,
    // entities
    disable_web_page_preview: bool
}

#[derive(Serialize, Deserialize)]
struct SendMessage<T> {
    chat_id: u64,
    text: T,
    #[serde(skip_serializing_if="Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if="Option::is_none")]
    parse_mode: Option<T>
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup{
    InlineKeyboard(InlineKeyboardMarkup)
}

#[derive(Serialize, Deserialize)]
struct InlineKeyboardMarkup{
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>
}

#[derive(Serialize, Deserialize)]
struct InlineKeyboardButton{
    text: String,
    #[serde(skip_serializing_if="Option::is_none")]
    url: Option<String>,
    callback_data: Option<String>
}

