# ᜆᜄᜊᜌ᜔ᜊᜌ᜔ / TagaBaybay: Algorithm

## Tokenization
The tokenization stage is a preprocessing step in the loanword adaptation algorithm. Its purpose is to convert an input string from the source language into a sequence of orthographic units (graphemes) that the subsequent rewrite rules can process. This allows the algorithm to handle digraphs, uppercase letters, and special characters in a structured and deterministic manner.

Sections:
1. [Algorithm](#algorithm)
2. [Grapheme Type](#graphemes-type)

---

### Algorithm

The tokenization function can be formalized as a mapping from strings over the source alphabet Σ to sequences of graphemes:

$$
\text{tokenize} : \Sigma^* \to \texttt{SourceGrapheme}^*
$$

- Input: a string s in Σ*.  
- Output: a sequence of graphemes.  

The function operates greedily from left to right, attempting to match the longest possible substring first. In the current implementation, the maximal substring length is two, corresponding to digraphs. If no digraph match is found, the algorithm falls back to mapping a single character to its corresponding grapheme.

 

```text
ALGORITHM tokenize(s: String) -> Array<SourceGrapheme>
    result <- empty array
    i <- 0

    while i < length(s):
        # Attempt digraph match (2 characters)
        if i + 2 <= length(s):
            digraph = s[i..i+2]
            match = match_digraph(digraph)  

            if match is Some(token):
                append result with token
                i <- i + 2
                continue

        # Fall back to monograph
        single = SourceGrapheme.from_char(s[i])
        append result with single
        i <- i + 1

    return result
```


The function calls two internal functions. The first, $\text{match\_digraph}$, is a partial function:

$$
\text{match\_digraph} : \{ s' \in \Sigma^* \mid |s'| = 2 \} \to \text{Option}(\texttt{SourceGrapheme})
$$

which returns $\text{Some}(g)$ if a digraph is recognized, and None otherwise. This implements a form of pattern recognition over substrings, allowing the algorithm to detect multi-character graphemes.

The second function, $\text{from\_char}$, maps individual characters to graphemes:

$$
\text{from\_char} : \Sigma \to \texttt{SourceGrapheme}
$$

This is a total and injective mapping:

$$
\forall c \in \Sigma, \exists! g \in \texttt{SourceGrapheme} \text{ such that } \text{from\_char}(c) = g
$$

forming a bijection between the subset of graphemes that represent single characters and the input alphabet. Its inverse is

$$
\text{to\_string} : \texttt{SourceGrapheme} \to \Sigma
$$

with the property

$$
\text{to\_string}(\text{from\_char}(c)) = c \quad \forall c \in \Sigma
$$

ensuring that tokenization and detokenization are lossless.

## Graphemes Type

The $\texttt{SourceGrapheme}$ datatype is a sum type (algebraic data type) representing all graphemes recognized by the tokenization algorithm. Each variant corresponds to a distinct unit of orthography in the source languages. Variants include:

- Single letters (vowels and consonants), both lowercase and uppercase.  
- Multigraphs (two-letter patterns such as digraphs).  
- Special constructors for whitespace, known non-alphanumeric characters, and unknown or non-ASCII characters.

Formally, the set of graphemes is a disjoint union:

$$
\texttt{SourceGrapheme} = 
    \texttt{A} \mid \texttt{B} \mid \dots \mid \texttt{Z} \mid
    \texttt{UpperA} \mid \dots \mid \texttt{UpperZ} \mid
    \texttt{PH} \mid \dots \mid \texttt{OO} \mid
    \texttt{Space} \mid \texttt{Passthrough(String)} \mid \texttt{Other}
$$

Each variant implements core traits or predicates that allow the algorithm to classify graphemes into semantic categories:

- $\text{is\_vowel}$: indicates whether a grapheme represents a vowel sound.  
- $\text{is\_consonant}$: indicates whether a grapheme represents a consonant sound.  
- $\text{is\_digraph}$: indicates whether a grapheme represents a multi-character orthographic pattern.  
- $\text{as\_str}$: converts a grapheme back to its canonical string representation.

These predicates allow subsequent algorithms to reason about phonological and orthographic properties instead of operating on raw characters. For example, $\text{is\_vowel}$ partitions the sum type into the set of vowel graphemes, enabling adaptation rules to target vowel sequences directly.

The sum type structure also distinguishes between plain constants $(\texttt{PH}, \texttt{A}, \texttt{B}, \dots)$ and parametric variants ($\texttt{Passthrough}(\text{String})$), allowing the algorithm to store additional information when necessary without breaking the type system.