//! adopted from https://fuku.day/blog/2022-03-15-strtol-rust/
//! SPDX-License-Identifier: CC0-1.0

use std::str::FromStr;

pub(super) fn str_to_fromstr<F: FromStr>(str: &str) -> Result<(F, &str), F::Err> {
    let iter = str.bytes().enumerate();

    let mut index = str.len();

    for (i, byte) in iter {
        if byte.is_ascii_digit() {
            continue;
        }

        index = i;
        break;
    }

    let digit_part = &str[..index];

    digit_part.parse().map(|value| (value, &str[index..]))
}

#[cfg(test)]
mod tests {
    use crate::strtol::str_to_fromstr;

    #[test]
    fn strto_test() {
        assert_eq!(str_to_fromstr("123"), Ok((123, "")));
        assert_eq!(str_to_fromstr("123あいう"), Ok((123, "あいう")));
        assert!(str_to_fromstr::<i32>("あbc").is_err());
        assert!(str_to_fromstr::<i32>("").is_err());
    }
}
