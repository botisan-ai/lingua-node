use neon::prelude::*;
use std::str::FromStr;
use lingua::{LanguageDetectorBuilder, Language, LanguageDetector, IsoCode639_1};

declare_types! {
  pub class JsLanguageDetector for LanguageDetector {
    init(mut cx) {
      let language_codes_array_handle: Handle<JsArray> = cx.argument::<JsArray>(0)?;

      let language_codes_jsvalue_vec: Vec<Handle<JsValue>> =
        language_codes_array_handle.to_vec(&mut cx)?;

      let languages: Vec<Language> = language_codes_jsvalue_vec.iter()
        .filter(|&value| value.is_a::<JsString>())
        .map(|&value| value.downcast::<JsString>())
        .map(|value| value.or_throw(&mut cx).unwrap().value())
        .map(|code| IsoCode639_1::from_str(&code).unwrap())
        .map(|iso_code| Language::from_iso_code_639_1(&iso_code))
        .collect::<Vec<_>>();

      if languages.is_empty() {
        let detector = LanguageDetectorBuilder::from_all_languages()
          .with_minimum_relative_distance(0.1)
          .build();

        Ok(detector)
      } else {
        let detector = LanguageDetectorBuilder::from_languages(&languages)
          .with_minimum_relative_distance(0.1)
          .build();

        Ok(detector)
      }
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
