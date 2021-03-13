//! Formatting numbers.

use crate::{
    arch::Word,
    div,
    div_ops::DivRem,
    ibig::IBig,
    math,
    primitive::{WORD_BITS, WORD_BITS_USIZE},
    radix::{self, Digit, DigitCase},
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use alloc::vec::Vec;
use core::{
    fmt::{self, Alignment, Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex, Write},
    mem,
};
use digit_writer::DigitWriter;

mod digit_writer;

/// Format non-power-of-2 radix in chunks of CHUNK_LEN * digits_per_word.
const CHUNK_LEN: usize = 16;

impl Display for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 10,
            prefix: "",
            digit_case: Some(DigitCase::NoLetters),
        }
        .fmt(f)
    }
}

impl Debug for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Binary for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 2,
            prefix: if f.alternate() { "0b" } else { "" },
            digit_case: Some(DigitCase::NoLetters),
        }
        .fmt(f)
    }
}

impl Octal for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 8,
            prefix: if f.alternate() { "0o" } else { "" },
            digit_case: Some(DigitCase::NoLetters),
        }
        .fmt(f)
    }
}

impl LowerHex for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl UpperHex for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: Positive,
            magnitude: self,
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Upper),
        }
        .fmt(f)
    }
}

impl Display for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 10,
            prefix: "",
            digit_case: Some(DigitCase::NoLetters),
        }
        .fmt(f)
    }
}

impl Debug for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Binary for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 2,
            prefix: if f.alternate() { "0b" } else { "" },
            digit_case: Some(DigitCase::NoLetters),
        }
        .fmt(f)
    }
}

impl Octal for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 8,
            prefix: if f.alternate() { "0o" } else { "" },
            digit_case: Some(DigitCase::NoLetters),
        }
        .fmt(f)
    }
}

impl LowerHex for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Lower),
        }
        .fmt(f)
    }
}

impl UpperHex for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: Some(DigitCase::Upper),
        }
        .fmt(f)
    }
}

impl UBig {
    /// Representation in a given radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(format!("{}", ubig!(83).in_radix(3)), "10002");
    /// assert_eq!(format!("{:+010}", ubig!(35).in_radix(36)), "+00000000z");
    /// ```
    pub fn in_radix(&self, radix: u32) -> InRadix {
        radix::check_radix_valid(radix);
        InRadix {
            sign: Positive,
            magnitude: self,
            radix,
            prefix: "",
            digit_case: None,
        }
    }
}

impl IBig {
    /// Representation in a given radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::prelude::*;
    /// assert_eq!(format!("{}", ibig!(-83).in_radix(3)), "-10002");
    /// assert_eq!(format!("{:010}", ibig!(-35).in_radix(36)), "-00000000z");
    /// ```
    pub fn in_radix(&self, radix: u32) -> InRadix {
        radix::check_radix_valid(radix);
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix,
            prefix: "",
            digit_case: None,
        }
    }
}

/// Representation of a `UBig` or `IBig` in any radix between 2 and 36 inclusive.
///
/// This can be used to format a number in a non-standard radix.
///
/// The default format uses lower-case letters a-z for digits 10-35.
/// The "alternative" format (`{:#}`) uses upper-case letters.
///
/// # Examples
///
/// ```
/// # use ibig::prelude::*;
/// assert_eq!(format!("{}", ubig!(83).in_radix(3)), "10002");
/// assert_eq!(format!("{:+010}", ubig!(35).in_radix(36)), "+00000000z");
/// // For bases 2, 8, 10, 16 we don't have to use `InRadix`:
/// assert_eq!(format!("{:x}", ubig!(3000)), "bb8");
/// assert_eq!(format!("{:x}", ibig!(-3000)), "-bb8");
/// ```
pub struct InRadix<'a> {
    sign: Sign,
    magnitude: &'a UBig,
    radix: Digit,
    prefix: &'static str,
    /// None means the case will be decide based on the "alternate" format.
    digit_case: Option<DigitCase>,
}

