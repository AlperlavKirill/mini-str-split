struct SplitStr<'remainder, D> {
    remainder: Option<&'remainder str>,
    delimiter: D,
}

impl<'remainder, D> SplitStr<'remainder, D> {
    fn new(init_str: &'remainder str, delimiter: D) -> Self {
        Self {
            remainder: Some(init_str),
            delimiter,
        }
    }
}

impl<'remainder, D> Iterator for SplitStr<'remainder, D>
    where D: Delimiter
{
    type Item = &'remainder str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next_in_str(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}


trait Delimiter {
    fn find_next_in_str(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next_in_str(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next_in_str(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn until_char(s: &str, c: char) -> &str {
    SplitStr::new(s, c)
        .next()
        .expect("SplitStr always returns at least one value")
}

#[test]
fn basic_work() {
    let to_split = "a b c d e";
    let letters: Vec<&str> = SplitStr::new(to_split, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"])
}

#[test]
fn tail_split() {
    let to_split = "a b c d ";
    let letters: Vec<&str> = SplitStr::new(to_split, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""])
}

#[test]
fn test_until_char() {
    let init_str = "a b c d e";
    let until_b = until_char(init_str, 'b');
    assert_eq!(until_b, "a ")
}

