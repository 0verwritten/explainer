// use crate::botlib::InlineQueryResult;
pub mod datatypes;
// use crate::datatypes::*;
// use datatypes::*;
include!("datatypes.rs");

use colored::*;
use std::collections::HashMap;
use std::thread;
use std::sync::Arc;
use reqwest::multipart::{ Form, Part };


pub struct Telebotapi {
    api: String,
    pub handle: Option<HashMap<MessageType, fn(String, Arc<BotUpdate>) -> ()>>
}

#[derive(Debug)]
pub enum BotError{
    NetworkError(String),
    ServerError,
    BotDoesntExist,
    InternalError(String)
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum MessageType{
    Text(String),
    InlineQuery,
    CallBackQuery(String)
}

impl Telebotapi {
    pub async fn new(api: &str) -> Result<Self, BotError> {
        let req = reqwest::get(&format!("https://api.telegram.org/bot{}/getMe", api))
            .await.expect("Could not send request to check api validness")
                .json::<ReqStatus<BotInfo>>().await.unwrap();
        
        println!( "Bot {} status: {}", req.result.first_name.bold(), if req.ok {"ok".green()} else {"bad".red()} );

        if !req.ok{
            return Err(BotError::BotDoesntExist)
        }
        Ok( Telebotapi { api: api.to_string(), handle: None } )
    }

    pub async fn listen(&self) -> Result<(), BotError> {

        let mut last_msg = 0;
        loop {
            let req = reqwest::get(&format!("https://api.telegram.org/bot{}/getUpdates?offset={}", self.api, last_msg+1))
                .await.expect("Could not send request to check api validness")
                    .json::<ReqStatus<Vec<BotUpdate>>>().await.unwrap();

            if req.result.len() > 0 {
                last_msg = req.result.last().unwrap().update_id;
                for request in req.result{
                    if let Some(handle) = &self.handle{
                        let mut count = 0;
                        for (_msg, func) in handle.iter().filter( |(msg, _)| msg == &&request ) {
                            let api = self.api.to_string();
                            let last_rec = Arc::new(request.clone());
                            let func = func.clone();
                            thread::spawn( move || func(api, last_rec).clone() );
                            count += 1;
                        }
                        if count == 0{
                            if !request.message.is_none(){
                                if let Some(func) = handle.get(&MessageType::Text(String::new())){
                                    let api = self.api.to_string();
                                    let func = func.clone();
                                    thread::spawn( move || func(api, Arc::new(request.clone())));
                                }
                            } else if !request.callback_query.is_none(){
                                if let Some(func) = handle.get(&MessageType::CallBackQuery(String::new())){
                                    let api = self.api.to_string();
                                    let func = func.clone();
                                    thread::spawn( move || func(api, Arc::new(request.clone())));
                                }
                            }
                        }
                    }

                    else{
                        let _ = reqwest::Client::new().post(&format!("https://api.telegram.org/bot{}/sendMessage", self.api))
                            .json( &SendMessage { chat_id: 1895656874, text: request.message.as_ref(), parse_mode: None, reply_markup: Some(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup{ inline_keyboard: vec![vec![InlineKeyboardButton{ text: String::from("abcdd"), callback_data: Some(String::from("test_app_bot")), url: None } ]] })) } )
                            .send().await.unwrap();
                    }
                }
            }
        }
    }

    pub fn add_handler(&mut self, message: MessageType, func: fn(String, Arc<BotUpdate>) -> ()) {
        let handle = self.handle.get_or_insert(HashMap::new());
        handle.insert(message, func);
    }
    pub async fn send_text(api: String, chat_id: u64, text: &str, parse_mode: Option<&str>, disable_web_page_preview: bool, disable_notification: bool, reply_to_message_id: bool, allow_sending_without_reply: bool, reply_markup: Option<ReplyMarkup>) -> Result<(), BotError> {
        let responce = reqwest::Client::new().post(format!("https://api.telegram.org/bot{}/sendMessage", api))
                .json( &SendMessage {chat_id: chat_id, text: text, reply_markup: reply_markup, parse_mode: parse_mode} )
                .send().await.unwrap();
        match responce.status(){
            code if code.is_success() => Ok(()),
            code if code.is_server_error() => Err(BotError::ServerError),
            code => { eprint!("{}", responce.text().await.unwrap()); Err(BotError::NetworkError(code.to_string()))}
        }
    }
    pub async fn send_voice(api: String, chat_id: u64, voice: Vec<u8>, caption: &str, parse_mode: Option<&str>, reply_markup: Option<ReplyMarkup>) -> Result<(), BotError> {
        if voice.len() == 0 {
            Telebotapi::send_text(api, chat_id, caption, parse_mode, true, false, false, true, reply_markup).await.unwrap();

            return Ok(());
        }
        let _responce  = reqwest::Client::new().post(format!("https://api.telegram.org/bot{}/sendVoice", api))
                .multipart( Form::new()
                                .text("chat_id", chat_id.to_string())
                                .text("caption", caption.to_string())
                                .part("voice", Part::bytes(voice).file_name("voice.ogg").mime_str("audio/ogg").unwrap())
                            ).send().await.unwrap();

        Ok(())
    }

