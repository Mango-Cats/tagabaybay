pub const PATINIG: (
    Alphabet,
    Alphabet,
    Alphabet,
    Alphabet,
    Alphabet,
) = (
    Alphabet::LetA,
    Alphabet::LetE,
    Alphabet::LetI,
    Alphabet::LetO,
    Alphabet::LetU,
);

pub enum Alphabet {
    // All Latin graphemes
    LetA,
    LetB,
    LetC,
    LetD,
    LetE,
    LetF,
    LetG,
    LetH,
    LetI,
    LetJ,
    LetK,
    LetL,
    LetM,
    LetN,
    LetO,
    LetP,
    LetQ,
    LetR,
    LetS,
    LetT,
    LetU,
    LetV,
    LetW,
    LetX,
    LetY,
    LetZ,

    // Additional letters
    TagNg,
    TagEnye,

    // Non-letter characters
    SymHyphen,
}

pub enum TagalogDigraph {
    TagTs,
    TagLy,
    TagNy,
    TagKs,
}
