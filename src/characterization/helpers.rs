// Table tests of the pure helpers, with explicit expected values.
//
// The expectations were derived by reading the code and confirmed by running
// it: they are what the helpers do today, not what French grammar says. The
// oddities are marked QUIRK.

use std::collections::HashSet;
use std::iter::FromIterator;

use crate::adj::Adj;
use crate::adj_enums::{AdjCatId, AdjId};
use crate::common_enums::{
    adjust_blacklist, merge_blacklists, Article, BlackLists, Gender, Number,
};
use crate::haiku::check_haiku_form;
use crate::hashsets::merge_hashsets;
use crate::noun::{get_apposition, Noun};
use crate::noun_enums::{NounCatId, NounId};
use crate::string::{
    drop_last_graphemes, take_last_grapheme, take_last_graphemes, uppercase_first_letter,
};
use crate::verb::{as_pronominal, get_verb, get_verb_cat, Verb};
use crate::verb_enums::{VerbCatId, VerbGroup, VerbId, VerbKind};
use crate::wordgroup::{add_words, check_ellision, fold_wordgroups, get_plural, WordGroup};

use super::names::article_name;
use super::seam;

fn wg(text: &str, feet: u8) -> WordGroup {
    WordGroup::new(text, feet)
}

fn range(text: &str, min: u8, max: u8) -> WordGroup {
    WordGroup {
        text: String::from(text),
        foots: (min, max),
    }
}

// ------------------------------------------------------------------ wordgroup

#[test]
fn wordgroup_constructors() {
    assert_eq!(wg("lune", 1), range("lune", 1, 1));
    assert_eq!(WordGroup::new_empty(), range("", 0, 0));
}

#[test]
fn check_ellision_table() {
    for letter in ['a', 'e', 'i', 'o', 'u', 'é', 'h', 'â'].iter() {
        assert!(check_ellision(letter), "{}", letter);
    }
    // QUIRK: the other accented vowels, y, œ and uppercase letters do not
    // elide; h always does (no aspirated h).
    for letter in [
        'b', 'c', 'l', 's', 'y', 'è', 'ê', 'à', 'î', 'ô', 'û', 'œ', 'A', 'É', '\'', ' ',
    ]
    .iter()
    {
        assert!(!check_ellision(letter), "{}", letter);
    }
}

#[test]
fn add_words_table() {
    // (a, b, expected). Feet are a + b, plus 1 when a mute "e" is sounded.
    let cases = [
        // --- joining
        // empty left side: no space
        (wg("", 0), wg("lune", 1), wg("lune", 1)),
        (wg("", 0), wg("", 0), wg("", 0)),
        // apostrophe: no space
        (wg("l'", 0), wg("étoile", 2), wg("l'étoile", 2)),
        (wg("d'", 0), wg("une lune", 3), wg("d'une lune", 3)),
        (wg("s'", 0), wg("éveille", 2), wg("s'éveille", 2)),
        // otherwise one space
        (wg("la", 1), wg("lune", 1), wg("la lune", 2)),
        // QUIRK: empty right side still gets the space
        (wg("lune", 1), wg("", 0), wg("lune ", 1)),
        // --- mute e: "<consonant>e" followed by a consonant counts one more
        (wg("la lune", 2), wg("calme", 1), wg("la lune calme", 4)),
        (wg("une", 1), wg("lune", 1), wg("une lune", 3)),
        (wg("arôme", 2), wg("sucré", 2), wg("arôme sucré", 5)),
        // ... not before a vowel
        (wg("la lune", 2), wg("immense", 2), wg("la lune immense", 4)),
        (wg("une", 1), wg("étoile", 2), wg("une étoile", 3)),
        (wg("lune", 1), wg("œil", 1), wg("lune œil", 2)),
        // QUIRK: h is in the consonant list here, although it elides elsewhere
        (wg("une", 1), wg("hermine", 2), wg("une hermine", 4)),
        // ... not when the letter before the e is a vowel
        (wg("la pie", 2), wg("noire", 1), wg("la pie noire", 3)),
        (wg("la rosée", 3), wg("glacée", 2), wg("la rosée glacée", 5)),
        // QUIRK: ç is not in the consonant list
        (wg("garçe", 1), wg("noire", 1), wg("garçe noire", 2)),
        // ... not when the left side ends with another letter
        (wg("le soleil", 3), wg("brun", 1), wg("le soleil brun", 4)),
        (wg("été", 2), wg("brûlant", 2), wg("été brûlant", 4)),
        // too short to have a letter before the e
        (wg("e", 1), wg("lune", 1), wg("e lune", 2)),
        // leading blanks of the right side are skipped when looking at its first letter
        (wg("lune", 1), wg(" calme", 1), wg("lune  calme", 3)),
        // --- mute e, plural: "<consonant>es" counts one more whatever follows
        (wg("les lunes", 2), wg("calmes", 1), wg("les lunes calmes", 4)),
        (wg("les lunes", 2), wg("immenses", 2), wg("les lunes immenses", 5)),
        // ... not when the letter before "es" is a vowel
        (wg("les pies", 2), wg("noires", 1), wg("les pies noires", 3)),
        (wg("es", 1), wg("lune", 1), wg("es lune", 2)),
        // QUIRK: nothing follows, so nothing is added, even for "es"
        (wg("lunes", 1), wg("", 0), wg("lunes ", 1)),
        // --- short-circuits: le, de, se, les, des never count one more ...
        (wg("le", 1), wg("bruissement", 3), wg("le bruissement", 4)),
        (wg("de", 1), wg("lune", 1), wg("de lune", 2)),
        (wg("se", 1), wg("couche", 1), wg("se couche", 2)),
        (wg("les", 1), wg("lunes", 1), wg("les lunes", 2)),
        (wg("des", 1), wg("lunes", 1), wg("des lunes", 2)),
        // ... nor a left side ending with " le", " de", " se"
        (wg("le bruissement de", 5), wg("feuille", 1), wg("le bruissement de feuille", 6)),
        (wg("la lune se", 3), wg("couche", 1), wg("la lune se couche", 4)),
        (wg("voit le", 2), wg("cerf", 1), wg("voit le cerf", 3)),
        // ... but " les" / " des" inside a group, or other short words, do
        (wg("voit les", 2), wg("cerfs", 1), wg("voit les cerfs", 4)),
        (wg("elle", 1), wg("dort", 1), wg("elle dort", 3)),
        (wg("que", 1), wg("lune", 1), wg("que lune", 2)),
        (wg("une", 1), wg("pie", 1), wg("une pie", 3)),
    ];
    for (a, b, expected) in cases.iter() {
        assert_eq!(&add_words(a, b), expected, "add_words({:?}, {:?})", a, b);
    }
}

