use neon::prelude::*;

mod language_detector;

use language_detector::JsLanguageDetector;

register_module!(mut cx, {
    cx.export_class::<JsLanguageDetector>("LanguageDetector")
});
