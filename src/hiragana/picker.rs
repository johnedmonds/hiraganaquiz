use rand::Rng;

use super::Hiragana;

pub fn pick_hiragana<R: Rng>(
    hiragana: &[&'static Hiragana<'static>],
    rng: &mut R,
) -> &'static Hiragana<'static> {
    hiragana[rng.gen_range(0..hiragana.len())]
}

pub fn pick_hiragana_exclude_existing<R: Rng>(
    hiragana: &[&'static Hiragana<'static>],
    rng: &mut R,
    existing: &'static Hiragana<'static>,
) -> &'static Hiragana<'static> {
    if let Some(position) = hiragana.iter().position(|h| existing == *h) {
        let next = rng.gen_range(0..hiragana.len() - 1);
        let next = if next < position { next } else { next + 1 };
        hiragana[next]
    } else {
        pick_hiragana(hiragana, rng)
    }
}
#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;

    use crate::hiragana::Hiragana;

    use super::pick_hiragana_exclude_existing;
    const TEST_HIRAGANA: &'static [&'static Hiragana<'static>] = &[
        &Hiragana { jpn: "a", eng: "b" },
        &Hiragana { jpn: "c", eng: "d" },
        &Hiragana { jpn: "e", eng: "f" },
    ];

    #[test]
    fn existing_before() {
        assert_eq!(
            TEST_HIRAGANA[0],
            pick_hiragana_exclude_existing(
                TEST_HIRAGANA,
                &mut StepRng::new(0, 0),
                &TEST_HIRAGANA[1]
            )
        );
    }
}
