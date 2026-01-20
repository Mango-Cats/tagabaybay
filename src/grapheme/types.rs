use super::source::SourceGrapheme;
use super::filipino::FilipinoGrapheme;

pub enum GraphemesSet {
    Src(SourceGrapheme),
    Fil(FilipinoGrapheme),
}