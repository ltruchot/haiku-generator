// Test-only labels for the id enums.
//
// Most id enums of the crate do not derive `Debug`, and part A must not add
// derives to production code, so the names are spelled out here. Every match
// is exhaustive on purpose: adding, removing or renaming a variant breaks the
// build of the characterization tests instead of silently changing a golden.

use crate::adj_enums::{AdjCatId, AdjId};
use crate::common_enums::{Article, Gender, Number};
use crate::noun_enums::{NounCatId, NounId};
use crate::verb_enums::{VerbCatId, VerbGroup, VerbId, VerbKind};

pub fn noun_id_name(id: NounId) -> &'static str {
    match id {
        NounId::Lune => "Lune",
        NounId::Soleil => "Soleil",
        NounId::Etoile => "Etoile",
        NounId::Bruit => "Bruit",
        NounId::Chant => "Chant",
        NounId::Bruissement => "Bruissement",
        NounId::Lumiere => "Lumiere",
        NounId::Rayon => "Rayon",
        NounId::Odeur => "Odeur",
        NounId::Parfum => "Parfum",
        NounId::Arome => "Arome",
        NounId::Printemps => "Printemps",
        NounId::Ete => "Ete",
        NounId::Automne => "Automne",
        NounId::Hiver => "Hiver",
        NounId::Prunier => "Prunier",
        NounId::Cerisier => "Cerisier",
        NounId::Oeillet => "Oeillet",
        NounId::Glycine => "Glycine",
        NounId::Pivoine => "Pivoine",
        NounId::Feuille => "Feuille",
        NounId::Branche => "Branche",
        NounId::Aurore => "Aurore",
        NounId::Crepuscule => "Crepuscule",
        NounId::Midi => "Midi",
        NounId::Minuit => "Minuit",
        NounId::Mousse => "Mousse",
        NounId::Liane => "Liane",
        NounId::Lierre => "Lierre",
        NounId::Chevrefeuille => "Chevrefeuille",
        NounId::Petale => "Petale",
        NounId::Etamine => "Etamine",
        NounId::Alouette => "Alouette",
        NounId::Mesange => "Mesange",
        NounId::Grive => "Grive",
        NounId::Canard => "Canard",
        NounId::RougeGorge => "RougeGorge",
        NounId::Fauvette => "Fauvette",
        NounId::Hirondelle => "Hirondelle",
        NounId::Merle => "Merle",
        NounId::Pie => "Pie",
        NounId::Cerf => "Cerf",
        NounId::Biche => "Biche",
        NounId::Faon => "Faon",
        NounId::Ecureuil => "Ecureuil",
        NounId::Belette => "Belette",
        NounId::Hermine => "Hermine",
        NounId::Daim => "Daim",
        NounId::Chevreuil => "Chevreuil",
        NounId::Renard => "Renard",
        NounId::Plume => "Plume",
        NounId::Reflet => "Reflet",
        NounId::Eclat => "Eclat",
        NounId::Raie => "Raie",
        NounId::Moment => "Moment",
        NounId::Heure => "Heure",
        NounId::Minute => "Minute",
        NounId::Calme => "Calme",
        NounId::Paix => "Paix",
        NounId::Serenite => "Serenite",
        NounId::Pluie => "Pluie",
        NounId::Bruine => "Bruine",
        NounId::Rosee => "Rosee",
        NounId::Orage => "Orage",
        NounId::Averse => "Averse",
        NounId::Embellie => "Embellie",
        NounId::Brume => "Brume",
        NounId::Saveur => "Saveur",
        NounId::Coeur => "Coeur",
        NounId::Ame => "Ame",
        NounId::Esprit => "Esprit",
    }
}

pub fn noun_cat_name(id: NounCatId) -> &'static str {
    match id {
        NounCatId::Astre => "Astre",
        NounCatId::Phenomene => "Phenomene",
        NounCatId::PhenomeneLumineux => "PhenomeneLumineux",
        NounCatId::EffetLumineux => "EffetLumineux",
        NounCatId::PhenomeneSonore => "PhenomeneSonore",
        NounCatId::PhenomeneSonoreFloral => "PhenomeneSonoreFloral",
        NounCatId::PhenomeneOlfactif => "PhenomeneOlfactif",
        NounCatId::Saison => "Saison",
        NounCatId::SaisonChaude => "SaisonChaude",
        NounCatId::SaisonFroide => "SaisonFroide",
        NounCatId::PlanteAFleur => "PlanteAFleur",
        NounCatId::OrganeDePlante => "OrganeDePlante",
        NounCatId::MomentDuJour => "MomentDuJour",
        NounCatId::MiJour => "MiJour",
        NounCatId::Plante => "Plante",
        NounCatId::Oiseau => "Oiseau",
        NounCatId::Mammifere => "Mammifere",
        NounCatId::PartieDOiseau => "PartieDOiseau",
        NounCatId::PortionDeTemps => "PortionDeTemps",
        NounCatId::Calme => "Calme",
        NounCatId::PhenomeneMeteo => "PhenomeneMeteo",
        NounCatId::PhenomeneTactile => "PhenomeneTactile",
        NounCatId::EspritHumain => "EspritHumain",
    }
}

