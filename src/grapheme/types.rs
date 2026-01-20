use super::filipino::FilipinoGrapheme;
use super::source::SourceGrapheme;

pub enum GraphemesSet {
    Src(SourceGrapheme),
    Fil(FilipinoGrapheme),
}
