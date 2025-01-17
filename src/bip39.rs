use bip39::{Mnemonic, MnemonicType, Language, Seed};

pub struct MnemonicPhrase<'a> {
    words: u8,
    list_of_words: [&'a str;24u8],
}

impl MnemonicPhrase {
    pub fn new(word_count: MnemonicType, language: Language) {
        let mnemonic = Mnemonic::new(word_count, language);
    }
}