pub fn adj_id_name(id: AdjId) -> &'static str {
    match id {
        AdjId::EnFleur => "EnFleur",
        AdjId::Sauvage => "Sauvage",
        AdjId::Libre => "Libre",
        AdjId::Printanier => "Printanier",
        AdjId::Estival => "Estival",
        AdjId::Automnal => "Automnal",
        AdjId::Hivernal => "Hivernal",
        AdjId::Violet => "Violet",
        AdjId::Orange => "Orange",
        AdjId::Brun => "Brun",
        AdjId::Dore => "Dore",
        AdjId::Argente => "Argente",
        AdjId::Roux => "Roux",
        AdjId::Ecarlate => "Ecarlate",
        AdjId::Ocre => "Ocre",
        AdjId::Pastel => "Pastel",
        AdjId::Bleu => "Bleu",
        AdjId::Rose => "Rose",
        AdjId::Diaphane => "Diaphane",
        AdjId::Diapre => "Diapre",
        AdjId::Moire => "Moire",
        AdjId::Irise => "Irise",
        AdjId::Hale => "Hale",
        AdjId::Bigarre => "Bigarre",
        AdjId::Grand => "Grand",
        AdjId::Immense => "Immense",
        AdjId::Majestueux => "Majestueux",
        AdjId::Noble => "Noble",
        AdjId::Delicat => "Delicat",
        AdjId::Leger => "Leger",
        AdjId::Doux => "Doux",
        AdjId::Inquiet => "Inquiet",
        AdjId::AuxAbois => "AuxAbois",
        AdjId::Vigilant => "Vigilant",
        AdjId::Perplexe => "Perplexe",
        AdjId::Agite => "Agite",
        AdjId::Emu => "Emu",
        AdjId::Calme => "Calme",
        AdjId::Sage => "Sage",
        AdjId::Heureux => "Heureux",
        AdjId::Immobile => "Immobile",
        AdjId::EnPaix => "EnPaix",
        AdjId::Paisible => "Paisible",
        AdjId::Caniculaire => "Caniculaire",
        AdjId::Brulant => "Brulant",
        AdjId::Glace => "Glace",
        AdjId::Glacial => "Glacial",
        AdjId::Polaire => "Polaire",
        AdjId::Moqueur => "Moqueur",
        AdjId::Malicieux => "Malicieux",
        AdjId::Narquois => "Narquois",
        AdjId::Railleur => "Railleur",
        AdjId::Noir => "Noir",
        AdjId::Sombre => "Sombre",
        AdjId::Gris => "Gris",
        AdjId::Ebene => "Ebene",
        AdjId::Anonyme => "Anonyme",
        AdjId::Etrange => "Etrange",
        AdjId::Etranger => "Etranger",
        AdjId::Epice => "Epice",
        AdjId::Sucre => "Sucre",
    }
}

pub fn adj_cat_name(id: AdjCatId) -> &'static str {
    match id {
        AdjCatId::EtatDeFloraison => "EtatDeFloraison",
        AdjCatId::Liberte => "Liberte",
        AdjCatId::RelAUneSaison => "RelAUneSaison",
        AdjCatId::ColorationTexture => "ColorationTexture",
        AdjCatId::ColorationDiapree => "ColorationDiapree",
        AdjCatId::ColorationBleue => "ColorationBleue",
        AdjCatId::ColorationRousse => "ColorationRousse",
        AdjCatId::ColorationSombre => "ColorationSombre",
        AdjCatId::ColorationBrillante => "ColorationBrillante",
        AdjCatId::Grandeur => "Grandeur",
        AdjCatId::Noblesse => "Noblesse",
        AdjCatId::Delicatesse => "Delicatesse",
        AdjCatId::Agitation => "Agitation",
        AdjCatId::Calme => "Calme",
        AdjCatId::Chaleur => "Chaleur",
        AdjCatId::Froideur => "Froideur",
        AdjCatId::CaractereMoqueur => "CaractereMoqueur",
        AdjCatId::Anonymat => "Anonymat",
        AdjCatId::Saveur => "Saveur",
    }
}

pub fn verb_id_name(id: VerbId) -> &'static str {
    match id {
        VerbId::Sembler => "Sembler",
        VerbId::Demeurer => "Demeurer",
        VerbId::Rester => "Rester",
        VerbId::Etre => "Etre",
        VerbId::AvoirLAir => "AvoirLAir",
        VerbId::Paraitre => "Paraitre",
        VerbId::SAssoupir => "SAssoupir",
        VerbId::SEveiller => "SEveiller",
        VerbId::SEclipser => "SEclipser",
        VerbId::SeCoucher => "SeCoucher",
        VerbId::SeLever => "SeLever",
        VerbId::SEvanouir => "SEvanouir",
        VerbId::SEndormir => "SEndormir",
    }
}

pub fn verb_cat_name(id: VerbCatId) -> &'static str {
    match id {
        VerbCatId::Etat => "Etat",
        VerbCatId::EtatDEveil => "EtatDEveil",
    }
}

pub fn verb_group_name(id: VerbGroup) -> &'static str {
    match id {
        VerbGroup::First => "First",
        VerbGroup::Second => "Second",
        VerbGroup::Third => "Third",
    }
}

pub fn verb_kind_name(id: VerbKind) -> &'static str {
    match id {
        VerbKind::Intransitive => "Intransitive",
        VerbKind::Copule => "Copule",
    }
}

pub fn gender_name(id: Gender) -> &'static str {
    match id {
        Gender::Female => "Female",
        Gender::Male => "Male",
    }
}

pub fn number_name(id: Number) -> &'static str {
    match id {
        Number::Plural => "Plural",
        Number::Singular => "Singular",
    }
}

pub fn article_name(id: Article) -> &'static str {
    match id {
        Article::None => "None",
        Article::Definite => "Definite",
        Article::Indefinite => "Indefinite",
    }
}
