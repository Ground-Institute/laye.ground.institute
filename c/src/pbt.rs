// "that laye itself laye ground institue itself has its own pbt governing itself, and you can see it on github, and that is what verified means"
// "Ground Institute would be governing laye.ground.institute"
//
// What C reads of the pbt it is deployed with: who administers the laye it
// names. The words are ground's: a `#` line is a comment, a block is a word,
// perhaps a name, and braces, and inside it `key: value`.

struct Block {
    kind: String,
    name: String,
    fields: Vec<(String, String)>,
}

fn skip_blank(s: &[char], at: &mut usize) {
    while *at < s.len() {
        if s[*at].is_whitespace() {
            *at += 1;
        } else if s[*at] == '#' {
            while *at < s.len() && s[*at] != '\n' {
                *at += 1;
            }
        } else {
            break;
        }
    }
}

fn word(s: &[char], at: &mut usize) -> String {
    let from = *at;
    while *at < s.len() && !s[*at].is_whitespace() && !matches!(s[*at], ':' | '{' | '}') {
        *at += 1;
    }
    s[from..*at].iter().collect()
}

fn value(s: &[char], at: &mut usize) -> String {
    if *at >= s.len() {
        return String::new();
    }
    let quote = s[*at];
    if quote == '"' || quote == '`' {
        *at += 1;
        let from = *at;
        while *at < s.len() && s[*at] != quote {
            *at += 1;
        }
        let said = s[from..*at].iter().collect();
        if *at < s.len() {
            *at += 1;
        }
        return said;
    }
    if quote == '[' {
        while *at < s.len() && s[*at] != ']' {
            *at += 1;
        }
        if *at < s.len() {
            *at += 1;
        }
        return String::new();
    }
    let from = *at;
    while *at < s.len() && !s[*at].is_whitespace() && s[*at] != '}' {
        *at += 1;
    }
    s[from..*at].iter().collect()
}

/// The fields of the block `at` stands in, to its closing brace. A block
/// inside it is stepped over.
fn fields(s: &[char], at: &mut usize) -> Vec<(String, String)> {
    let mut out = Vec::new();
    loop {
        skip_blank(s, at);
        if *at >= s.len() {
            break;
        }
        if s[*at] == '}' {
            *at += 1;
            break;
        }
        let key = word(s, at);
        if key.is_empty() {
            *at += 1;
            continue;
        }
        skip_blank(s, at);
        if *at < s.len() && s[*at] == ':' {
            *at += 1;
            skip_blank(s, at);
            out.push((key, value(s, at)));
            continue;
        }
        if *at < s.len() && s[*at] != '{' {
            word(s, at);
            skip_blank(s, at);
        }
        if *at < s.len() && s[*at] == '{' {
            *at += 1;
            fields(s, at);
        }
    }
    out
}

fn blocks(pbt: &str) -> Vec<Block> {
    let s: Vec<char> = pbt.chars().collect();
    let mut at = 0;
    let mut out = Vec::new();
    loop {
        skip_blank(&s, &mut at);
        if at >= s.len() {
            break;
        }
        let kind = word(&s, &mut at);
        if kind.is_empty() {
            at += 1;
            continue;
        }
        skip_blank(&s, &mut at);
        let name = if at < s.len() && s[at] != '{' {
            let name = value(&s, &mut at);
            skip_blank(&s, &mut at);
            name
        } else {
            String::new()
        };
        if at >= s.len() || s[at] != '{' {
            continue;
        }
        at += 1;
        let fields = fields(&s, &mut at);
        out.push(Block { kind, name, fields });
    }
    out
}

fn field<'a>(block: &'a Block, key: &str) -> Option<&'a str> {
    block
        .fields
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
        .filter(|v| !v.is_empty())
}

/// Who a pbt says administers its laye: the administrator of the org that
/// governs it. None when it names no laye, no org governs it, or that org
/// names nobody.
pub fn administrator_of(pbt: &str) -> Option<String> {
    let all = blocks(pbt);
    let laye = all.iter().find(|b| b.kind == "laye")?;
    let org = field(laye, "org")?;
    let governs = all.iter().find(|b| b.kind == "org" && b.name == org)?;
    field(governs, "administrator").map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::administrator_of;
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn the_pbt_c_is_deployed_with_names_its_administrator() {
        let ours = include_str!("../../controls/laye.pbt");
        assert_eq!(administrator_of(ours).as_deref(), Some("teranos"));
    }

    #[test]
    fn a_pbt_that_names_nobody_names_nobody() {
        assert_eq!(administrator_of(""), None);
        assert_eq!(administrator_of("org A {\n  administrator: a\n}\n"), None);
        assert_eq!(
            administrator_of("laye {\n  url: \"https://laye.example\"\n}\n"),
            None
        );
        assert_eq!(
            administrator_of("org A {\n}\nlaye {\n  url: \"https://laye.example\"\n  org: A\n}\n"),
            None
        );
        assert_eq!(
            administrator_of("org A {\n  administrator: a\n}\nlaye {\n  org: B\n}\n"),
            None
        );
    }

    #[test]
    fn the_org_that_governs_is_the_one_whose_administrator_is_named() {
        let two = "# \"a { quoted } comment: here\"\n\
            person a {\n}\nperson b {\n}\n\
            org A {\n  github: \"https://github.com/a\"\n  sentry {\n    dsn: \"https://tim@example.com/1\"\n  }\n  administrator: a\n}\n\
            org B {\n  administrator: b\n}\n\
            laye {\n  url: \"https://laye.example\"\n  org: B\n}\n";
        assert_eq!(administrator_of(two).as_deref(), Some("b"));
    }
}
