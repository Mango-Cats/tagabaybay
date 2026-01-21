# ᜆᜄᜊᜌ᜔ᜊᜌ᜔ / TagaBaybay: Rules

**Prerequisites**:

- [context-sensitive grammars (CSG)](https://en.wikipedia.org/wiki/Context-sensitive_grammar)
- [finite-state transducer (FST)](https://en.wikipedia.org/wiki/Finite-state_transducer)

**Source Code:**

- [Cursor](/src/adaptation/cursor.rs)

An input loanword $w \in L_2$ goes through different types of rewrite
rules defined in TagaBaybay. Which type it goes through depends on
the dimension required. For instance, the word "red" is simple enough
that a context-free (CF) rewrite is needed. But for, cases like
"ideal" and "idea" the infix "dea" might be difficult to process
since theyre pronounced differently.

Here we introduce the different types of rewrite rules defined in
TagaBaybay, how to write them, and when they're invoked.

## preliminaries: the $\mathrm{Cursor}$ type

The rewrite rules primarily use $\mathrm{ctx}:\mathrm{Cursor}$. This
is a compound type

$$
\begin{align*}
    \mathrm{grapheme}&:\mathrm{Vec}\langle G \rangle, \\
    \mathrm{phonemes}&:\mathrm{Vec}\langle P \rangle, \\
    \mathrm{index}&:\mathrm{usize}
\end{align*}
$$

There is no need for us to know what the types $G$ and $P$ are hence
they're abstracted away. But, $G \ne P$.

Furthermore, $\mathrm{ctx}$ defines the following associated
functions:

- $\mathrm{current\_grapheme}$: which returns $\mathrm{ctx.grapheme}_\mathrm{ctx.index}$
- $\mathrm{prev\_grapheme}$: which returns $\mathrm{Some}(\mathrm{ctx.grapheme}_\mathrm{ctx.index})$ if $0 < \mathrm{ctx.index}$. Otherwise, $\mathrm{None}$.
- $\mathrm{next\_grapheme}$: which returns $\mathrm{Some}(\mathrm{ctx.grapheme}_\mathrm{ctx.index})$ if $\mathrm{ctx.index} < \mathrm{len}(\mathrm{ctx.grapheme})$. Otherwise, $\mathrm{None}$.
- $\mathrm{lookat\_grapheme}$: which returns $\mathrm{Some}(\mathrm{ctx.grapheme}_\mathrm{ctx.index})$ if $0 < \mathrm{ctx.index} < \mathrm{len}(\mathrm{ctx.grapheme})$. Otherwise, $\mathrm{None}$.

Finally, for the grapheme functions, each have a corresponding case
preserving and non-case preserving functions. That is, if
$\mathrm{ctx.grapheme}_\mathrm{ctx.index} = ``\mathrm{A}"$ then
$\mathrm{current\_grapheme} = ``\mathrm{A}"$ (case preserved)
and $\mathrm{current\_grapehem\_low} = ``\mathrm{a}"$
(lowercased). And, we also defined the same functions for the
phonemes.

## dimension: orthographic

**Source code:**
[`adaptation/orthographic`](/src/adaptation/orthographic)

Strings can be adapted by only using its orthography or how
it is written. Examples are words like "rack" and "race". Which are
simply enough, that by looking at it, we can tell how it is
pronounced.

But, look at the pronunciation of 'c' in both words. The 'c' in
"rack" is pronounced as /k/ while it is /s/ in "race".

This is where context comes into play -- there are some letters in
the English language that have multiple pronunciations. But, we can
easily know which one we pick by looking at the context at which that
character exists.

Using the two examples above, we see that the 'c' in "rack" is
immediately followed by a 'k', and, in "race" it is immediately
followed by a 'e'. Perhaps, we can define that as the context
for rewriting 'c':

$$
\begin{align*}
& i := \mathrm{ctx.current\_grapheme} \\
& j := \mathrm{ctx.next\_grapheme} \\
\end{align*}
$$

then,

$$
\Theta_\mathrm{c}(i,j) = \begin{cases}
/\mathrm{k}/, \quad i = \mathrm{``c"} \wedge j = \mathrm{``k"} \\
/\mathrm{se}/, \quad i = \mathrm{``c"} \wedge j = \mathrm{''e"}

\end{cases}
$$

Or, the rewrite $\Theta_\mathrm{c}$ rewrites a pair of
characters $(i,j)$ to (a single sound) /k/ given "ck". If, instead
given, "ce" rewrite to /se/.

Notice that some rewrites may consume or reduce a contiguous
sequence to a single or reduced phoneme string. In the example
above, the string "ck" (with length 2) reduced to the string "k"
(with length 1).

The rewrite $\Theta_\mathrm{c}$ is an example of a
**context-sensitive** rewrite, since we require additional context(in this case, the character $j$) to perform the rewrite. Rewrites that only take in a single character, or do not require additional context, are known as **context-free**.

## invariants

For any symbol $\sigma\in \Sigma$, if there doesn't exist a $\Theta_\sigma(i, \alpha, \dots, \beta)$, then we say that $\sigma$ is an **invariant**. That is, there doesnt exist a context-sensitive rewrite for that $\sigma$: there is a unique context-free rewrite.

## context-predictable variants

For any symbol $\gamma\in\Sigma$, if there exists $\Theta_\gamma(i, \alpha, \dots,\beta)$, then we say that $\gamma$ is a context-predictable variant. That is, $\gamma$ has different variants for its pronunciation but is predictable given the context $\alpha,\dots,\beta$.

---

Going back to the original "race" and "rack" example. Notice that 'a' in "race" is pronounced as /ei/ while it is pronounced as /a/ in "rack". Perhaps the context "ace" allows us to differentiate /a/ and /ei/. But the word "menace" contradicts that (here the "ace" is pronounced as /uh.s/).

## context-unpredictable variants

For any symbol $\omega\in\Sigma$, if it is impossible to construct a
$\Theta_\omega(i,\alpha,\dots,\beta)$ that always gives the admissible rewrite of $\omega$ given the context $\alpha,\dots,\beta$. Or, it would require to enumerate through all possible contexts where $\omega$ could exist. Then we say that $\omega$ is an context-unpredictable invariant. That is, the pronunciation of $\omega$ does not depend on its orthographic context. Hence, another dimension of loanword adaptation is required.

## dimension: phonetics

**Source code:**
[`adaptation/phonetic`](/src/adaptation/phonetic/)

A subset of symbols in $\Sigma$ cannot be rewritten only using its orthography -- requiring another dimension for adaptation. Of course, considering how a word is pronounced would answer the question about how it is adapted. However, English-centric G2P may lead to errors [invariant](#invariants) or [predictable variants](#context-predictable-variants) characters. So, as to avoid this, we will limit ourselves to using G2P on unpredictable invariants.
