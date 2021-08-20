use std::fmt::{ Formatter, Display };
use serde::{Deserialize, Serialize};
use bytes::Bytes;
use std::io::{Error, Cursor, ErrorKind, Read};

#[derive(Serialize, Deserialize, Debug)]
struct WordMeta {id: String, offensive:bool}
#[derive(Serialize, Deserialize, Debug)]
struct WordAudio { hw: String, prs: Option<Vec<WordAudioExample>> }
#[derive(Serialize, Deserialize, Debug)]
struct WordAudioExample { mw: Option<String>, sound: Option<WordAudioMeta> }
#[derive(Serialize, Deserialize, Debug)]
struct WordAudioMeta { audio: String }

#[derive(Serialize, Deserialize, Debug)]
struct WordResponce{
    meta: WordMeta,
    hwi: WordAudio,
    shortdef: Vec<String>
}

pub struct WordFinder{
    word: String,
    offensive: bool,
    headword: String, // pronunciation
    pronunciation: Option<String>,
    audio_link: Option<String>,
    definition: Vec<String> // list of short definitions
}

impl WordFinder {
    pub async fn new(api: &str, word: &str) -> Result<WordFinder, Vec<String>> {
        if word.is_empty() { 
            /*return Err(String::from("no word no definition"))*/
            return Err( vec![] )
        }
        // let resp: Vec::<WordResponce> = serde_json::from_str(
        //     &reqwest::get(&format!("https://dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}", word, API_KEY))
        //         .await.unwrap().text().await.unwrap()
        //     ).unwrap();
        
        let word_request = reqwest::get(&format!("https://dictionaryapi.com/api/v3/references/collegiate/json/{}?key={}", word, api)).await.unwrap();
        let request_responce = word_request.text().await.unwrap();

        match serde_json::from_str::<Vec<WordResponce>>(
            &request_responce
        ) {
            Ok(val) if val.len() > 0 => Ok( WordFinder::from_wordresponce( val.into_iter().nth(0).unwrap() )),
            Ok(val) => { println!("{:?}", val); Err( vec![] ) }
            Err(_) => Err(serde_json::from_str::<Vec<String>>( &request_responce ).expect("Error during request for query word correction"))
        }
    }

    fn from_wordresponce(wordresponce: WordResponce) -> WordFinder{
        WordFinder{
            word: wordresponce.meta.id.to_string(),
            offensive: wordresponce.meta.offensive,
            headword: wordresponce.hwi.hw.to_string(),
            pronunciation: match wordresponce.hwi.prs.as_ref(){
                Some(val) => Some(val[0].mw.as_ref().unwrap_or(&String::new()).to_string()),
                None => None
            },
            audio_link: match wordresponce.hwi.prs.as_ref() {
                Some(val) => match &val.iter().filter( |x| !x.sound.is_none() ).next().unwrap().sound{
                    Some(val) => match val.audio.to_string() {
                        val if val.len() > 3 && val.get(..3).unwrap() == "bix" => Some( format!("https://media.merriam-webster.com/audio/prons/en/us/ogg/bix/{}.ogg", val) ),
                        val if val.len() > 2 && val.get(..2).unwrap() == "gg" => Some( format!("https://media.merriam-webster.com/audio/prons/en/us/ogg/gg/{}.ogg", val) ),
                        val if val.chars().nth(0).unwrap().is_alphabetic() => Some( format!("https://media.merriam-webster.com/audio/prons/en/us/ogg/{}/{}.ogg", val.get(..1).unwrap(), val) ),
                        val if val.is_empty() => None,
                        val => Some( format!("https://media.merriam-webster.com/audio/prons/en/us/ogg/number/{}.ogg", val) )
                    },
                    None => None
                },
                None => None
            },
            definition: wordresponce.shortdef
        }
    }
    pub async fn get_audio(&self) -> Result<Vec<u8>, Error> {
        match &self.audio_link{
            Some(audio) => {
                let mut res = Cursor::new(reqwest::get(audio).await.unwrap().bytes().await.unwrap());
                let mut vec_res = Vec::new();
                res.read_to_end(&mut vec_res).unwrap();

                Ok(vec_res)
            },
            None =>    Err(Error::from(ErrorKind::NotFound))
        }
    }
    pub fn get_audio_link(&self) -> Option<String> {
        match self.audio_link.as_ref() { Some(val) => Some(val.to_string()), None => None }
    }
    pub fn get_word(&self) -> String{
        self.word.split(":").next().unwrap().to_string()
    }
}

impl Display for WordFinder{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "word: {}", self.get_word());
        if let Some(pronoun) = &self.pronunciation {
            writeln!(f, "pronunciation: {}", pronoun);
        }
        writeln!(f, "definitions:\n\t- {}", self.definition.join("\n\t- "));
        // if let Some(audio) = &self.audio_link{
        //     return write!(f, "audio: {}", audio);
        // }
        Ok(())
    }
}