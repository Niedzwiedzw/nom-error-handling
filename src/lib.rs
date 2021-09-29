use nom::bytes::complete::tag;
use nom::character::complete::digit1;

use nom::*;
use nom::combinator::map_res;
use nom::error::*;
pub type Res<T, U> = IResult<T, U, VerboseError<T>>;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyCustomError {
    #[error("Failed to parse stuff :: {0}")]
    ParserError(String),
}

#[macro_export]
macro_rules! custom_err {
    ($input:expr, $msg:expr) => {{
        let e = nom::error::ErrorKind::Satisfy;
        Err(nom::Err::Error(VerboseError::from_external_error(
            $input,
            e,
            MyCustomError::ParserError(format!("[{}:{}] {}", file!(), line!(), $msg)),
        )))
    }};
}

#[macro_export]
macro_rules! validate {
    ($cond:expr, $input:expr) => {{
        let cond = stringify!($cond);
        if !$cond {
            return custom_err!($input, format!("assertion failed :: {}", cond));
        }
    }};
}



pub fn two_numbers(input: &str) -> Res<&str, i32> {
    let (
        input,
        (
            one,
            other,
        ),
    ): (_, (i32, i32)) = separated_pair(
        preceded(tag("one: "), map_res(digit1, |v: &str| v.parse())),
        tag("\n"),
        preceded(tag("other: "), map_res(digit1, |v: &str| v.parse())),
        )(input)?;
    validate!(one == other, input);
    Ok((
        input,
        other,
    ))
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        two_numbers(r#"one: 123
other: 321"#).unwrap();
    }
}
