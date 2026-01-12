use crate::nativization::context::Context;
use crate::tokenization::phl_graphemes::FilipinoGrapheme;

pub fn handle_vowel(ctx: &Context) -> Option<(Vec<FilipinoGrapheme>, usize)> {
    let curr = ctx.current();

    // dbg!(&ctx.ipa);
    // look for the specific ipa transcription the vowel/curr is referring to
    None
}

// /// handle 'a' vowel patterns
// ///
// /// # arguments
// ///
// /// * `ctx` - context containing the grapheme sequence and current position
// ///
// /// # returns
// ///
// /// returns `some((phonemes, consumed))` if a pattern matches, `none` otherwise.
// fn handle_vowel_a(ctx: &context) -> option<(vec<phoneme>, usize)> {
//     // check for "ate" pattern (a-t-e at end) → "eyt"
//     if let some(grapheme::t) = ctx.next() {
//         if let some(grapheme::e) = ctx.lookahead(2) {
//             if ctx.position() + 2 == ctx.graphemes.len() - 1 {
//                 return some((vec![phl_graphemes::e, phl_graphemes::y, phl_graphemes::t], 3));
//             }
//         }
//     }
//     none
// }

// /// handle 'e' vowel patterns
// ///
// /// # arguments
// ///
// /// * `ctx` - context containing the grapheme sequence and current position
// ///
// /// # returns
// ///
// /// returns `some((phonemes, consumed))` if a pattern matches, `none` otherwise.
// fn handle_vowel_e(ctx: &context) -> option<(vec<phoneme>, usize)> {
//     // remove trailing 'e'
//     if ctx.at_end() {
//         return some((vec![], 1));
//     }

//     // ei -> i (consume both e and i)
//     match ctx.next() {
//         some(grapheme::i) => some((vec![phl_graphemes::i], 2)),
//         _ => none,
//     }
// }

// /// handle 'i' vowel patterns
// ///
// /// # arguments
// ///
// /// * `ctx` - context containing the grapheme sequence and current position
// ///
// /// # returns
// ///
// /// returns `some((phonemes, consumed))` if a pattern matches, `none` otherwise.
// fn handle_vowel_i(ctx: &context) -> option<(vec<phoneme>, usize)> {
//     // check for "ide" pattern (i-d-e at end) → "ayd"
//     if let some(grapheme::d) = ctx.next() {
//         if let some(grapheme::e) = ctx.lookahead(2) {
//             if ctx.position() + 2 == ctx.graphemes.len() - 1 {
//                 return some((vec![phl_graphemes::a, phl_graphemes::y, phl_graphemes::d], 3));
//             }
//         }
//     }

//     // regular i + vowel patterns
//     match ctx.next() {
//         some(grapheme::a) => some((vec![phl_graphemes::i, phl_graphemes::y, phl_graphemes::a], 2)),
//         some(grapheme::e) => some((vec![phl_graphemes::i, phl_graphemes::y, phl_graphemes::e], 2)),
//         some(grapheme::o) => some((vec![phl_graphemes::i, phl_graphemes::y, phl_graphemes::o], 2)),
//         some(grapheme::u) => some((vec![phl_graphemes::i, phl_graphemes::y, phl_graphemes::u], 2)),
//         _ => none,
//     }
// }

// /// handle 'o' vowel patterns
// ///
// /// # arguments
// ///
// /// * `ctx` - context containing the grapheme sequence and current position
// ///
// /// # returns
// ///
// /// returns `some((phonemes, consumed))` if a pattern matches, `none` otherwise.
// fn handle_vowel_o(ctx: &context) -> option<(vec<phoneme>, usize)> {
//     // check for "one" pattern (o-n-e at end) → "own"
//     if let some(grapheme::n) = ctx.next() {
//         if let some(grapheme::e) = ctx.lookahead(2) {
//             if ctx.position() + 2 == ctx.graphemes.len() - 1 {
//                 return some((vec![phl_graphemes::o, phl_graphemes::w, phl_graphemes::n], 3));
//             }
//         }
//     }

//     match ctx.next() {
//         some(vowel) if vowel.is_vowel() => {
//             // o + vowel -> oy + vowel (unless next is also a vowel)
//             match ctx.lookahead(2) {
//                 some(v) if v.is_vowel() => none,
//                 _ => some((
//                     vec![
//                         phl_graphemes::o,
//                         phl_graphemes::y,
//                         match vowel {
//                             grapheme::a => phl_graphemes::a,
//                             grapheme::e => phl_graphemes::e,
//                             grapheme::i => phl_graphemes::i,
//                             grapheme::o => phl_graphemes::o,
//                             grapheme::u => phl_graphemes::u,
//                             _ => phl_graphemes::other,
//                         },
//                     ],
//                     2,
//                 )),
//             }
//         }
//         _ => none,
//     }
// }

// /// handle 'u' vowel patterns
// ///
// /// # arguments
// ///
// /// * `ctx` - context containing the grapheme sequence and current position
// ///
// /// # returns
// ///
// /// returns `some((phonemes, consumed))` if a pattern matches, `none` otherwise.
// fn handle_vowel_u(ctx: &context) -> option<(vec<phoneme>, usize)> {
//     match ctx.next() {
//         some(grapheme::a) => some((vec![phl_graphemes::u, phl_graphemes::w, phl_graphemes::a], 2)),
//         some(grapheme::e) => some((vec![phl_graphemes::u, phl_graphemes::w, phl_graphemes::e], 2)),
//         some(grapheme::i) => some((vec![phl_graphemes::u, phl_graphemes::w, phl_graphemes::i], 2)),
//         some(grapheme::o) => some((vec![phl_graphemes::u, phl_graphemes::w, phl_graphemes::o], 2)),
//         some(grapheme::u) => some((vec![phl_graphemes::u, phl_graphemes::w, phl_graphemes::u], 2)),
//         _ => match ctx.prev() {
//             some(grapheme::e) => some((vec![phl_graphemes::y, phl_graphemes::u], 1)),
//             _ => none,
//         },
//     }
// }
