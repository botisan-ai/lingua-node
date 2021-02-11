use neon::prelude::*;
use lingua::{LanguageDetectorBuilder, Language, LanguageDetector};
use lingua::Language::{English, German, French, Portuguese, Dutch, Italian, Polish, Russian, Japanese, Chinese};

declare_types! {
  pub class JsLanguageDetector for LanguageDetector {
    init(_) {
      // let language_codes_array_handle: Handle<JsArray> = cx.argument::<JsArray>(0)?;

      // let language_codes_jsvalue_vec: Vec<Handle<JsValue>> =
      //   language_codes_array_handle.to_vec(&mut cx)?;

      // let languages: Vec<Language> = language_codes_jsvalue_vec.iter()
      //   .filter(|&value| value.is_a::<JsString>())
      //   .map(|&value| value.downcast::<JsString>())
      //   .map(|value| value.or_throw(&mut cx).unwrap().value())
      //   .map(|code| Language::from_iso_code_639_1())
      //   .collect::<Vec<_>>();

      let detector = LanguageDetectorBuilder::from_languages(
        &vec![English, German, French, Portuguese, Dutch, Italian, Polish, Russian, Japanese, Chinese]
      )
      .with_minimum_relative_distance(0.1)
      .build();

      Ok(detector)
    }

    method detectLanguage(mut cx) {
      let text = cx.argument::<JsString>(0)?.value();
      let this = cx.this();
      let guard = cx.lock();

      let language: Option<Language> = this.borrow(&guard).detect_language_of(text);
      match language {
        None => Ok(cx.undefined().upcast()),
        Some(l) => Ok(cx.string(l.iso_code_639_1().to_string()).upcast()),
      }
    }
  }
}
