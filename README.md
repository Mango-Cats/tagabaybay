# ᜆᜄᜊᜌ᜔ᜊᜌ᜔ / TagaBaybay

**Question**: _How do we orthographically adapt frequently used loanwords in Filipino?_

**Abstract**: _Orthographic nativization_, generating standardized written forms of loanwords in a recipient language, remains understudied computationally despite its importance for text normalization and multilingual NLP. We present a rule-based rewrite cascade for orthographic nativization of English loanwords into Filipino. The system operates through four stages: graphemic tokenization, priority-ordered rewrite rules, phonetic resolution via grapheme-to-phoneme lookup, and output normalization. Rules are hand-authored from prescriptive orthographic guidelines and descriptive phonological sources, enabling deployment without parallel training corpora. We evaluate on a newly constructed gold standard of 2,319 English-Filipino loanword pairs. The proposed method achieves 5.14% character error rate under a 5-vowel evaluation and 4.05% under a 3-vowel evaluation that accounts for Filipino's vowel variation. The system outperforms a regular expressions, an orthographic-only ablation without phonetic resolution, and zero-shot prompting of a large language model given the same mapping rules. Error analysis reveals that most remaining errors stem from schwa ambiguity in unstressed syllables, glide insertion, and the quality of the phonetic transcription, suggesting directions for refinement. The linguistically grounded approach demonstrates that prescriptive rules can be operationalized into effective orthographic nativization systems for low-resource languages.
</details>

## Project Team

**Project Lead**: Zhean Robby Ganituen △◇ <br>
**Faculty Supervisor**: Nathaniel Oco △◇ <br>
**Project Members**:

- Erin Gabrielle Chua ◇
- Jaztin Jacob Jimenez ◇
- Justin Ethan Ching ◇
- Clarance Ivan Ang △
- Roan Cedric Campo △
- Clive Jarel Ang △

\***Correspondence:** `zr.gntn AT gmail DOT com` <br>
△ Part of the legacy project. <br>
◇ Part of the current project.

## License

TagaBaybay is distributed under the terms of Apache License Version 2.0.

See [LICENSE](LICENSE) for details.

## Citation

If you use this software in your research, consider citing our work (see [`CITATION.cff`](/CITATION.cff)).

**Cite as** Ganituen, Z. R., Chua, E. G., Ching, J. E., Jimenez, J. J., Ang, C. J., Ang, C. I., Campo, R. C., & Oco, N. TagaBaybay [Computer software]. https://github.com/Mango-Cats/tagabaybay/

BibTeX:
```bib
@software{tagabaybay,
  author = {Ganituen, Zhean Robby and Chua, Erin Gabrielle and Ching, Justin Ethan and Jimenez, Jaztin Jacob and Ang, Clive Jarel and Ang, Clarence Ivan and Campo, Roan Cedric and Oco, Nathaniel},
  license = {Apache-2.0},
  title = {{TagaBaybay}},
  url = {https://github.com/Mango-Cats/tagabaybay/}
}
```

## Bibliography

1. Ma. Lourdes S. Bautista. 2000. Defining Standard Philippine English: Its status and grammatical features. De La Salle University Press.
1. James Hillenbrand, Laura A. Getty, Michael J. Clark, and Kimberlee Wheeler. 1995. Acoustic characteristics of American English vowels. The Journal of the Acoustical Society of America 97, 5 (May 1995), 3099–3111. <https://doi.org/10.1121/1.411872>
1. Virgilio S. Almario (Ed.). 2014. KWF Manwal sa Masinop na Pagsulat. Komisyon sa Wikang Filipino, Quezon City.
1. Teodoro A. Llamzon. 1966. Tagalog Phonology. Anthropological Linguistics 8, 1 (January 1966).
1. Teodoro A. Llamzon. 1997. The Phonology of Philippine English. In English is an Asian Language: The Philippine Context, Ma. Lourdes S. Bautista (Ed.). The Macquarie Library, 41–48.
1. Teodoro A. Llamzon. 1969. Standard Filipino English. Ateneo de Manila University Press, Quezon City.
1. Teri An Joy Magpale. 2025. Beyond the standard: A sociolectal and probabilistic turn in Philippine English phonology. Journal of English and Applied Linguistics 4, 2 (December 2025).
1. Teri An Joy Magpale. 2026. A probabilistic phonology of the Philippine English mesolect: an optimality-theoretic and MaxEnt account. Lingua 332, 104096 (March 2026), 104096.
1. Teri Magpale and Sung-Hoon Hong. 2024. MaxEnt modeling of sC clusters in Philippine English: Bridging phonotactic theory and sociolectal diversity. Studies in Phonetics, Phonology and Morphology 30, 2 (2024), 193–224. <https://doi.org/10.17959/sppm.2024.30.2.193>
1. Isabel Pefianco Martin. 2020. Philippine English. In The Handbook of Asian Englishes, Kingsley Bolton, Botha Werner, and Andy Kirkpatrick (Eds.). Wiley-Blackwell.
1. Eden Regala-Flores. 2014. Phonological features of Basilectal Philippine English: An exploratory study. International Journal of English and Literature 5, 6 (August 2014). <https://doi.org/10.5897/IJEL2014.0586>
1. Paul Schachter and Fe T. Otanes. 1972. Tagalog Reference Grammar. University of California Press, Berkeley, Los Angeles, London.
1. Ma. Lourdes G. Tayao. 2004. The evolving study of Philippine English phonology. World Englishes 23, 1 (February 2004). <https://doi.org/10.1111/j.1467-971X.2004.00336.x>
1. Lingshuang Mao and Mans Hulden. 2016. How Regular is Japanese Loanword Adaptation? A Computational Study. In Proceedings of COLING 2016, the 26th International Conference on Computational Linguistics: Technical Papers, Yuji Matsumoto and Rashmi Prasad (Eds.). The COLING 2016 Organizing Committee, Osaka, Japan, 847–856. <https://aclanthology.org/C16-1081/>
