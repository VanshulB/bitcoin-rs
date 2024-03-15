use anyhow::{Error, Result};
use bdk::{
    keys::{
        bip39::{Language, Mnemonic, WordCount},
        GeneratableKey, GeneratedKey,
    },
    miniscript::Tap,
};

fn main() -> Result<()> {
    let mnemonic: GeneratedKey<_, Tap> =
        Mnemonic::generate((WordCount::Words12, Language::English))
            .map_err(|_| Error::msg("mnemonic generation failed"))?;
    println!("Generated Phrase: \n{}", *mnemonic);
    Ok(())
}
