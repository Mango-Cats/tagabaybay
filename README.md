# ᜆᜄᜊᜌ᜔ᜊᜌ᜔ / TagaBaybay

A loanword adaptation algorithm for Filipino loanwords.
Written in Rust.

## Project Team

**Project Lead**: Zhean Robby Ganituen △◇ <br>
**Faculty Supervisor**: Nathaniel Oco △◇ <br>
**Project Members**:

- Clarance Ivan Ang △
- Roan Cedric Campo △
- Erin Gabrielle Chua ◇
- Jaztin Jacob Jimenez ◇
- Justin Ethan Ching ◇
- Clive Jarel Ang △

\***Correspondence:** `zr.gntn AT gmail DOT com` <br>
△ Part of the legacy project. <br>
◇ Part of the current project.

## Motivation

**Code-switching**, refers to the alternation between two
or more languages within a single utterance. For example,
we can use the loanword *directly*, as in:

```text
    Pupunta  ako  sa   paaralan  mamaya
    go.FUT   1SG  LOC  school    later

    "I will go to school later"
```

Or, *adapt the loanword* using the Phonetics of the first
language (which, in this case, is Filipino).

```text
    Pakuha    nga  'yung  selpon     ko
    get.IMPV  EMP  DEF    cellphone  1SG.POSS

    "(Please) get my cellphone"
```

Despite its prevalence in everyday communication, relatively little
attention has been given to NLP methodologies that operate on
code-switched inputs. One central challenge in processing
code-switched Filipino speech is **loanword adaptation**: the
transformation of foreign lexical items into phonetic forms that
conform to Filipino pronunciation patterns. Loanword adaptation is
essential for speech-centered Filipino NLP tasks, including
text-to-speech synthesis and speech confusibility modeling.

From a computational perspective, loanword adaptation can be viewed
as a transduction pipeline:
$$\Sigma_s^* \mapsto \Phi_s^* \mapsto \Phi_F^* \mapsto \Sigma_F^*$$

Where:

- $\Sigma_s$ is the orthographic alphabet of the source language S.
- $\Phi_s$ is the phoneme inventory of S.
- $\Phi_F$ is the phoneme inventory of Filipino F.
- $\Sigma_F$ is the orthographic alphabet of F.

The mapping $\Sigma_s^{\ast} \mapsto \Phi_s^{\ast}$ is G2P in $S$.
In the context of Filipino loanword adaptation, since the source language
is usually English, Spanish, or Chinese, this is a surjection due to homophones.
The mapping $\Phi_s^{\ast} \rightarrow \Phi_F^{\ast}$ is the core loanword
adaptation function. The final mapping $\Phi_F^{\ast} \rightarrow \Sigma_F^{\ast}$
is an optional P2G step that renders the adapted phoneme sequence in Filipino
orthography.

While G2P and P2G modeling have been extensively studied,
computational approaches to the phonological loanword adaptation
step remain comparatively underexplored, particularly in the
context of code-switched speech.

## Algorithm

> For more information, see [`docs/algorithm`](docs/algorithm.md).

Our approach treats the process of loanword adaptation as a
mapping, specifically a procedural rule-based rewrite system. For
an input word $\omega$ in the source language $L_1$, the
algorithm $A$ maps $\omega$ to $A(\omega) \in \Sigma_F^*$.

### Implementation

The Algorithm is implemented in the Rust programming language.

### Evaluation

The Algorithm is compared against gold standards.

See the [`gold/`](gold/) directory and its corresponding
[`README`](gold/README.md) for details.

### Legacy

This project was initially a final course project by Clarence,
Zhean, Roan, and Clive for Nathaniel Oco's Natural Language
Processing class at De La Salle University.

See the [`legacy/`](legacy/) directory and its corresponding
[`README`](legacy/README) for details.

## Usage

**Requirements:**

1. **[uv](https://docs.astral.sh/uv/)** - Python package manager (handles phonemizer dependencies automatically). [GitHub](https://github.com/astral-sh/uv).
2. **eSpeak-NG** - Speech synthesizer backend. [User Guide](https://github.com/espeak-ng/espeak-ng/blob/master/docs/guide.md).

**Important for Windows users:** You must set the `ESPEAK_LIB` environment variable to point to the eSpeak-NG DLL:

```cmd
:: Command Prompt
set ESPEAK_LIB=<Your/Path/To/DLL>

:: Or permanently via PowerShell
[Environment]::SetEnvironmentVariable("ESPEAK_LIB", "<Your/Path/To/DLL>", "User")
```

**To run:**

```cmd
cargo r
```

## License

TagaBaybay is (as of December 15, 2025) is a private project, but it is distributed
under the terms of Apache License Version 2.0.

See [LICENSE](LICENSE) for details.

## Citation

If you use this software in your research, consider citing our work.

BibTeX:

```bibtex
@software{Ganituen_TagaBaybay,
    author = {Ganituen, Zhean Robby and Ang, Clarence Ivan and Campo, Roan Cedric and Ching, Justin Ethan and Chua, Erin Gabrielle and Jimenez, Jaztin Jacob and Ang, Clive Jarel and Oco, Nathaniel},
    license = {Apache-2.0},
    title = {{TagaBaybay}},
    url = {https://github.com/Mango-Cats/tagabaybay/}
}
```

Also see [`CITATION.cff`](/CITATION.cff).

## Bibliography

1. Komisyon sa Wikang Filipino ed. 2015. KWF manwal sa masinop na pagsulat. Komisyon sa Wikang Filipino.
1. Christian Uffmann 2015. Loanword Adaptation. The Oxford Handbook of Historical Phonology. Patrick Honeybone and Joseph Salmons, eds. Oxford University Press. 644–666.
1. Lingshuang Mao and Mans Hulden How Regular is Japanese Loanword Adaptation? A Computational Study.
