mod botlib;
use crate::botlib::*;
use explainer::*;
use std::env::args;

const API_KEY: &str =  env!("DICTIONARY_API" );
const BOT_API: &str = env!("BOT_API" );

#[tokio::main]
async fn main(){

    // let word = WordFinder::new(std::env::args().collect::<Vec<String>>().iter().nth(1).unwrap_or(&String::new())).await;
    // println!("{}", word);

    let mut bot = Telebotapi::new(&BOT_API).await.unwrap();

    bot.add_handler(MessageType::Text(String::new()), |api, responce| {
        tokio::runtime::Runtime::new().unwrap().block_on( async {
            // send_text!(api, 1895656874, responce.message.as_ref().unwrap().text.as_ref().unwrap());
            
            let word: Option<(String, Vec<u8>)> = match WordFinder::new( &API_KEY, responce.message.as_ref().unwrap().text.as_ref().unwrap() ).await{
                Ok(responce) => Some((format!("{}", responce), responce.get_audio().await.unwrap_or(vec![]))),
                Err(responce_vec) => { 
                    if responce_vec.len() > 0 { 
                        // let w = WordFinder::new( &responce_vec[0] ).await.unwrap(); 
                        // ( format!("{}", w), w.get_audio().await.unwrap_or(vec![]));
                        // responce_vec.iter().map( |x| (x, String::from("something")) ).collect();
                        Telebotapi::send_text(api.to_string(), responce.message.as_ref().unwrap().chat.id, "Word not found. Here are similar words", None, true, false, false, true, 
                                                    Some(
                                                        ReplyMarkup::InlineKeyboardMarkup(
                                                            // responce_vec.into_iter().map( |word| vec![ (word.to_string(), format!("search_for_word {}", word) ) ] ).collect()
                                                            responce_vec.into_iter().map( |word| vec![ (word.to_string(), format!("{}", word) ) ] ).collect()
                                                        )
                                                    )
                                                ).await.unwrap();
                        None
                    }
                    else { Some((format!("Word: {} not found in english dictionary", responce.message.as_ref().unwrap().text.as_ref().unwrap()), vec![])) } 
                }
            };

            if let Some(word) = word{
                Telebotapi::send_voice(api, responce.message.as_ref().unwrap().chat.id, word.1, &word.0, None, None).await.unwrap();
            }
        } );
    });

    bot.add_handler(MessageType::CallBackQuery(String::new()), |api, responce| {
        tokio::runtime::Runtime::new().unwrap().block_on( async {
            let word = WordFinder::new( &API_KEY, responce.callback_query.as_ref().unwrap().data.as_ref().unwrap() ).await.unwrap();
            Telebotapi::send_voice(api, responce.callback_query.as_ref().unwrap().message.as_ref().unwrap().chat.id, word.get_audio().await.unwrap_or(vec![]), &format!("{}", word), None, None).await.unwrap();
        } );
    });

    bot.add_handler(MessageType::Text(String::from("/start")), |api, responce| {
        send_text!(api, responce.message.as_ref().unwrap().chat.id, "superText");
    });

    bot.add_handler(MessageType::Text(String::from("/help")), |api, responce| {
        send_text!(api, responce.message.as_ref().unwrap().chat.id, "*Help* page\nas you can see it's _in the development_", "markdownv2");
    });

    bot.add_handler(MessageType::InlineQuery, |api, responce| {
        tokio::runtime::Runtime::new().unwrap().block_on( async {
            let word: (String, String, Option<String> ) = match WordFinder::new( &API_KEY, &responce.inline_query.as_ref().unwrap().query ).await{
                Ok(responce) => ( format!("Definition of word {}", responce.get_word()), format!("{}", responce), responce.get_audio_link() ),
                Err(responce_vec) => { if responce_vec.len() > 0 { ( format!("Definition for word {}", responce_vec[0]), format!("{}", WordFinder::new( &API_KEY, &responce_vec[0] ).await.unwrap()), None )}
                                        else { ( String::from("No word found") , String::from("No word found"), None ) } }
            };

            Telebotapi::answer_query(
                        api, 
                        responce.inline_query.as_ref().unwrap().id.to_string(), 
                        inline_query![
                            voice   => ( &word.0, &word.1, word.2.as_ref() ),
                            voice   => ( "It's pronounciation", "", word.2.as_ref() )
                        ]
                ).await
            } ).unwrap();
    });

    bot.listen().await.unwrap();

}