use lingua::{LanguageDetectorBuilder, Language, LanguageDetector};
use lingua::Language::{Chinese, English, Turkish, Russian};

fn main() {
  let text = "davay";
  println!("detecting language for text {:?}", text);
  let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(
    &vec![Chinese, English, Turkish, Russian]
  ).build();
  println!("Detector initialized");
  let language: Option<Language> = detector.detect_language_of(text);
  match language {
    None => {
      println!("cannot detect");
    },
    Some(l) => {
      println!("{:}", l.iso_code_639_1().to_string());
    },
  };
}
