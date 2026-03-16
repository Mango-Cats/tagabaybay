# TagaBaybay: Gold Standards

This directory contains gold standards for validating the accuracy of
TagaBaybay. The directory contains gold standards ranging from general
English words to specific domains where phonetic adaptation commonly
occurs.

## Construction

These gold standards were manually generated (excluding `wiki`) by Native
Filipino speakers using the Komisyon sa Wikang Filipino's Manwal sa
Masinop na Pagusulat `[1]` as reference.

## Gold Standards

We enumerate below the gold data contained in this directory, as well
as their specific domain and author.

| Filename                | Domain                 | Author(s)                  |
| ---                     | ---                    | ---                        |
| `common_drugs`          | Pharmaceutical         | Z. Ganituen & C. Ang       |
| `ph_fda_human`          | Pharmaceutical         | Z. Ganituen                |
| `common_eng`            | General                | Z. Ganituen                |
| `ching_chua`            | General                | J. Ching & E. Chua         |
| `wiki`                  | General                | Wikipedia contributors     |

## Validation

Validation of the gold standard is done via inter-annotator agreement. A Python
script gets a random sample of a selected gold standard to be validated, the
size of this random sample is 60% the original size of the gold standard.

Simply run

```
python3 validation.py
```

## Bibliography

`[1]` Virgilio S. Almario (Ed.). 2014. KWF Manwal sa Masinop na Pagsulat. Komisyon sa Wikang Filipino, Quezon City.