#[test]
fn add_words_adds_both_bounds_of_the_feet_range() {
    assert_eq!(
        add_words(&range("la lumière", 3, 4), &range("violette", 2, 3)),
        range("la lumière violette", 6, 8)
    );
    assert_eq!(
        add_words(&range("un prunier", 3, 4), &range("violet", 2, 3)),
        range("un prunier violet", 5, 7)
    );
}

#[test]
fn fold_wordgroups_table() {
    assert_eq!(fold_wordgroups(vec![]), wg("", 0));
    assert_eq!(fold_wordgroups(vec![&wg("lune", 1)]), wg("lune", 1));
    assert_eq!(
        fold_wordgroups(vec![&wg("", 0), &wg("lune", 1), &wg("de", 1), &wg("printemps", 2)]),
        wg("lune de printemps", 5)
    );
    assert_eq!(
        fold_wordgroups(vec![&wg("la lune", 2), &wg("reste", 1), &wg("calme", 1)]),
        wg("la lune reste calme", 6)
    );
}

#[test]
fn get_plural_table() {
    let cases = [
        // ends with s or x: unchanged
        ("printemps", "printemps"),
        ("paix", "paix"),
        ("roux", "roux"),
        ("s", "s"),
        // -al -> -aux
        ("automnal", "automnaux"),
        ("glacial", "glaciaux"),
        ("al", "aux"),
        // otherwise + s
        ("lune", "lunes"),
        ("été", "étés"),
        ("cœur", "cœurs"),
        ("en fleur", "en fleurs"),
        // QUIRK: no -eau / -eu / -ou rule, and an empty word becomes "s"
        ("oiseau", "oiseaus"),
        ("bleu", "bleus"),
        ("", "s"),
    ];
    for (singular, plural) in cases.iter() {
        assert_eq!(get_plural(&wg(singular, 2)), wg(plural, 2), "{}", singular);
    }
    // the feet range is kept as is
    assert_eq!(get_plural(&range("cerisier", 3, 4)), range("cerisiers", 3, 4));
    assert_eq!(get_plural(&range("automnal", 3, 4)), range("automnaux", 3, 4));
}

// ----------------------------------------------------------------- adjectives

