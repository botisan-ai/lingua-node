import { LanguageDetector as NativeLanguageDetector } from '../native';

export class LanguageDetector {
  private detector: NativeLanguageDetector;

  constructor() {
    console.log('initializing language detector');
    console.time('language detector init');
    this.detector = new NativeLanguageDetector();
    console.timeEnd('language detector init');
  }

  public detectLanguage(text: string): string | undefined {
    return this.detector.detectLanguage(text);
  }
}

const detector = new LanguageDetector();
setInterval(() => console.log(detector.detectLanguage('hello world')), 10000);
