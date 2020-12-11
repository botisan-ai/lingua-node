import { LanguageDetector as NativeLanguageDetector } from '../native';

export class LanguageDetector {
  private detector: NativeLanguageDetector;

  constructor() {
    console.time('language detector init');
    this.detector = new NativeLanguageDetector();
    console.timeEnd('language detector init');
  }

  public detectLanguage(text: string): string | undefined {
    return this.detector.detectLanguage(text);
  }
}