// `get_feminine` is private to adj.rs: it is reached through `Adj::agreed`
// on a regular adjective (no special form).
#[test]
fn feminine_and_plural_rules_table() {
    // (masculine, feminine, masculine plural, feminine plural)
    let cases = [
        // ends with e: unchanged
        ("libre", "libre", "libres", "libres"),
        // -eux / -eur -> -euse
        ("heureux", "heureuse", "heureux", "heureuses"),
        ("moqueur", "moqueuse", "moqueurs", "moqueuses"),
        // -er -> -ère
        ("léger", "légère", "légers", "légères"),
        ("printanier", "printanière", "printaniers", "printanières"),
        // -et -> -ette
        ("violet", "violette", "violets", "violettes"),
        // otherwise + e
        ("brun", "brune", "bruns", "brunes"),
        ("doré", "dorée", "dorés", "dorées"),
        ("bleu", "bleue", "bleus", "bleues"),
        ("gris", "grise", "gris", "grises"),
        ("automnal", "automnale", "automnaux", "automnales"),
        // QUIRK: what the rules give for irregular adjectives when no special
        // form is declared
        ("roux", "rouxe", "roux", "rouxes"),
        ("inquiet", "inquiette", "inquiets", "inquiettes"),
        ("doux", "douxe", "doux", "douxes"),
        ("blanc", "blance", "blancs", "blances"),
        ("meilleur", "meilleuse", "meilleurs", "meilleuses"),
    ];
    for (masc, fem, masc_plur, fem_plur) in cases.iter() {
        let adj = Adj::new(AdjId::Libre, masc, (2, 3));
        let agreed = |gender, number| adj.agreed(gender, number);
        assert_eq!(agreed(Gender::Male, Number::Singular), range(masc, 2, 3));
        assert_eq!(agreed(Gender::Female, Number::Singular), range(fem, 2, 3));
        assert_eq!(agreed(Gender::Male, Number::Plural), range(masc_plur, 2, 3));
        assert_eq!(agreed(Gender::Female, Number::Plural), range(fem_plur, 2, 3));
    }
}

#[test]
fn special_adjective_forms_win_over_the_rules() {
    let some = |text: &str| Some(String::from(text));
    let adj = Adj::new_special(
        AdjId::Roux,
        "roux",
        some("rousse"),
        some("ROUX"),
        some("rousses"),
        false,
        (1, 2),
    );
    assert_eq!(adj.agreed(Gender::Male, Number::Singular), range("roux", 1, 2));
    assert_eq!(adj.agreed(Gender::Female, Number::Singular), range("rousse", 1, 2));
    assert_eq!(adj.agreed(Gender::Male, Number::Plural), range("ROUX", 1, 2));
    assert_eq!(adj.agreed(Gender::Female, Number::Plural), range("rousses", 1, 2));

    // a missing special form falls back to the rule, each form on its own:
    // the feminine plural is NOT derived from the special feminine
    let adj = Adj::new_special(AdjId::Inquiet, "inquiet", some("inquiète"), None, None, false, (2, 3));
    assert_eq!(adj.agreed(Gender::Female, Number::Singular), range("inquiète", 2, 3));
    assert_eq!(adj.agreed(Gender::Male, Number::Plural), range("inquiets", 2, 3));
    assert_eq!(adj.agreed(Gender::Female, Number::Plural), range("inquiettes", 2, 3));

    // invariable wins over everything, special forms included
    let adj = Adj::new_special(AdjId::Pastel, "pastel", some("x"), some("y"), some("z"), true, (2, 2));
    for gender in [Gender::Male, Gender::Female].iter() {
        for number in [Number::Singular, Number::Plural].iter() {
            assert_eq!(adj.agreed(*gender, *number), wg("pastel", 2));
        }
    }
}

// ---------------------------------------------------------------------- nouns

#[test]
fn noun_articles_table() {
    let lune = Noun::new(NounId::Lune, "lune", Gender::Female, (1, 1));
    let cerf = Noun::new(NounId::Cerf, "cerf", Gender::Male, (1, 1));
    let etoile = Noun::new(NounId::Etoile, "étoile", Gender::Female, (2, 2));
    let hiver = Noun::new(NounId::Hiver, "hiver", Gender::Male, (2, 2));
    let empty = Noun::new(NounId::Lune, "", Gender::Female, (0, 0));
    let empty_male = Noun::new(NounId::Lune, "", Gender::Male, (0, 0));
    let s = Number::Singular;
    let p = Number::Plural;
    let cases = [
        (&lune, s, Article::None, wg("", 0)),
        (&lune, s, Article::Definite, wg("la", 1)),
        (&lune, s, Article::Indefinite, wg("une", 1)),
        (&cerf, s, Article::None, wg("", 0)),
        (&cerf, s, Article::Definite, wg("le", 1)),
        (&cerf, s, Article::Indefinite, wg("un", 1)),
        // elision: only the singular definite article, and it has no foot
        (&etoile, s, Article::Definite, wg("l'", 0)),
        (&etoile, s, Article::Indefinite, wg("une", 1)),
        (&hiver, s, Article::Definite, wg("l'", 0)),
        (&hiver, s, Article::Indefinite, wg("un", 1)),
        // plural: gender and first letter do not matter
        (&lune, p, Article::None, wg("", 0)),
        (&lune, p, Article::Definite, wg("les", 1)),
        (&etoile, p, Article::Definite, wg("les", 1)),
        (&cerf, p, Article::Indefinite, wg("des", 1)),
        // empty lemma: an error marker as text, for the singular definite only
        (&empty, s, Article::Definite, wg("#error#get_with_some_article#No first letter for ellision#", 0)),
        (&empty_male, s, Article::Definite, wg("#error#get_with_some_article#No first letter for ellision#", 0)),
        (&empty, s, Article::Indefinite, wg("une", 1)),
        (&empty, p, Article::Definite, wg("les", 1)),
    ];
    for (noun, number, article, expected) in cases.iter() {
        assert_eq!(
            &noun.get_article(*number, *article),
            expected,
            "{} {}",
            noun.word.text,
            article_name(*article)
        );
    }
}

