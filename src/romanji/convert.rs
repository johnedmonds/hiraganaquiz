use crate::hiragana::{data::SYMBOLS, Hiragana};
use lazy_static::lazy_static;
use nom::{IResult, branch::{alt, Alt}, multi::many0};
use patricia_tree::PatriciaMap;

struct HiraganaAlt<'a> {
    lookup: &'a PatriciaMap<&'static Hiragana<'static>>,
}

impl<'a, 'b> Alt<&'a str, &'static Hiragana<'static>, ()> for HiraganaAlt<'b> {
    fn choice(&mut self, input: &'a str) -> nom::IResult<&'a str, &'static Hiragana<'static>, ()> {
        if let Some((k, v)) = self.lookup.get_longest_common_prefix(input) {
            Ok((&input[k.len()..], v))
        } else {
            println!("not found {}",input);
            println!("{:?}", self.lookup.get("a"));
            Err(nom::Err::Error(()))
        }
    }
}

lazy_static! {
    static ref HIRAGANA_MAP_BY_ENG: PatriciaMap<&'static Hiragana<'static>> = {
        let mut map = PatriciaMap::new();
        for hiragana in SYMBOLS {
            map.insert(hiragana.eng, hiragana);
        }
        map
    };
}

pub fn to_hiragana(s: &str) -> IResult<&str, Vec<&'static Hiragana<'static>>, ()> {
    many0(alt::<&str, &'static Hiragana<'static>, (), HiraganaAlt>(
        HiraganaAlt {
            lookup: &*HIRAGANA_MAP_BY_ENG,
        },
    ))(s)
}

#[cfg(test)]
mod tests {
    use crate::{hiragana::data, romanji::convert::to_hiragana};

    #[test]
    fn hiragana() {
        assert_eq!(Ok(("", vec![&data::A])), to_hiragana("a"));
    }
    #[test]
    fn hiragana_multi() {
        assert_eq!(Ok(("", vec![&data::A, &data::SHI])), to_hiragana("ashi"));
    }

    #[test]
    fn hiragana_err() {
        assert_eq!(Err(nom::Err::Error(())), to_hiragana("x"));
    }
}