impl Display for InRadix<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let digit_case = self.digit_case.unwrap_or_else(|| {
            if self.radix <= 10 {
                DigitCase::NoLetters
            } else if f.alternate() {
                DigitCase::Upper
            } else {
                DigitCase::Lower
            }
        });

        if self.radix.is_power_of_two() {
            match self.magnitude.repr() {
                Small(word) => {
                    let mut prepared = PreparedWordInPow2::new(*word, self.radix);
                    self.format_prepared(f, digit_case, &mut prepared)
                }
                Large(buffer) => {
                    let mut prepared = PreparedLargeInPow2::new(buffer, self.radix);
                    self.format_prepared(f, digit_case, &mut prepared)
                }
            }
        } else {
            match self.magnitude.repr() {
                Small(word) => {
                    let mut prepared = PreparedWordInNonPow2::new(*word, self.radix, 1);
                    self.format_prepared(f, digit_case, &mut prepared)
                }
                Large(buffer) => {
                    let radix_info = radix::radix_info(self.radix);
                    let max_digits = buffer.len() * (radix_info.digits_per_word + 1);
                    if max_digits <= CHUNK_LEN * radix_info.digits_per_word {
                        let mut prepared = PreparedMediumInNonPow2::new(self.magnitude, self.radix);
                        self.format_prepared(f, digit_case, &mut prepared)
                    } else {
                        let mut prepared = PreparedLargeInNonPow2::new(self.magnitude, self.radix);
                        self.format_prepared(f, digit_case, &mut prepared)
                    }
                }
            }
        }
    }
}

impl InRadix<'_> {
    /// Format using a `PreparedForFormatting`.
    fn format_prepared(
        &self,
        f: &mut Formatter,
        digit_case: DigitCase,
        prepared: &mut dyn PreparedForFormatting,
    ) -> fmt::Result {
        let mut width = prepared.width();

        // Adding sign and prefix to width will not overflow, because Buffer::MAX_CAPACITY leaves
        // (WORD_BITS - 1) spare bits before we would hit overflow.
        let sign = if self.sign == Negative {
            "-"
        } else if f.sign_plus() {
            "+"
        } else {
            ""
        };
        // In bytes, but it's OK because everything is ASCII.
        width += sign.len() + self.prefix.len();

        let mut write_digits = |f| {
            let mut digit_writer = DigitWriter::new(f, digit_case);
            prepared.write(&mut digit_writer)?;
            digit_writer.flush()
        };

        match f.width() {
            None => {
                f.write_str(sign)?;
                f.write_str(self.prefix)?;
                write_digits(f)?
            }
            Some(min_width) => {
                if width >= min_width {
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    write_digits(f)?;
                } else if f.sign_aware_zero_pad() {
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    for _ in 0..min_width - width {
                        f.write_str("0")?;
                    }
                    write_digits(f)?;
                } else {
                    let left = match f.align() {
                        Some(Alignment::Left) => 0,
                        Some(Alignment::Right) | None => min_width - width,
                        Some(Alignment::Center) => (min_width - width) / 2,
                    };
                    let fill = f.fill();
                    for _ in 0..left {
                        f.write_char(fill)?;
                    }
                    f.write_str(sign)?;
                    f.write_str(self.prefix)?;
                    write_digits(f)?;
                    for _ in left..min_width - width {
                        f.write_char(fill)?;
                    }
                }
            }
        }

        Ok(())
    }
}

/// Trait for state of a partially-formatted `UBig`.
///
/// The state must be such the width (number of digits) is already known.
trait PreparedForFormatting {
    /// Returns the number of characters that will be written.
    fn width(&self) -> usize;

    /// Write to a stream.
    fn write(&mut self, digit_writer: &mut DigitWriter) -> fmt::Result;
}

/// A `Word` prepared for formatting in a power-of-2 radix.
struct PreparedWordInPow2 {
    word: Word,
    log_radix: u32,
    width: usize,
}