#[test]
fn noun_agreed_and_with_article() {
    let cerisier = Noun::new(NounId::Cerisier, "cerisier", Gender::Male, (3, 4));
    assert_eq!(cerisier.agreed(Number::Singular), range("cerisier", 3, 4));
    assert_eq!(cerisier.agreed(Number::Plural), range("cerisiers", 3, 4));
    assert_eq!(
        cerisier.get_with_article(Article::Definite, Number::Plural),
        range("les cerisiers", 4, 5)
    );
    assert_eq!(
        cerisier.get_with_article(Article::None, Number::Singular),
        range("cerisier", 3, 4)
    );
    // "une" + consonant: the mute e of the article counts
    let lune = Noun::new(NounId::Lune, "lune", Gender::Female, (1, 1));
    assert_eq!(
        lune.get_with_article(Article::Indefinite, Number::Singular),
        wg("une lune", 3)
    );
    let etoile = Noun::new(NounId::Etoile, "étoile", Gender::Female, (2, 2));
    assert_eq!(
        etoile.get_with_article(Article::Definite, Number::Singular),
        wg("l'étoile", 2)
    );
}

#[test]
fn apposition_table() {
    let lune = Noun::new(NounId::Lune, "lune", Gender::Female, (1, 1));
    let etoile = Noun::new(NounId::Etoile, "étoile", Gender::Female, (2, 2));
    let cerf = Noun::new(NounId::Cerf, "cerf", Gender::Male, (1, 1));
    let hiver = Noun::new(NounId::Hiver, "hiver", Gender::Male, (2, 2));
    let empty = Noun::new(NounId::Lune, "", Gender::Male, (0, 0));
    let cases = [
        (&lune, Article::None, range("de lune", 2, 2)),
        (&etoile, Article::None, range("d'étoile", 2, 2)),
        (&cerf, Article::None, range("de cerf", 2, 2)),
        (&hiver, Article::None, range("d'hiver", 2, 2)),
        (&lune, Article::Definite, range("de la lune", 3, 3)),
        // QUIRK: "de l'" counts 2 feet for a feminine noun, 1 for a masculine
        (&etoile, Article::Definite, range("de l'étoile", 4, 4)),
        (&hiver, Article::Definite, range("de l'hiver", 3, 3)),
        (&cerf, Article::Definite, range("du cerf", 2, 2)),
        // QUIRK: "d'une" is 1 to 2 feet (and the mute e rule adds one before a
        // consonant), "d'un" exactly 1
        (&lune, Article::Indefinite, range("d'une lune", 3, 4)),
        (&etoile, Article::Indefinite, range("d'une étoile", 3, 4)),
        (&cerf, Article::Indefinite, range("d'un cerf", 2, 2)),
        (&hiver, Article::Indefinite, range("d'un hiver", 3, 3)),
        // empty lemma: error markers as text
        (&empty, Article::None, range("#error#get_apposition#No first letter# ", 0, 0)),
        (&empty, Article::Definite, range("#error#get_apposition#No first letter# ", 1, 1)),
    ];
    for (noun, article, expected) in cases.iter() {
        assert_eq!(
            &get_apposition(noun, *article),
            expected,
            "{} {}",
            noun.word.text,
            article_name(*article)
        );
    }
}

// ---------------------------------------------------------------------- verbs

fn verb(infinitive: &str, feet: u8, group: VerbGroup, pronominal: bool) -> Verb {
    Verb::new(
        VerbId::Rester,
        infinitive,
        (feet, feet),
        group,
        pronominal,
        VerbKind::Intransitive,
    )
}

