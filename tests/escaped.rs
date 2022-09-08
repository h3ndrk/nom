use nom::{
  branch::alt,
  bytes::complete::{escaped, escaped_transform, tag, take_while},
  character::complete::{digit1, one_of},
  combinator::value,
  error::ErrorKind,
  sequence::delimited,
  Err, IResult,
};

fn esc(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
  escaped(digit1, '\\', one_of("\"n\\"))(s)
}

#[cfg(feature = "alloc")]
fn esc_trans(s: &str) -> IResult<&str, String, (&str, ErrorKind)> {
  escaped_transform(digit1, '\\', tag("n"))(s)
}

#[test]
fn test_escaped() {
  assert_eq!(esc("abcd"), Err(Err::Error(("abcd", ErrorKind::Escaped))));
}

#[test]
#[cfg(feature = "alloc")]
fn test_escaped_transform() {
  use nom::error::Error;

  assert_eq!(
    esc_trans("abcd"),
    Err(Err::Error(("abcd", ErrorKind::EscapedTransform)))
  );
  assert_eq!(
    delimited::<_, _, _, _, Error<&str>, _, _, _>(
      tag("\""),
      escaped_transform(
        take_while(|character: char| character.is_alphanumeric()),
        '\\',
        alt((value("\\", tag("\\")), value("\"", tag("\""))))
      ),
      tag("\"")
    )("\"foo\""),
    Ok(("", "foo".to_string()))
  );
}