impl PreparedWordInPow2 {
    /// Prepare a `Word` for formatting in a power-of-2 radix.
    fn new(word: Word, radix: Digit) -> PreparedWordInPow2 {
        debug_assert!(radix >= 2 && radix.is_power_of_two());
        let log_radix = radix.trailing_zeros();
        debug_assert!(log_radix <= WORD_BITS);
        let width = math::ceil_div(math::bit_len(word), log_radix).max(1) as usize;

        PreparedWordInPow2 {
            word,
            log_radix,
            width,
        }
    }
}

impl PreparedForFormatting for PreparedWordInPow2 {
    fn width(&self) -> usize {
        self.width
    }

    fn write(&mut self, digit_writer: &mut DigitWriter) -> fmt::Result {
        let mask: Word = math::ones(self.log_radix);
        let mut digits = [0; WORD_BITS_USIZE];
        for idx in 0..self.width {
            let digit = ((self.word >> (idx as u32 * self.log_radix)) & mask) as u8;
            digits[self.width - 1 - idx] = digit;
        }
        digit_writer.write(&digits[..self.width])
    }
}

/// A large number prepared for formatting in a power-of-2 radix.
struct PreparedLargeInPow2<'a> {
    words: &'a [Word],
    log_radix: u32,
    width: usize,
}

impl PreparedLargeInPow2<'_> {
    /// Prepare a large number for formatting in a power-of-2 radix.
    fn new(words: &[Word], radix: Digit) -> PreparedLargeInPow2 {
        debug_assert!(radix::is_radix_valid(radix) && radix.is_power_of_two());
        let log_radix = radix.trailing_zeros();
        debug_assert!(log_radix <= WORD_BITS);

        // No overflow because words.len() * WORD_BITS <= usize::MAX for
        // words.len() <= Buffer::MAX_CAPACITY.
        let width = math::ceil_div(
            words.len() * WORD_BITS_USIZE - words.last().unwrap().leading_zeros() as usize,
            log_radix as usize,
        )
        .max(1);

        PreparedLargeInPow2 {
            words,
            log_radix,
            width,
        }
    }
}

impl PreparedForFormatting for PreparedLargeInPow2<'_> {
    fn width(&self) -> usize {
        self.width
    }

    fn write(&mut self, digit_writer: &mut DigitWriter) -> fmt::Result {
        let mask: Word = math::ones(self.log_radix);

        let mut it = self.words.iter().rev();
        let mut word = it.next().unwrap();
        let mut bits = (self.width * self.log_radix as usize
            - (self.words.len() - 1) * WORD_BITS_USIZE) as u32;

        loop {
            let digit;
            if bits < self.log_radix {
                match it.next() {
                    Some(w) => {
                        let extra_bits = self.log_radix - bits;
                        bits = WORD_BITS - extra_bits;
                        digit = ((word << extra_bits | w >> bits) & mask) as u8;
                        word = w;
                    }
                    None => break,
                }
            } else {
                bits -= self.log_radix;
                digit = ((word >> bits) & mask) as u8;
            }
            digit_writer.write(&[digit])?;
        }
        debug_assert!(bits == 0);
        Ok(())
    }
}

/// A `Word` prepared for formatting in a non-power-of-2 radix.
struct PreparedWordInNonPow2 {
    // digits[start_index..] actually used.
    digits: [u8; radix::MAX_WORD_DIGITS_NON_POW_2],
    start_index: usize,
}

impl PreparedWordInNonPow2 {
    /// Prepare a `Word` for formatting in a non-power-of-2 radix.
    fn new(mut word: Word, radix: Digit, min_digits: usize) -> PreparedWordInNonPow2 {
        debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
        let radix_info = radix::radix_info(radix);

        let mut prepared = PreparedWordInNonPow2 {
            digits: [0; radix::MAX_WORD_DIGITS_NON_POW_2],
            start_index: radix::MAX_WORD_DIGITS_NON_POW_2,
        };

        let max_start = radix::MAX_WORD_DIGITS_NON_POW_2 - min_digits;
        while prepared.start_index > max_start || word != 0 {
            let (new_word, d) = radix_info.fast_div_radix.div_rem(word);
            word = new_word;
            prepared.start_index -= 1;
            prepared.digits[prepared.start_index] = d as u8;
        }

        prepared
    }
}