#[test]
fn verb_agree_table() {
    let s = Number::Singular;
    let p = Number::Plural;
    let cases = [
        // first group: -er -> -e / -ent, one foot less than the infinitive
        (verb("rester", 2, VerbGroup::First, false), s, wg("reste", 1)),
        (verb("rester", 2, VerbGroup::First, false), p, wg("restent", 1)),
        (verb("demeurer", 3, VerbGroup::First, false), p, wg("demeurent", 2)),
        // first group in -ever: è
        (verb("lever", 2, VerbGroup::First, false), s, wg("lève", 1)),
        (verb("lever", 2, VerbGroup::First, false), p, wg("lèvent", 1)),
        (verb("achever", 3, VerbGroup::First, false), s, wg("achève", 2)),
        // second group: -ir -> -it / -issent, same feet as the infinitive
        (verb("assoupir", 3, VerbGroup::Second, false), s, wg("assoupit", 3)),
        (verb("assoupir", 3, VerbGroup::Second, false), p, wg("assoupissent", 3)),
        // third group without irregular forms: an error marker as text
        (
            verb("dormir", 2, VerbGroup::Third, false),
            s,
            wg("error#Verb::agree#third group is not implemented yet", 0),
        ),
        (
            verb("dormir", 2, VerbGroup::Third, false),
            p,
            wg("error#Verb::agree#third group is not implemented yet", 0),
        ),
        // pronominal: "se" (1 foot) or "s'" (0 foot) in front
        (verb("lever", 2, VerbGroup::First, true), s, wg("se lève", 2)),
        (verb("coucher", 2, VerbGroup::First, true), p, wg("se couchent", 2)),
        (verb("éveiller", 3, VerbGroup::First, true), s, wg("s'éveille", 2)),
        (verb("assoupir", 3, VerbGroup::Second, true), p, wg("s'assoupissent", 3)),
        // QUIRK: the pronoun is put in front of the error marker too, and "e" elides
        (
            verb("dormir", 2, VerbGroup::Third, true),
            s,
            wg("s'error#Verb::agree#third group is not implemented yet", 0),
        ),
    ];
    for (verb, number, expected) in cases.iter() {
        assert_eq!(&verb.agree(number), expected, "{}", verb.word.text);
    }
}

#[test]
fn irregular_verb_forms_win_over_the_group() {
    let third = Verb::new_special(
        VerbId::SEndormir,
        "endormir",
        (3, 3),
        true,
        VerbKind::Intransitive,
        Some(wg("endors", 2)),
        Some(wg("endort", 2)),
        Some(wg("endorment", 2)),
    );
    assert_eq!(third.agree(&Number::Singular), wg("s'endort", 2));
    assert_eq!(third.agree(&Number::Plural), wg("s'endorment", 2));
    // new_special always builds a third-group verb: a missing person falls
    // into the "not implemented" marker
    let partial = Verb::new_special(
        VerbId::Etre,
        "être",
        (1, 1),
        false,
        VerbKind::Copule,
        None,
        Some(wg("est", 1)),
        None,
    );
    assert_eq!(partial.agree(&Number::Singular), wg("est", 1));
    assert_eq!(
        partial.agree(&Number::Plural),
        wg("error#Verb::agree#third group is not implemented yet", 0)
    );
}

#[test]
fn as_pronominal_table() {
    let cases = [
        (wg("couche", 1), wg("se couche", 2)),
        (wg("lève", 1), wg("se lève", 2)),
        (wg("éveille", 2), wg("s'éveille", 2)),
        (wg("endort", 2), wg("s'endort", 2)),
        (wg("habille", 2), wg("s'habille", 2)),
        // QUIRK: è, ê, y ... do not elide
        (wg("ère", 1), wg("se ère", 2)),
        // QUIRK: empty verb keeps the joining space
        (wg("", 0), wg("se ", 1)),
    ];
    for (verb, expected) in cases.iter() {
        assert_eq!(&as_pronominal(verb), expected);
    }
}

#[test]
fn verb_lookups() {
    assert_eq!(get_verb(VerbId::Etre).ok().map(|v| v.word), Some(wg("être", 1)));
    assert_eq!(get_verb(VerbId::SeLever).ok().map(|v| v.word), Some(wg("lever", 2)));
    assert_eq!(get_verb_cat(&VerbCatId::Etat).map(|c| c.len()), Ok(6));
    assert_eq!(get_verb_cat(&VerbCatId::EtatDEveil).map(|c| c.len()), Ok(7));
}

// ---------------------------------------------------------------------- haiku

#[test]
fn check_haiku_form_table() {
    let form = [5, 7, 5];
    let line = |min: u8, max: u8| range("la lune", min, max);
    let some = Some(String::from("la lune"));
    let cases = [
        // (line number, feet range, accepted)
        (0, (5, 5), true),
        (0, (4, 5), true),
        (0, (5, 6), true),
        (0, (3, 9), true),
        (0, (4, 4), false),
        (0, (6, 6), false),
        (0, (6, 7), false),
        (1, (7, 7), true),
        (1, (5, 7), true),
        (1, (7, 9), true),
        (1, (5, 5), false),
        (1, (5, 6), false),
        (1, (8, 9), false),
        (2, (5, 5), true),
        (2, (7, 7), false),
        // an inverted range accepts nothing
        (0, (6, 4), false),
    ];
    for (nb, (min, max), accepted) in cases.iter() {
        let result = check_haiku_form(form, *nb, &line(*min, *max));
        assert_eq!(
            result,
            if *accepted { some.clone() } else { None },
            "line {} feet ({},{})",
            nb,
            min,
            max
        );
    }
    // the form is a parameter
    assert_eq!(check_haiku_form([1, 2, 3], 2, &line(3, 3)), some);
    assert_eq!(check_haiku_form([1, 2, 3], 2, &line(5, 5)), None);
}