    pub async fn send_audio(api: String, chat_id: u64, audio: &str, caption: &str, parse_mode: Option<&str>) -> Result<(), BotError> {
        let _responce  = reqwest::Client::new().post(format!("https://api.telegram.org/bot{}/sendAudio", api))
                .multipart( Form::new()
                                .text("chat_id", chat_id.to_string())
                                .text("caption", caption.to_string())
                                .text("audio", audio.to_string())
                            ).send().await.unwrap();

        Ok(())
    }

    // pub async fn send_photo(api: String, chat_id: u64, photo: &str){
    pub async fn send_photo(api: String, chat_id: u64, photo: Vec<u8>){ // photo - bytes of the file
        let responce  = reqwest::Client::new().post(format!("https://api.telegram.org/bot{}/sendVoice", api))
                .multipart( Form::new()
                                .text("chat_id", chat_id.to_string())
                                .part("photo", Part::bytes(photo).mime_str("image/jpeg").unwrap().file_name("some_img.jpg"))
                            ).send().await.unwrap();

        eprint!("{}", responce.text().await.unwrap());
    }

    pub async fn answer_query(api: String, inline_query_id: String, results: Vec<InlineQueryResult>) -> Result<(), BotError> {
        let responce = reqwest::Client::new().post(format!("https://api.telegram.org/bot{}/answerInlineQuery", api))
                .json( &AnswerInlineQuery {inline_query_id: inline_query_id, results: results} )
                .send().await.unwrap();
            

        match responce.status(){
            code if code.is_success() => Ok(()),
            code if code.is_server_error() => Err(BotError::ServerError),
            code => { eprint!("{}", responce.text().await.unwrap()); Err(BotError::NetworkError(code.to_string()))}
            // code => Err(BotError::NetworkError(code.to_string()))
        }
    }
}

#[macro_export]
macro_rules! send_text {
    ($api: expr, $chat_id: expr, $text: expr) => {
        tokio::runtime::Runtime::new().unwrap().block_on(Telebotapi::send_text($api, $chat_id, $text, None, false, false, false, false, None)).unwrap();
    };
    ($api: expr, $chat_id: expr, $text: expr, $parse: expr) => {
        tokio::runtime::Runtime::new().unwrap().block_on(Telebotapi::send_text($api, $chat_id, $text, Some($parse), false, false, false, false, None)).unwrap();
    }
}

impl PartialEq<BotUpdate> for MessageType {
    fn eq(&self, other: &BotUpdate) -> bool {
        if let Some(msg) = &other.message {
            if let Some(text) = &msg.text { 
                if let MessageType::Text(msg_text) = self {
                    // if msg_text.is_empty() { return true; }
                    return text == msg_text;
                }
            }
        }
        if let Some(_) = &other.inline_query{
            return self == &MessageType::InlineQuery;
        }
        if let Some(query) = &other.callback_query{
            if let Some(data) = &query.data{
                if let MessageType::CallBackQuery(msg) = self{
                    return msg == data;
                }
            }
        }
        false
    }
}

impl InlineQueryResult{
    pub fn article(id: u8, title: &str, responce: &str) -> Result<Self, BotError>{
        if title.is_empty() || responce.is_empty() { return Err(BotError::InternalError(String::from("Invalid inline query request"))) }
        Ok(InlineQueryResult::Article( InlineQueryResultArticle {
            query_type: String::from("article"),
            id: id,
            title: title.to_string(),
            input_message_content: InputMessageContent::Text(InputTextMessageContent{ message_text: responce.to_string(), disable_web_page_preview: false})
        }))
    }
    pub fn voice(id: u8, title: &str, caption: &str, voice_url: Option<&String>) -> Result<Self, BotError>{
        if title.is_empty() { return Err(BotError::InternalError(String::from("Invalid inline query request"))) }
        if let Some(voice_url) = voice_url{
            return Ok(InlineQueryResult::Voice( InlineQueryResultVoice {
                query_type: String::from("voice"),
                id: id,
                title: title.to_string(),
                voice_url: voice_url.to_string(),
                caption: Some(caption.to_string()),
                // parse_mode: Some(parse_mode.to_string())
            }))
        }else{
            Self::article(id, title, caption)
        }
    }
}


#[macro_export]
macro_rules! inline_query {
    ( $( $q_type:ident => ( $($attr: expr),+ ) ),* ) => {
        {
            let mut res = Vec::new();
            let mut i = 1;
            $(
                match InlineQueryResult::$q_type(i, $( $attr ),+ ){
                    Ok(val) => res.push(val),
                    Err(msg) => eprintln!("{:?}", msg)
                };
                i += 1;
            )*
            res
        }
    };
    ( $( *q_type:ident => $query_list:tt ),* ) => {
        let res = Vec::new();
        let mut i = 1;
        $(
            match InlineQueryResult::$q_type(i, query_list[0], query_list[1]){
                Ok(val) => res.push(val),
                Err(msg) => eprintln!("{:?}", msg)
            };
            i += 1;
        )*
        res
    }
}

impl ReplyMarkup{
    pub fn inline_keyboard_markup(keys: Vec<Vec<(String, String)>>) -> ReplyMarkup{
        ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup{
            inline_keyboard: keys.into_iter().map( |arr| arr.into_iter().map( |(text, callback_data)| InlineKeyboardButton{ text: text, callback_data: Some(callback_data), url: None} ).collect() ).collect()
        })
    }
}