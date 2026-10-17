---
name: haiku-grammar-data
description: Add or change words, categories and sentence patterns of the haiku generator, and read what the change does to the set of producible lines. Use before editing src/*_data.rs, src/*_enums.rs or src/combination.rs, and when a word never appears in a haiku.
paths: "src/*_data.rs, src/*_enums.rs, src/combination.rs, src/combination_data.rs, src/noun.rs, src/adj.rs, src/verb.rs, src/wordgroup.rs"
---

# Words, categories, patterns

The poet draws from tables written by hand. A change here changes what the
page can say: it is its own commit, with the golden diff read.

## Where things are

| File                                             | Holds                                                                                                                                                                                                         |
| ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `noun_enums.rs`, `adj_enums.rs`, `verb_enums.rs` | The ids (`NounId`, `NounCatId`, …)                                                                                                                                                                            |
| `noun_data.rs`                                   | `NOUNS` (id, lemma, gender, feet) and `NOUN_CATS`: members of a category and its relations: `attributes` (after a linking verb), `epithets` (in the noun group), `functions` (verbs), `affiliations` (`de …`) |
| `adj_data.rs`, `verb_data.rs`                    | `ADJS`, `ADJ_CATS`, `VERBS`, `VERB_CATS`                                                                                                                                                                      |
| `combination.rs`, `combination_data.rs`          | The 5 sentence patterns and their order                                                                                                                                                                       |
| `noun.rs`, `adj.rs`, `verb.rs`, `wordgroup.rs`   | Agreement, articles, elision, the count of feet                                                                                                                                                               |

The table types carry their length (`[Noun; 71]`): the compiler says when it
is wrong.

## Add a word

1. A variant in the id enum.
2. A row in the table, **with that id**. Bump the array length.
3. The id in at least one category, or the word is never drawn.
4. A label in `src/characterization/names.rs` (the `match` is exhaustive: it
   does not compile without).
5. `UPDATE_GOLDEN=1 mise exec -- cargo test`, then read
   `git diff --stat tests/golden` and the diff: only lines with the new word.
6. Update the counts a few tests hold (`characterization/haiku.rs`), from
   their failure messages.

Feet are `(min, max)`: a final mute `e` gives two readings. Look at a
neighbour of the same ending in `tests/golden/nouns.txt` before choosing.

## Traps

| Symptom                                        | Cause                                                                                                                                                                     | Fix                                                                                   |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| A word never appears                           | Its row carries another word's id (copy-paste). Lookup is by id: the first row wins, the id it should have has no entry, and every draw of it fails silently into a retry | Compare ids and lemmas in `tests/golden/nouns.txt`: `lookup_by_id` shows the mismatch |
| Still so today                                 | "bigarré" has the id of "hâlé", "doux" the id of "paisible"; `AdjId::Grand` and `AdjId::Diaphane` sit in categories without a row                                         | Left as in 2020; fixing each adds lines, as "raie" did                                |
| A shorter word makes haikus fail less          | The longest pattern rarely has 5 feet; the retry loop gives up after 10,001 tries and writes `#err#…`                                                                     | Expected; `the_same_pattern_can_serve_two_lines` holds the probability                |
| A category "fix" changes nothing or everything | The season rules (`adjust_blacklist`) work on category ids                                                                                                                | Read `common_enums.rs` before renaming a category                                     |

Not to touch in passing: the elision letters (`wordgroup.rs`), the plural and
feminine rules, `has_intransitive_verb`. Each one is wrong in some case and
every producible line depends on it.