// ----------------------------------------------------------------- blacklists

fn blacklists(nouns: &[NounId], adjs: &[AdjId], verbs: &[VerbId]) -> BlackLists {
    BlackLists {
        nouns: HashSet::from_iter(nouns.iter().cloned()),
        adjs: HashSet::from_iter(adjs.iter().cloned()),
        verbs: HashSet::from_iter(verbs.iter().cloned()),
    }
}

// BlackLists has neither PartialEq nor Debug; AdjId and VerbId have no Debug.
fn same(a: &BlackLists, b: &BlackLists) -> bool {
    a.nouns == b.nouns && a.adjs == b.adjs && a.verbs == b.verbs
}

const SEASON_NOUNS: [NounId; 4] = [
    NounId::Printemps,
    NounId::Ete,
    NounId::Automne,
    NounId::Hiver,
];
const SEASON_ADJS: [AdjId; 4] = [
    AdjId::Printanier,
    AdjId::Estival,
    AdjId::Automnal,
    AdjId::Hivernal,
];

#[test]
fn merge_hashsets_is_a_union() {
    let a: HashSet<u8> = HashSet::from_iter([1, 2].iter().cloned());
    let b: HashSet<u8> = HashSet::from_iter([2, 3].iter().cloned());
    let empty: HashSet<u8> = HashSet::new();
    assert_eq!(
        merge_hashsets(&a, &b),
        HashSet::from_iter([1, 2, 3].iter().cloned())
    );
    assert_eq!(merge_hashsets(&a, &empty), a);
    assert_eq!(merge_hashsets(&empty, &empty), empty);
}

#[test]
fn blacklists_new_empty_and_merge() {
    let empty = BlackLists::new_empty();
    assert!(empty.nouns.is_empty() && empty.adjs.is_empty() && empty.verbs.is_empty());
    let a = blacklists(&[NounId::Lune], &[AdjId::Bleu], &[]);
    let b = blacklists(&[NounId::Lune, NounId::Cerf], &[], &[VerbId::Etre]);
    let merged = merge_blacklists(&a, &b);
    assert!(same(
        &merged,
        &blacklists(&[NounId::Lune, NounId::Cerf], &[AdjId::Bleu], &[VerbId::Etre])
    ));
    assert!(same(&merge_blacklists(&a, &empty), &a));
}

#[test]
fn adjust_blacklist_table() {
    let all_seasons = |nouns: &[NounId], adjs: &[AdjId], verbs: &[VerbId]| {
        let mut all = blacklists(nouns, adjs, verbs);
        all.nouns.extend(SEASON_NOUNS.iter().cloned());
        all.adjs.extend(SEASON_ADJS.iter().cloned());
        all
    };
    // (input, expected)
    let cases = vec![
        // nothing special: unchanged
        (blacklists(&[], &[], &[]), blacklists(&[], &[], &[])),
        (
            blacklists(&[NounId::Lune, NounId::Aurore], &[AdjId::Bleu], &[VerbId::Etre]),
            blacklists(&[NounId::Lune, NounId::Aurore], &[AdjId::Bleu], &[VerbId::Etre]),
        ),
        // a season noun: the four season nouns AND the four season adjectives
        (
            blacklists(&[NounId::Ete], &[], &[]),
            all_seasons(&[], &[], &[]),
        ),
        (
            blacklists(&[NounId::Hiver, NounId::Lune], &[AdjId::Bleu], &[VerbId::Etre]),
            all_seasons(&[NounId::Lune], &[AdjId::Bleu], &[VerbId::Etre]),
        ),
        // a season adjective: same thing
        (
            blacklists(&[], &[AdjId::Automnal], &[]),
            all_seasons(&[], &[], &[]),
        ),
        (
            blacklists(&[NounId::Printemps], &[AdjId::Estival], &[]),
            all_seasons(&[], &[], &[]),
        ),
        // midi or minuit: both
        (
            blacklists(&[NounId::Midi], &[], &[]),
            blacklists(&[NounId::Midi, NounId::Minuit], &[], &[]),
        ),
        (
            blacklists(&[NounId::Minuit, NounId::Cerf], &[AdjId::Bleu], &[]),
            blacklists(&[NounId::Midi, NounId::Minuit, NounId::Cerf], &[AdjId::Bleu], &[]),
        ),
        // both rules at once
        (
            blacklists(&[NounId::Midi], &[AdjId::Hivernal], &[VerbId::SeLever]),
            all_seasons(&[NounId::Midi, NounId::Minuit], &[], &[VerbId::SeLever]),
        ),
        // already complete: unchanged
        (
            all_seasons(&[NounId::Midi, NounId::Minuit], &[], &[]),
            all_seasons(&[NounId::Midi, NounId::Minuit], &[], &[]),
        ),
        // the moments of the day are not concerned
        (
            blacklists(&[NounId::Aurore, NounId::Crepuscule], &[], &[]),
            blacklists(&[NounId::Aurore, NounId::Crepuscule], &[], &[]),
        ),
    ];
    for (nb, (input, expected)) in cases.into_iter().enumerate() {
        let adjusted = adjust_blacklist(input);
        assert!(
            same(&adjusted, &expected),
            "case {}: nouns {:?}, {} adjectives, {} verbs",
            nb,
            adjusted.nouns,
            adjusted.adjs.len(),
            adjusted.verbs.len()
        );
    }
}

