use nom::{
    bytes::complete::tag,
    character::complete::*,
    combinator::{all_consuming, map, recognize},
    multi::*,
    sequence::*,
    IResult,
};

// mxmxvkd kfcds sqjhc nhms (contains dairy, fish)

// mxmxvkd kfcds sqjhc nhms
fn parse_ingredients(i: &str) -> IResult<&str, Vec<&str>> {
    many1(delimited(space0, alpha1, space0))(i)
}

fn delim(i: &str) -> IResult<&str, &str> {
    recognize(many0(one_of(", ")))(i)
}

// dairy, fish
fn parse_allergens(i: &str) -> IResult<&str, Vec<&str>> {
    many1(delimited(delim, alpha1, delim))(i)
}

pub fn parse_food(i: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    map(
        all_consuming(tuple((
            parse_ingredients,
            tag("(contains"),
            parse_allergens,
            tag(")"),
        ))),
        |(ingred, _, allerg, _)| (ingred, allerg),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingredients_correct() {
        assert_eq!(
            parse_ingredients("mxmxvkd kfcds sqjhc nhms"),
            Ok(("", vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"]))
        );
    }

    #[test]
    fn allergens_correc() {
        assert_eq!(
            parse_allergens("dairy, fish"),
            Ok(("", vec!["dairy", "fish"]))
        );
    }

    #[test]
    fn food_correct() {
        assert_eq!(
            parse_food("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            Ok((
                "",
                (
                    vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                    vec!["dairy", "fish"]
                )
            ))
        );
    }
}
