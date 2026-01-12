use crate::nativization::context::Context;
use crate::tokenization::phoneme::FilipinoGrapheme;

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
//                 return some((vec![phoneme::e, phoneme::y, phoneme::t], 3));
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
//         some(grapheme::i) => some((vec![phoneme::i], 2)),
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
//                 return some((vec![phoneme::a, phoneme::y, phoneme::d], 3));
//             }
//         }
//     }

//     // regular i + vowel patterns
//     match ctx.next() {
//         some(grapheme::a) => some((vec![phoneme::i, phoneme::y, phoneme::a], 2)),
//         some(grapheme::e) => some((vec![phoneme::i, phoneme::y, phoneme::e], 2)),
//         some(grapheme::o) => some((vec![phoneme::i, phoneme::y, phoneme::o], 2)),
//         some(grapheme::u) => some((vec![phoneme::i, phoneme::y, phoneme::u], 2)),
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
//                 return some((vec![phoneme::o, phoneme::w, phoneme::n], 3));
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
//                         phoneme::o,
//                         phoneme::y,
//                         match vowel {
//                             grapheme::a => phoneme::a,
//                             grapheme::e => phoneme::e,
//                             grapheme::i => phoneme::i,
//                             grapheme::o => phoneme::o,
//                             grapheme::u => phoneme::u,
//                             _ => phoneme::other,
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
//         some(grapheme::a) => some((vec![phoneme::u, phoneme::w, phoneme::a], 2)),
//         some(grapheme::e) => some((vec![phoneme::u, phoneme::w, phoneme::e], 2)),
//         some(grapheme::i) => some((vec![phoneme::u, phoneme::w, phoneme::i], 2)),
//         some(grapheme::o) => some((vec![phoneme::u, phoneme::w, phoneme::o], 2)),
//         some(grapheme::u) => some((vec![phoneme::u, phoneme::w, phoneme::u], 2)),
//         _ => match ctx.prev() {
//             some(grapheme::e) => some((vec![phoneme::y, phoneme::u], 1)),
//             _ => none,
//         },
//     }
// }
