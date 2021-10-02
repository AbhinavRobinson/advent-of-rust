use crate::structs::{Color, Error, Length, Passport, PassportBuilder, Year, ID};

impl<'a> PassportBuilder<'a> {
    fn build(self) -> Result<Passport<'a>, Error> {
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
