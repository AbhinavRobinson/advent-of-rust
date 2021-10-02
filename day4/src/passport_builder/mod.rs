use crate::structs::{Color, Error, Length, Passport, PassportBuilder, Year, ID};

impl<'a> PassportBuilder<'a> {
    pub fn build(self) -> Result<Passport<'a>, Error> {
        macro_rules! build {
        (
          required => {
              $($req: ident),* $(,)*
          }$(,)*
          optional => {
              $($opt: ident),* $(,)*
          }$(,)*
      ) => {
          Ok(Passport {
              $($req: self.$req.ok_or(Error::MissingField(stringify!($req)))?),*,
              $($opt: self.$opt),*
          })
      }
      }
        build! {
            required => {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
            },
            optional => {
                country_id,
            },
        }
    }
    pub fn parse(input: &'a str) -> Self {
        let mut b: Self = Default::default();

        peg::parser! {
            grammar parser() for str {

                pub(crate) rule root(b: &mut PassportBuilder<'input>)
                    = (field(b) separator()*)* ![_]

                rule separator()
                    = ['\n' | ' ']

                rule field(b: &mut PassportBuilder<'input>)
                    // years
                    = byr(b) / iyr(b) / eyr(b)
                    // height
                    / hgt(b)
                    // colors
                    / hcl(b) / ecl(b)
                    // IDs
                    / pid(b) / cid(b)

                rule byr(b: &mut PassportBuilder<'input>)
                    = "byr:" year:year() { b.birth_year = Some(year) }

                rule iyr(b: &mut PassportBuilder<'input>)
                    = "iyr:" year:year() { b.issue_year = Some(year) }

                rule eyr(b: &mut PassportBuilder<'input>)
                    = "eyr:" year:year() { b.expiration_year = Some(year) }

                rule hgt(b: &mut PassportBuilder<'input>)
                    = "hgt:" height:length() { b.height = Some(height) }

                rule pid(b: &mut PassportBuilder<'input>)
                    = "pid:" id:id() { b.passport_id = Some(id) }

                rule cid(b: &mut PassportBuilder<'input>)
                    = "cid:" id:id() { b.country_id = Some(id) }

                rule hcl(b: &mut PassportBuilder<'input>)
                    = "hcl:" color:color() { b.hair_color = Some(color) }

                rule ecl(b: &mut PassportBuilder<'input>)
                    = "ecl:" color:color() { b.eye_color = Some(color) }

                rule year() -> Year
                    = num:num() { Year(num) }

                rule color() -> Color<'input>
                    = s:$((!separator()[_])*) { Color(s) }

                rule length() -> Length
                    = num:num() "cm" { Length::Cm(num) }
                    / num:num() "in" { Length::In(num) }
                    / num:num() { Length::Unspecified(num) }

                rule num() -> u64
                    = s:$(['0'..='9']+) { s.parse().unwrap() }

                rule id() -> ID<'input>
                    = s:$(['0'..='9' | 'a'..='z' | '#']+) { ID(s) }
            }
        }

        parser::root(input, &mut b).unwrap_or_else(|e| panic!("Could not parse {}: {}", input, e));
        b
    }
}

#[test]
fn test_builder() {
    assert!(PassportBuilder {
        ..Default::default()
    }
    .build()
    .is_err());
    assert!(PassportBuilder {
        birth_year: Some(Year(2014)),
        issue_year: Some(Year(2017)),
        expiration_year: Some(Year(2023)),
        height: Some(Length::Cm(195)),
        hair_color: Some(Color("#ffffff")),
        eye_color: Some(Color("#ee7812")),
        passport_id: Some(ID("00023437")),
        country_id: None,
    }
    .build()
    .is_ok());
}
