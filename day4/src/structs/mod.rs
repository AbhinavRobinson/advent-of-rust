#[derive(Clone, Copy, PartialEq, Debug)]
struct Year(u64);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Length {
    Cm(u64),
    In(u64),
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Color<'a>(&'a str);

#[derive(Clone, Copy, PartialEq, Debug)]
struct ID<'a>(&'a str);

#[derive(PartialEq, Debug)]
struct Passport<'a> {
    birth_year: Year,
    issue_year: Year,
    expiration_year: Year,
    height: Length,
    hair_color: Color<'a>,
    eye_color: Color<'a>,
    passport_id: ID<'a>,
    country_id: Option<ID<'a>>,
}

#[derive(PartialEq, Debug, Default)]
struct PassportBuilder<'a> {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Length>,
    hair_color: Option<Color<'a>>,
    eye_color: Option<Color<'a>>,
    passport_id: Option<ID<'a>>,
    country_id: Option<ID<'a>>,
}
