// Characterization (golden-master) tests.
//
// They describe what the crate does TODAY, bugs included, so that later
// dependency / edition upgrades can be proven behaviour-preserving. Nothing in
// here is a specification: when a test fails after a refactoring, either the
// refactoring changed the behaviour, or the change is intended and the golden
// has to be regenerated with
//
//     UPDATE_GOLDEN=1 mise exec -- cargo test
//
// Layout
//   golden   text-file golden helper (tests/golden/*.txt)
//   seam     the only place that builds the production (rand) Chooser
//   odometer a Chooser that walks a decision tree exhaustively; tests of the
//            seam itself (choose, shuffle)
//   names    labels for the id enums (they do not derive Debug)
//   labels   lemma labels and formatting shared by the dumps
//   grammar  exhaustive dumps of the grammar over the data tables
//   helpers  table tests of the pure helpers
//   enumeration  everything each sentence pattern can produce (odometer
//            over the real closures)
//   patterns sentence patterns: enumeration and Monte-Carlo against goldens
//   haiku    properties of generate_haiku
//   retry    the retry / reshuffle loop of generate_haiku, scripted
//
// The module is declared from src/lib.rs; src/main.rs is the CLI over the
// library and has no test of its own.

mod golden;
mod labels;
mod names;
mod seam;

mod enumeration;
mod grammar;
mod haiku;
mod helpers;
mod odometer;
mod patterns;
mod retry;
