#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Year(pub(crate) u64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Length {
    Cm(u64),
    In(u64),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color<'a>(pub(crate) &'a str);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ID<'a>(pub(crate) &'a str);

#[derive(PartialEq, Debug)]
pub(crate) struct Passport<'a> {
    pub birth_year: Year,
    pub issue_year: Year,
    pub expiration_year: Year,
    pub height: Length,
    pub hair_color: Color<'a>,
    pub eye_color: Color<'a>,
    pub passport_id: ID<'a>,
    pub country_id: Option<ID<'a>>,
}

#[derive(PartialEq, Debug, Default)]
pub(crate) struct PassportBuilder<'a> {
    pub birth_year: Option<Year>,
    pub issue_year: Option<Year>,
    pub expiration_year: Option<Year>,
    pub height: Option<Length>,
    pub hair_color: Option<Color<'a>>,
    pub eye_color: Option<Color<'a>>,
    pub passport_id: Option<ID<'a>>,
    pub country_id: Option<ID<'a>>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
}