impl PreparedForFormatting for PreparedWordInNonPow2 {
    fn width(&self) -> usize {
        radix::MAX_WORD_DIGITS_NON_POW_2 - self.start_index
    }

    fn write(&mut self, digit_writer: &mut DigitWriter) -> fmt::Result {
        digit_writer.write(&self.digits[self.start_index..])
    }
}

/// A medium number prepared for formatting in a non-power-of-2 radix.
/// Must have no more than CHUNK_LEN * digits_per_word digits.
struct PreparedMediumInNonPow2 {
    top_group: PreparedWordInNonPow2,
    // Little endian in groups of digits_per_word.
    low_groups: [Word; CHUNK_LEN],
    num_low_groups: usize,
    radix: Digit,
}

impl PreparedMediumInNonPow2 {
    /// Prepare a medium number for formatting in a non-power-of-2 radix.
    fn new(number: &UBig, radix: Digit) -> PreparedMediumInNonPow2 {
        debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
        let radix_info = radix::radix_info(radix);

        let (mut buffer, mut buffer_len) = ubig_to_chunk_buffer(number);

        let mut low_groups = [0; CHUNK_LEN];
        let mut num_low_groups = 0;

        while buffer_len > 1 {
            let rem = div::fast_div_by_word_in_place(
                &mut buffer[..buffer_len],
                radix_info.fast_div_range_per_word,
            );
            low_groups[num_low_groups] = rem;
            num_low_groups += 1;

            while buffer[buffer_len - 1] == 0 {
                buffer_len -= 1;
            }
        }
        assert!(buffer_len == 1);
        PreparedMediumInNonPow2 {
            top_group: PreparedWordInNonPow2::new(buffer[0], radix, 1),
            low_groups,
            num_low_groups,
            radix,
        }
    }
}

impl PreparedForFormatting for PreparedMediumInNonPow2 {
    fn width(&self) -> usize {
        let radix_info = radix::radix_info(self.radix);
        self.top_group.width() + self.num_low_groups * radix_info.digits_per_word
    }

    fn write(&mut self, digit_writer: &mut DigitWriter) -> fmt::Result {
        let radix_info = radix::radix_info(self.radix);

        self.top_group.write(digit_writer)?;

        for group_word in self.low_groups[..self.num_low_groups].iter().rev() {
            let mut prepared =
                PreparedWordInNonPow2::new(*group_word, self.radix, radix_info.digits_per_word);
            prepared.write(digit_writer)?;
        }
        Ok(())
    }
}

/// A large number prepared for formatting in a non-power-of-2 radix.
struct PreparedLargeInNonPow2 {
    top_chunk: PreparedMediumInNonPow2,
    // radix^((digits_per_word * CHUNK_LEN) << i)
    radix_powers: Vec<UBig>,
    // little endian chunks: (i, (digits_per_word * CHUNK_LEN)<<i digit number)
    // decreasing in size, so there is a logarithmic number of them
    big_chunks: Vec<(usize, UBig)>,
    radix: Digit,
}

