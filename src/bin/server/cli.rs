use clap::{App, Arg, crate_authors, crate_description, crate_name, crate_version};

use pixie::common::ApplicationError;

use crate::config::{ApplicationConfig, ApplicationConfigBuilder};

pub fn parse() -> Result<ApplicationConfig, ApplicationError> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("width")
                .about("display width")
                .default_value("960")
                .validator(is_positive_number)
                .short('w'),
        )
        .arg(
            Arg::new("height")
                .default_value("240")
                .about("display height")
                .validator(is_positive_number)
                .short('h'),
        )
        .arg(
            Arg::new("server")
                .default_value("127.0.0.1:6789")
                .about("server address")
                .short('s'),
        )
        .get_matches();

    let mut builder = ApplicationConfigBuilder::new();

    builder.width(matches.value_of_t::<u32>("width")?);
    builder.height(matches.value_of_t::<u32>("height")?);
    builder.address(matches.value_of_t::<String>("server")?);
    Ok(builder.build())
}


pub fn is_positive_number(value: &str) -> Result<(), ApplicationError> {
    match value.parse::<u64>() {
        Ok(number) => {
            if number == 0 {
                return Err(ApplicationError::Configuration("Must be greater than zero".to_string()));
            }
        }
        Err(_) => {
            return Err(ApplicationError::Configuration("Must be greater than zero".to_string()));
        }
    }
    Ok(())
}