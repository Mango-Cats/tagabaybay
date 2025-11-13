# TagaBaybay

Nativization for Filipino spelling written in Rust.

| Author          | GitHub                                         |               |
|-----------------|-----------------------------------------------|--------------------|
| Zhean Ganituen | [@zrygan](https://github.com/zrygan)           | *Correspondence*   |
| Nathaniel Oco |                                               | Faculty Adviser   |

## The Name

The name is a word play on the Tagalog prefix "taga-" which denotes the doer of an action, "Taga" as in **Taga**log, and "baybay" which is the root word of the Tagalog word _baybayin_ which means to spell.

## Project Description
TagaBaybay ($\textbf{baybay}$) is a nativization algorithm that converts loan words or non-Filipino words $\omega$ to their corresponding nativized loan word $w$.

Examples:
- $\omega := \text{cellphone}; \qquad \textbf{baybay}(\omega) = \text{selpon}$.
- $\omega := \text{paracetamol}; \qquad \textbf{baybay}(\omega) = \text{parasetamol}$.

To limit the scope, we will define loan words as words that belong to the English language $\mathcal{E}$ or Spanish language $\mathcal{S}$.

Formally:
$$\text{baybay} : \mathcal{E}\cup\mathcal{S} \mapsto \Sigma_\mathcal{F}^*$$

where $\mathcal{F}$ is the Filipino language, and $\Sigma_\mathcal{F}$ denotes the alphabet of the Filipino language.

Here are literature for orthographic rules in Tagalog. The [Manwal sa Masinop na Pagsulat](https://kwf.gov.ph/wp-content/uploads/MMP_Full.pdf) of the Komisyon sa Wikang Filipino is a general reference, [[OSB+24]](https://aclanthology.org/2024.paclic-1.141/) discuss spelling variants and identification, [[Bak17]](https://archium.ateneo.edu/kk/vol1/iss28/4/) discuss types of borrowings in Filipino, [[CAC+07]](https://ejournals.ph/article.php?id=2172) discuss spelling errors.

The paper by \[CAC+07], recommends:
> instead of using the n-gram model, the syllabication nature of Filipino may be exploited to improve performance of the corrector module...

The rules of syllabication is available in the Manwal sa Masinop na Pagsulat and we may extract all valid syllabications by performing text extraction algorithms on pre-existing Tagalog or Filipino corpora.