impl PreparedLargeInNonPow2 {
    /// Prepare a medium number for formatting in a non-power-of-2 radix.
    fn new(number: &UBig, radix: Digit) -> PreparedLargeInNonPow2 {
        debug_assert!(radix::is_radix_valid(radix) && !radix.is_power_of_two());
        let radix_info = radix::radix_info(radix);

        let mut radix_powers = Vec::new();
        let mut big_chunks = Vec::new();
        let chunk_power = UBig::from_word(radix_info.range_per_word).pow(CHUNK_LEN);
        if chunk_power > *number {
            return PreparedLargeInNonPow2 {
                top_chunk: PreparedMediumInNonPow2::new(number, radix),
                radix_powers,
                big_chunks,
                radix,
            };
        }

        radix_powers.push(chunk_power);
        loop {
            let prev = radix_powers.last().unwrap();
            // Avoid multiplication if we know prev * prev > number just by looking at lengths.
            if 2 * prev.len() - 1 > number.len() {
                break;
            }
            let new = prev * prev;
            if new > *number {
                break;
            }
            radix_powers.push(new);
        }

        let mut power_iter = radix_powers.iter().enumerate().rev();
        let mut x = {
            let (i, p) = power_iter.next().unwrap();
            let (q, r) = number.div_rem(p);
            big_chunks.push((i, r));
            q
        };
        for (i, p) in power_iter {
            if x >= *p {
                let (q, r) = x.div_rem(p);
                big_chunks.push((i, r));
                x = q;
            }
        }

        PreparedLargeInNonPow2 {
            top_chunk: PreparedMediumInNonPow2::new(&x, radix),
            radix_powers,
            big_chunks,
            radix,
        }
    }

    /// Write (digits_per_word * CHUNK_LEN) << i digits.
    fn write_big_chunk(&self, digit_writer: &mut DigitWriter, i: usize, x: UBig) -> fmt::Result {
        if i == 0 {
            self.write_chunk(digit_writer, x)
        } else {
            let (q, r) = x.div_rem(&self.radix_powers[i - 1]);
            self.write_big_chunk(digit_writer, i - 1, q)?;
            self.write_big_chunk(digit_writer, i - 1, r)
        }
    }

    /// Write digits_per_word * CHUNK_LEN digits.
    fn write_chunk(&self, digit_writer: &mut DigitWriter, x: UBig) -> fmt::Result {
        let radix_info = radix::radix_info(self.radix);
        let (mut buffer, mut buffer_len) = ubig_to_chunk_buffer(&x);

        let mut groups = [0; CHUNK_LEN];

        for group in groups.iter_mut() {
            *group = div::fast_div_by_word_in_place(
                &mut buffer[..buffer_len],
                radix_info.fast_div_range_per_word,
            );
            while buffer_len != 0 && buffer[buffer_len - 1] == 0 {
                buffer_len -= 1;
            }
        }
        assert_eq!(buffer_len, 0);

        for group in groups.iter().rev() {
            let mut prepared =
                PreparedWordInNonPow2::new(*group, self.radix, radix_info.digits_per_word);
            prepared.write(digit_writer)?;
        }

        Ok(())
    }
}

impl PreparedForFormatting for PreparedLargeInNonPow2 {
    fn width(&self) -> usize {
        let mut num_digits = self.top_chunk.width();
        let radix_info = radix::radix_info(self.radix);
        for (i, _) in &self.big_chunks {
            num_digits += (radix_info.digits_per_word * CHUNK_LEN) << i;
        }
        num_digits
    }

    fn write(&mut self, digit_writer: &mut DigitWriter) -> fmt::Result {
        self.top_chunk.write(digit_writer)?;

        let mut big_chunks = mem::take(&mut self.big_chunks);
        for (i, val) in big_chunks.drain(..).rev() {
            self.write_big_chunk(digit_writer, i, val)?;
        }
        Ok(())
    }
}

fn ubig_to_chunk_buffer(x: &UBig) -> ([Word; CHUNK_LEN], usize) {
    let mut buffer = [0; CHUNK_LEN];
    let buffer_len;
    match x.repr() {
        Small(0) => {
            buffer_len = 0;
        }
        Small(word) => {
            buffer_len = 1;
            buffer[0] = *word;
        }
        Large(b) => {
            buffer_len = b.len();
            buffer[..buffer_len].copy_from_slice(b);
        }
    }
    (buffer, buffer_len)
}