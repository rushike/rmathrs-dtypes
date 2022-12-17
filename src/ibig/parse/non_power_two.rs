//! Parse in a non-power-of-two radix.

use std::time::Instant;

use crate::ibig::{
    arch::word::Word,
    buffer::{Buffer, self},
    error::ParseError,
    mul,
    radix::{self, Digit},
    ubig::UBig, parse::parsebytes::{parse_4_byte, parse_8_bytes, parse_16_bytes, parse_8_bytes_flex},
};
use alloc::vec;

/// Parse in chunks of CHUNK_LEN * digits_per_word.
const CHUNK_LEN: usize = 256;

/// Parse an unsigned string to [UBig].
pub(crate) fn parse(src: &str, radix: Digit) -> Result<UBig, ParseError> {
    debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
    let radix_info = radix::radix_info(radix);
    let bytes = src.as_bytes();

    if bytes.len() <= radix_info.digits_per_word {
        let word = parse_word(bytes, radix)?; // ti took 1 - 2 micro second
        let res = UBig::from_word(word); // it takes < 1 micro sec
        Ok(res)
    } else if bytes.len() <= CHUNK_LEN * radix_info.digits_per_word {
        parse_chunk(bytes, radix)
    } else {
        parse_large(bytes, radix)
    }
}

pub(crate) fn parse2(bytes: &[u8], radix: Digit) -> Result<UBig, ParseError> {
    debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
    let radix_info = radix::radix_info(radix);

    if bytes.len() <= radix_info.digits_per_word {
        let word = parse_word(bytes, radix)?; // ti took 1 - 2 micro second
        let res = UBig::from_word(word); // it takes < 1 micro sec
        Ok(res)
    } else if bytes.len() <= CHUNK_LEN * radix_info.digits_per_word {
        parse_chunk(bytes, radix)
    } else {
        parse_large(bytes, radix)
    }
}
pub fn parse3(bytes: &[u8], radix: Digit) -> Result<UBig, ParseError> {
    debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
    let radix_info = radix::radix_info(radix);
    let size = bytes.len();
    let group_size = 8;
    let group_size_half = group_size >> 1;
    let groups = bytes.rchunks(group_size);
    // let mut itrm_buffer = Buffer::allocate(groups.len());
    let mut buffer = Buffer::allocate(groups.len());
    for group in groups.rev() {
        // println!("group : {group:?}");
        let localnum = if group.len() < group_size_half {
            parse_8_bytes_flex(group)
        } else {
            parse_8_bytes(group)
        };

        // itrm_buffer.push(localnum);

        // println!("local num : {localnum:?}");

        let carry = mul::mul_word_in_place_with_carry(&mut buffer, 100000000, localnum as Word);
        if carry != 0 {
            buffer.push(carry);
        }
    }
    // println!("itrm buffer : {:?}", itrm_buffer);
    // println!("radixx info : {:?}", radix_info.range_per_word);
    // let mut buffer = Buffer::allocate(itrm_buffer.len());
    // itrm_buffer.reverse();
    // for lnum in itrm_buffer.iter() {
    //     let carry = mul::mul_word_in_place_with_carry(&mut buffer, 100000000, *lnum);
    //     if carry != 0 {
    //         buffer.push(carry);
    //     }
    // }

    // println!("buffer : {:?}", buffer);
    // println!("parse chunk res : {:?}", parse_chunk(bytes, radix));
    return Ok(buffer.into());
}


/// Parse an unsigned string to `Word`.
///
/// The length of the string must be at most `digits_per_word`.
fn parse_word(src: &[u8], radix: Digit) -> Result<Word, ParseError> {
    // debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
    debug_assert!(src.len() <= radix::radix_info_internal(radix).digits_per_word);
    let radixword = radix as Word;
    let mut word: Word = 0;
    for byte in src.iter() {
        let digit = radix::digit_from_utf8_byte(*byte, radix).ok_or(ParseError::InvalidDigit)?;
        word = word * radixword + (digit as Word);
    }
    Ok(word) // it takes 1 - 2 micro seconds
}

/// Parse an unsigned string to [UBig].
///
/// The length of input is limited to `CHUNK_LEN * digits_per_word`.
fn parse_chunk(bytes: &[u8], radix: Digit) -> Result<UBig, ParseError> {
    debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
    let radix_info = radix::radix_info(radix);
    debug_assert!(bytes.len() <= CHUNK_LEN * radix_info.digits_per_word);
    // println!("bytes : {bytes:?}");
    let groups = bytes.rchunks(radix_info.digits_per_word);
    let mut buffer = Buffer::allocate(groups.len());
    // println!("parse chunk groups : {groups:?}");
    for group in groups.rev() {
        // println!("group : parse chunk : {group:?}");
        let next = parse_word(group, radix)?;
        // println!("next : {next:?}");
        let carry = mul::mul_word_in_place_with_carry(&mut buffer, radix_info.range_per_word, next);
        if carry != 0 {
            buffer.push(carry);
        }
    }

    // println!("parse chunk buffer : {buffer:?}");
    Ok(buffer.into())
}

/// Parse an unsigned string to [UBig].
///
/// This result will usually not fit in CHUNK_LEN words.
fn parse_large(bytes: &[u8], radix: Digit) -> Result<UBig, ParseError> {
    debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
    let radix_info = radix::radix_info(radix);
    let chunk_bytes = CHUNK_LEN * radix_info.digits_per_word;
    assert!(bytes.len() > chunk_bytes);

    // Calculate radix^(CHUNK_LEN<<i).
    let mut radix_powers = vec![UBig::from_word(radix_info.range_per_word).pow(CHUNK_LEN)];

    // while (chunk_bytes << radix_powers.len()) < bytes.len()
    // To avoid overflow:
    while chunk_bytes <= (bytes.len() - 1) >> radix_powers.len() {
        let prev = radix_powers.last().unwrap();
        let new = prev * prev;
        radix_powers.push(new);
    }

    parse_large_divide_conquer(bytes, radix, chunk_bytes, &radix_powers)
}

/// Convert an unsigned string to [UBig].
///
/// `radix_powers` contains radix^n for n = chunk digits << i
fn parse_large_divide_conquer(
    bytes: &[u8],
    radix: Digit,
    chunk_bytes: usize,
    radix_powers: &[UBig],
) -> Result<UBig, ParseError> {
    debug_assert!(bytes.len() <= chunk_bytes << radix_powers.len());

    match radix_powers.split_last() {
        None => parse_chunk(bytes, radix),
        Some((radix_power, radix_powers)) => {
            let bytes_lo_len = chunk_bytes << radix_powers.len();
            if bytes.len() <= bytes_lo_len {
                parse_large_divide_conquer(bytes, radix, chunk_bytes, radix_powers)
            } else {
                let (bytes_hi, bytes_lo) = bytes.split_at(bytes.len() - bytes_lo_len);
                let res_hi =
                    parse_large_divide_conquer(bytes_hi, radix, chunk_bytes, radix_powers)?;
                let res_lo =
                    parse_large_divide_conquer(bytes_lo, radix, chunk_bytes, radix_powers)?;
                Ok(res_hi * radix_power + res_lo)
            }
        }
    }
}
