pub fn firstn(s: &str, n: usize) -> &str {
    let idx = s
        .char_indices()
        .nth(n)
        .map(|(i, _)| i)
        .unwrap_or(s.len());

    return &s[..idx];
}

pub fn lastn(s: &str, n: usize) -> &str {
    let mut new_n = 0;
    let mut d = 0;  // default

    // hacky?
    if n == 0 {
        return "";
    }

    if n > s.len() {
        return s;
    }
    //

    if n > 0 {
        new_n = n - 1;
    }

    if s.len() > 0 {
        d = s.len() - 1;
    }

    let idx = s
        .char_indices()
        .rev()
        .nth(new_n)
        .map(|(i, _)| i)
        .unwrap_or(d);

    return &s[idx..];
}

fn substr(s: &str, begin: usize, end: Option<usize>) -> Result<&str, &str> {
    let mut begin_idx = s.len();
    let mut end_idx = None;
    let mut chars = 0;
    for (i, (idx, _)) in s.char_indices().enumerate() {
        chars += 1;
        if i == begin {
            begin_idx = idx;
        }
        if i == end.unwrap_or(usize::MAX) {
            end_idx = Some(idx);
            break;
        }
    }
    return if begin > chars || (begin == chars && end != Some(begin)) {
        Err("")
    } else if end_idx.is_some() || end.unwrap_or(chars) == chars {
        Ok(&s[begin_idx..end_idx.unwrap_or(s.len())])
    } else {
        Err(&s[begin_idx..s.len()])
    }
}

#[cfg(test)]
mod tests {
    use crate::{firstn, lastn, slice};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_nth() {
        assert_eq!("", firstn("test", 0));
        assert_eq!("t", firstn("test", 1));
        assert_eq!("te", firstn("test", 2));
        assert_eq!("tes", firstn("test", 3));
        assert_eq!("test", firstn("test", 4));
        assert_eq!("test", firstn("test", 10));
        assert_eq!("Hello", firstn("Hello, World!", 5));

        assert_eq!("", firstn("", 0));
        assert_eq!("", firstn("", 1));
        assert_eq!("", firstn("", 5));
    }

    #[test]
    fn test_last() {
        assert_eq!("", lastn("test", 0));
        assert_eq!("t", lastn("test", 1));
        assert_eq!("st", lastn("test", 2));
        assert_eq!("est", lastn("test", 3));
        assert_eq!("test", lastn("test", 4));
        assert_eq!("test", lastn("test", 10));
        assert_eq!("World!", lastn("Hello, World!", 6));

        assert_eq!("", lastn("", 0));
        assert_eq!("", lastn("", 1));
        assert_eq!("", lastn("", 11));
    }

    #[test]
    fn test_substr() {
        let s = "abc🙂";
        assert_eq!(Ok("bc"), substr(s, 1, Some(3)));
        assert_eq!(Ok("c🙂"), substr(s, 2, Some(4)));
        assert_eq!(Ok("c🙂"), substr(s, 2, None));
        assert_eq!(Err("c🙂"), substr(s, 2, Some(99)));
        assert_eq!(Ok(""), substr(s, 2, Some(2)));
        assert_eq!(Ok(""), substr(s, 4, Some(4)));
        assert_eq!(Err(""), substr(s, 5, Some(5)));
        assert_eq!(Err(""), substr(s, 5, Some(9)));
        assert_eq!(Err(""), substr(s, 5, Some(1)));
        assert_eq!(Ok(""), substr("", 0, Some(0)));
        assert_eq!(Err(""), substr("", 0, Some(1)));
    }
}
