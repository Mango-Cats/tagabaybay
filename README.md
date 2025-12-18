# ᜆᜄᜊᜌ᜔ᜊᜌ᜔ / TagaBaybay 

A phonetic nativization algorithm for Filipino loanwords. Written in Rust.

## Project Team

**Project Lead**: Zhean Robby Ganituen △◇
**Faculty Supervisor**: Nathaniel Oco △◇
**Project Members**:
- Clarance Ivan Ang △
- Roan Cedric Campo △
- Justin Ethan Ching ◇
- Erin Gabrielle Chua ◇
- Jaztn Jacob Jimenez ◇
- Clive Jarel Ang △

\***Correspondence:** `zr.gntn AT gmail DOT com`
△ Part of the legacy project.
◇ Part of the current project.


## Motivation

Working in NLP in the Philippine context is challenging due to
the multilingual nature of Filipinos and their frequent tendency to 
code-switch and borrow. Code-switching can occur through direct
translation, for example:

```
    Pupunta  ako  sa   paaralan  mamaya
    go.FUT   1SG  LOC  school    later

    "I will go to school later"
```

Alternatively, code-switching can involve the phonetic nativization of
loanwords, as in:

```
    Pakuha    nga  'yung  selpon     ko
    get.IMPV  EMP  DEF    cellphone  1SG.POSS

    "(Please) get my cellphone"
```

Although there are linguistic descriptions and rules for phonetic nativization
`[1]`, there is currently no software capable of automatically generating phonetic
nativizations of English loanwords. This gap limits Taglish language processing,
downstream NLP tasks, and broader computational work involving Philippine languages. 

## Algorithm

Our approach treats the process of phonetic nativization as a mapping, specifically
a rule-based procedural rewrite system. For an input word $w$ in the English language
$\mathcal{E}$, the function $M$ maps $w$ to $M(w) \in \mathcal{F}$, where $M(w)$
represents the phonetic nativization of $w$. Formally, 
$M: \mathcal{E} \mapsto\mathcal{F}$.

In practice, however, phonetic nativization is not strictly bijective. A single
$w$ may correspond to multiple plausible candidates for $M(w)$, depending on
factors such as speaker accent, exposure to the source language, or minor
differences in the original pronunciation. To resolve this ambiguity, we rely on
the resources in `[1]` as the authoritative reference for the final mapping.

Moreover, most rules for phonetic nativization are context-sensitive, relying
on surrounding graphemes or symbols to determine the appropriate phoneme output.

### Implementation

The Algorithm is implemented in the Rust programming language.

### Evaluation

The Algorithm is compared against gold standards.

See the [`gold/`](gold/) directory and its corresponding [`README`](gold/README.md)
for details.

### Legacy

This project was initially a final course project by Clarence, Zhean, Roan, and
Clive for Nathaniel's Natural Language Processing class at De La Salle University.

See the [`legacy/`](legacy/) directory and its corresponding [`README`](legacy/README)
for details.

## License

TagaBaybay is (as of December 15, 2025) is a private project, but it is distributed
under the terms of Apache License Version 2.0.

See [LICENSE](LICENSE) for details.

## Citation

If you use this software in your research, consider citing our work.

BibTeX:
```bibtex
@software{Zhean_Robby_TagaBaybay,
    author = {Zhean Robby, Ganituen and Clarence Ivan, Ang and Roan Cedric, Campo and Justin Ethan, Ching and Chua, Erin Gabrielle and Jaztin Jacob, Jimenez and Clive Jarel, Ang and Nathaniel, Oco},
    license = {Apache-2.0},
    title = {{TagaBaybay}},
    url = {https://github.com/Mango-Cats/tagabaybay/}
}
```

Also see [`CITATION.cff`](/CITATION.cff).

## Bibliography

`[1]` Virgilio S. Almario (Ed.). 2014. KWF Manwal sa Masinop na Pagsulat. Komisyon sa Wikang Filipino, Quezon City.
