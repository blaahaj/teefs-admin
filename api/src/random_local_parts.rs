use chbs::{
    config::BasicConfig, probability::Probability::Never, scheme::ToScheme, word::WordList,
};
use rand::Rng;

#[derive(serde::Serialize)]
pub(crate) struct RandomLocalParts {
    six_hex: String,
    four_dotted_words: String,
}

pub(crate) fn generate() -> RandomLocalParts {
    RandomLocalParts {
        six_hex: six_hex(),
        four_dotted_words: four_dotted_words(),
    }
}

fn six_hex() -> String {
    let mut bytes = [0u8; 3];
    rand::thread_rng().fill(&mut bytes);
    format!("{:02x}{:02x}{:02x}", bytes[0], bytes[1], bytes[2])
}

fn four_dotted_words() -> String {
    let mut config = BasicConfig::default();
    config.word_provider = WordList::builtin_eff_short().sampler();
    config.words = 4;
    config.separator = ".".into();
    config.capitalize_first = Never;
    config.capitalize_words = Never;
    config.to_scheme().generate()
}