// -------------------------------------------------------------------- strings

#[test]
fn take_last_grapheme_table() {
    let cases = [
        ("", ""),
        ("a", "a"),
        ("lune", "e"),
        ("été", "é"),
        ("cœur", "r"),
        ("l'", "'"),
        ("lune ", " "),
        // a decomposed accent stays with its letter
        ("ete\u{301}", "e\u{301}"),
    ];
    for (word, expected) in cases.iter() {
        assert_eq!(&take_last_grapheme(word), expected, "{:?}", word);
    }
}

#[test]
fn take_last_graphemes_table() {
    let cases = [
        ("lune", 0, ""),
        ("lune", 1, "e"),
        ("lune", 2, "ne"),
        ("lune", 4, "lune"),
        ("lune", 9, "lune"),
        ("", 3, ""),
        ("été", 2, "té"),
        ("la lune se", 3, " se"),
        ("paraissent", 3, "ent"),
        ("ete\u{301}", 2, "te\u{301}"),
    ];
    for (word, n, expected) in cases.iter() {
        assert_eq!(&take_last_graphemes(word, *n), expected, "{:?} {}", word, n);
    }
}

#[test]
fn drop_last_graphemes_table() {
    let cases = [
        ("lune", 0, "lune"),
        ("lune", 1, "lun"),
        ("automnal", 2, "automn"),
        ("lune", 4, ""),
        ("lune", 9, ""),
        ("", 3, ""),
        ("éveiller", 2, "éveill"),
        ("été", 1, "ét"),
        ("ete\u{301}", 1, "et"),
    ];
    for (word, n, expected) in cases.iter() {
        assert_eq!(&drop_last_graphemes(word, *n), expected, "{:?} {}", word, n);
    }
}

#[test]
fn uppercase_first_letter_table() {
    let cases = [
        ("", ""),
        ("lune", "Lune"),
        ("la lune calme", "La lune calme"),
        ("étoile", "Étoile"),
        ("âme", "Âme"),
        ("œillet", "Œillet"),
        ("l'étoile", "L'étoile"),
        ("Lune", "Lune"),
        ("LUNE", "LUNE"),
        // nothing to uppercase
        (" lune", " lune"),
        ("#err", "#err"),
        ("1 lune", "1 lune"),
        // works on the first CHAR (not grapheme), with its full uppercase mapping
        ("e\u{301}toile", "E\u{301}toile"),
        ("ßa", "SSa"),
    ];
    for (word, expected) in cases.iter() {
        assert_eq!(&uppercase_first_letter(word), expected, "{:?}", word);
    }
}

// ------------------------------------------------------------ random pickers
//
// Only the outcomes that do not depend on the draw: one candidate, no
// candidate, unknown id. Each case is repeated to make sure of that.

const REPEAT: usize = 50;

#[test]
fn rand_noun_edges() {
    let none: HashSet<NounId> = HashSet::new();
    let not_found = Err(String::from("err#get_rand_noun#Noun not found"));
    for _ in 0..REPEAT {
        let lemma = |nouns: &[NounId], blacklist: &HashSet<NounId>| {
            seam::rand_noun(&nouns.to_vec(), blacklist).map(|noun| noun.word.text)
        };
        assert_eq!(lemma(&[NounId::Lune], &none), Ok(String::from("lune")));
        assert_eq!(lemma(&[], &none), not_found);
        // everything blacklisted
        let all: HashSet<NounId> = HashSet::from_iter([NounId::Lune, NounId::Cerf].iter().cloned());
        assert_eq!(lemma(&[NounId::Lune, NounId::Cerf], &all), not_found);
        // the blacklist filters before the draw
        let lune: HashSet<NounId> = HashSet::from_iter([NounId::Lune].iter().cloned());
        assert_eq!(lemma(&[NounId::Lune, NounId::Cerf], &lune), Ok(String::from("cerf")));
        // CURRENT BEHAVIOUR: "raie" is declared with the id of "éclat"
        // (noun_data.rs), so NounId::Raie has no entry and NounId::Eclat is
        // always "éclat"
        assert_eq!(lemma(&[NounId::Raie], &none), not_found);
        assert_eq!(lemma(&[NounId::Eclat], &none), Ok(String::from("éclat")));
    }
}

