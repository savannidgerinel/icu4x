use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};
use nom::{
    branch::alt, bytes::complete::{tag, take_till, take_until}, character::complete::char, combinator::eof, multi::many0, sequence::delimited, Finish, IResult, Parser
};

// https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/text/MessageFormat.html
//
// MessageFormat uses patterns of the following form:
//
//      message = messageText (argument messageText)*
//      argument = noneArg | simpleArg | complexArg
//      complexArg = choiceArg | pluralArg | selectArg | selectordinalArg
//
//      noneArg = '{' argNameOrNumber '}'
//      simpleArg = '{' argNameOrNumber ',' argType [',' argStyle] '}'
//      choiceArg = '{' argNameOrNumber ',' "choice" ',' choiceStyle '}'
//      pluralArg = '{' argNameOrNumber ',' "plural" ',' pluralStyle '}'
//      selectArg = '{' argNameOrNumber ',' "select" ',' selectStyle '}'
//      selectordinalArg = '{' argNameOrNumber ',' "selectordinal" ',' pluralStyle '}'
//
//      choiceStyle: see ChoiceFormat
//        [https://docs.oracle.com/javase/8/docs/api/java/text/ChoiceFormat.html?is-external=true]
//      pluralStyle: see PluralFormat
//        [https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/text/PluralFormat.html]
//      selectStyle: see SelectFormat
//        [https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/text/SelectFormat.html]
//
//      argNameOrNumber = argName | argNumber
//      argName = [^[[:Pattern_Syntax:][:Pattern_White_Space:]]]+
//      argNumber = '0' | ('1'..'9' ('0'..'9')*)
//
//      argType = "number" | "date" | "time" | "spellout" | "ordinal" | "duration"
//      argStyle = "short" | "medium" | "long" | "full" | "integer" | "currency" | "percent" | argStyleText | "::" argSkeletonText

/// This data structure stores a parsed AST for a single message and is able to render that message
/// into a user-visible string.
#[derive(Debug)]
pub struct Message {
    parts: Vec<MessagePart>,
}

impl Message {
    pub fn render(&self) -> String {
        self.parts
            .as_slice()
            .iter()
            .map(|part| part.render())
            .collect::<String>()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseMessageError {
    IncompletelyParsed,
    MissingDelimiter,

    Unknown(nom::error::ErrorKind),
}

impl From<nom::error::ErrorKind> for ParseMessageError {
    fn from(err: nom::error::ErrorKind) -> Self {
        ParseMessageError::Unknown(err)
    }
}

impl TryFrom<&str> for Message {
    type Error = ParseMessageError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match parse_message(input).finish() {
            Ok((remainder, message)) => {
                if remainder.is_empty() {
                    Ok(message)
                } else {
                    Err(ParseMessageError::IncompletelyParsed)
                }
            }
            Err(err) => match err.code {
                nom::error::ErrorKind::Tag => Err(ParseMessageError::MissingDelimiter),
                _ => Err(ParseMessageError::Unknown(err.code)),
            }
        }
    }
}

#[derive(Debug)]
enum MessagePart {
    MessageText(String),
    Argument(Argument),
}

impl MessagePart {
    fn render(&self) -> String {
        use MessagePart::*;

        match self {
            MessageText(text) => text.clone(),
            Argument(_) => unimplemented!(),
        }
    }
}

#[derive(Debug)]
enum Argument {
    NoneArg(String),
    SimpleArg,
    ComplexArg(ComplexArg),
}

#[derive(Debug)]
enum ComplexArg {
    ChoiceArg,
    PluralArg,
    SelectArg,
    SelectOrdinalArg,
}

fn parse_message(input: &str) -> IResult<&str, Message> {
    let (input, part1) = take_till(|c| c == '{')(input)?;
    let (input, rest) = many0(parse_message_part)(input)?;

    let mut rest = rest.into_iter().flatten().collect::<Vec<MessagePart>>();
    let mut result = vec![MessagePart::MessageText(part1.to_owned())];
    result.append(&mut rest);

    Ok((
        input,
        Message {
            parts: result,
        },
    ))
}

fn parse_message_part(input: &str) -> IResult<&str, Vec<MessagePart>> {
    let (input, argument) = delimited(char('{'), take_until("}"), char('}'))(input)?;
    let (input, text) = take_till(|c| c == '{')(input)?;

    let mut result = vec![MessagePart::Argument(Argument::NoneArg(argument.to_owned()))];
    if !text.is_empty() { result.push(MessagePart::MessageText(text.to_owned())) };

    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    // "{gender_of_host, select, "
    //   "female {"
    //     "{num_guests, plural, offset:1 "
    //       "=0 {{host} does not give a party.}"
    //       "=1 {{host} invites {guest} to her party.}"
    //       "=2 {{host} invites {guest} and one other person to her party.}"
    //       "other {{host} invites {guest} and # other people to her party.}}}"
    //   "male {"
    //     "{num_guests, plural, offset:1 "
    //       "=0 {{host} does not give a party.}"
    //       "=1 {{host} invites {guest} to his party.}"
    //       "=2 {{host} invites {guest} and one other person to his party.}"
    //       "other {{host} invites {guest} and # other people to his party.}}}"
    //   "other {"
    //     "{num_guests, plural, offset:1 "
    //       "=0 {{host} does not give a party.}"
    //       "=1 {{host} invites {guest} to their party.}"
    //       "=2 {{host} invites {guest} and one other person to their party.}"
    //       "other {{host} invites {guest} and # other people to their party.}}}}"

    // "At {1,time,::jmm} on {1,date,::dMMMM}, there was {2} on planet{0,number,integer}."

    #[test]
    fn it_parses_a_simple_message() {
        let message =
            Message::try_from("Welcome to ICU4X MessageFormat").expect("message to parse");
        assert_eq!(message.render(), "Welcome to ICU4X MessageFormat");
    }

    #[test]
    fn it_parses_messages_with_basic_placeholders() {
        let message = Message::try_from("At {1} on {1}, there was {2} on planet {0}.")
            .expect("message to parse");

        println!("{:?}", message);

        assert_eq!(
            message.render(),
            "At 11:05 on June 5th, there was a great celebration on planet Mars"
        );
    }
}
