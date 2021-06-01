declare namespace Lingua {
  export class LanguageDetector {
    constructor(languages: string[]);
    detectLanguage(text: string): string;
  }
}

export = Lingua;
