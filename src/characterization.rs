// Characterization (golden-master) tests.
//
// They describe what the crate does TODAY, bugs included, so that later
// dependency / edition upgrades can be proven behaviour-preserving. Nothing in
// here is a specification: when a test fails after a refactoring, either the
// refactoring changed the behaviour, or the change is intended and the golden
// has to be regenerated with
//
//     UPDATE_GOLDEN=1 cargo test
//
// Layout
//   golden   text-file golden helper (tests/golden/*.txt)
//   seam     the only place that touches `rand`
//   names    labels for the id enums (they do not derive Debug)
//   labels   lemma labels and formatting shared by the dumps
//   grammar  exhaustive dumps of the grammar over the data tables
//   helpers  table tests of the pure helpers
//   model    exhaustive enumeration of what each sentence pattern can produce
//   patterns sentence patterns: enumeration and Monte-Carlo against goldens
//   haiku    properties of generate_haiku
//
// The module is only declared from src/main.rs (bin target): declaring it in
// src/lib.rs too would run every test twice.

mod golden;
mod labels;
mod names;
mod seam;

mod grammar;
mod haiku;
mod helpers;
mod model;
mod patterns;
