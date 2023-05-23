
fn is_open(c: char) -> bool {
    c == '{' || c == '(' || c == '{'
}

fn matches(c1: char, c2: char) -> bool {
    c1 = '{'
}

pub fn is_valid(str: String) -> bool {
    let mut parens = vec![];
    for c in str.chars() {
        if is_open(c) {
            parens.push(c);
        } else {
            let top = parens.pop().unwrap();
        }
    }
    true
}

#[cfg(test)]
mod test {
    #[test]
    fn is_valid() {
        let x = super::is_valid(String::from("aaa"));
        let y = super::is_valid(String::from("bbb"));
        assert_eq!(x, y)
    }
}