#[test]
fn rand_adj_edges() {
    let none: HashSet<AdjId> = HashSet::new();
    let not_found = Err(String::from("err#get_rand_adj#Adjective not found"));
    for _ in 0..REPEAT {
        let lemma = |adjs: &[AdjId], blacklist: &HashSet<AdjId>| {
            seam::rand_adj(&adjs.to_vec(), blacklist).map(|adj| adj.word.text)
        };
        assert_eq!(lemma(&[AdjId::Bleu], &none), Ok(String::from("bleu")));
        assert_eq!(lemma(&[], &none), not_found);
        let bleu: HashSet<AdjId> = HashSet::from_iter([AdjId::Bleu].iter().cloned());
        assert_eq!(lemma(&[AdjId::Bleu], &bleu), not_found);
        assert_eq!(lemma(&[AdjId::Bleu, AdjId::Brun], &bleu), Ok(String::from("brun")));
        // CURRENT BEHAVIOUR: ids without an entry in ADJS ...
        for missing in [AdjId::Grand, AdjId::Diaphane, AdjId::Bigarre, AdjId::Doux].iter() {
            assert_eq!(lemma(&[*missing], &none), not_found);
        }
        // ... two of them because "bigarré" and "doux" are declared with the
        // ids of "halé" and "paisible" (adj_data.rs)
        assert_eq!(lemma(&[AdjId::Hale], &none), Ok(String::from("halé")));
        assert_eq!(lemma(&[AdjId::Paisible], &none), Ok(String::from("paisible")));
    }
}

#[test]
fn rand_verb_edges() {
    let none: HashSet<VerbId> = HashSet::new();
    let not_found = Err(String::from("err#get_rand_verb#Verb not found"));
    for _ in 0..REPEAT {
        let lemma = |verbs: &[VerbId], blacklist: &HashSet<VerbId>| {
            seam::rand_verb(&verbs.to_vec(), blacklist).map(|verb| verb.word.text)
        };
        assert_eq!(lemma(&[VerbId::Etre], &none), Ok(String::from("être")));
        assert_eq!(lemma(&[], &none), not_found);
        let etre: HashSet<VerbId> = HashSet::from_iter([VerbId::Etre].iter().cloned());
        assert_eq!(lemma(&[VerbId::Etre], &etre), not_found);
        assert_eq!(lemma(&[VerbId::Etre, VerbId::Rester], &etre), Ok(String::from("rester")));
    }
}

#[test]
fn rand_category_edges() {
    for _ in 0..REPEAT {
        let nouns = seam::rand_noun_cat(&vec![NounCatId::MiJour]).map(|cat| cat.nouns);
        assert_eq!(nouns, Ok(vec![NounId::Midi, NounId::Minuit]));
        assert_eq!(
            seam::rand_noun_cat(&vec![]).map(|cat| cat.nouns),
            Err(String::from("err#get_rand_cat_noun#Category not found"))
        );
        let adjs = seam::rand_adj_cat(&vec![AdjCatId::Saveur]);
        assert!(adjs == Ok(vec![AdjId::Sucre, AdjId::Epice]));
        assert!(
            seam::rand_adj_cat(&vec![])
                == Err(String::from("err#get_rand_adj_cat#Category not found"))
        );
    }
}

#[test]
fn rand_article_and_number_edges() {
    let mut articles = HashSet::new();
    let mut numbers = HashSet::new();
    for _ in 0..REPEAT * 20 {
        // one candidate
        assert_eq!(
            article_name(seam::rand_article(Some(&vec![Article::Definite]))),
            "Definite"
        );
        // no candidate: Indefinite
        assert_eq!(article_name(seam::rand_article(Some(&vec![]))), "Indefinite");
        // no list: any of the three; a number: any of the two
        articles.insert(article_name(seam::rand_article(None)));
        numbers.insert(super::names::number_name(seam::rand_number()));
    }
    // 1000 draws: missing one of 3 (or 2) values has probability < 1e-170
    assert_eq!(articles.len(), 3);
    assert_eq!(numbers.len(), 2);
}
