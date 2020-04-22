use std::fmt;

pub fn write_string_sum<I>(f: &mut fmt::Formatter<'_>, strings: I) -> fmt::Result
where
    I: Iterator<Item = String>,
{
    for (i, string) in strings.enumerate() {
        if i > 0 && !string.starts_with('-') {
            write!(f, "+")?
        }
        write!(f, "{}", string)?;
    }
    Ok(())
}
