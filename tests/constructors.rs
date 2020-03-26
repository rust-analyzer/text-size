use {
    std::{borrow::Cow, ops::Deref, sync::Arc},
    text_size::*,
};

struct StringLike<'a>(&'a str);

impl Deref for StringLike<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn main() {
    macro_rules! test {
        ($($expr:expr),+ $(,)?) => {
            $({
                let s = $expr;
                let _ = TextSize::of(&s);
            })+
        };
    }

    test! {
        "",
        String::new(),
        Cow::Borrowed(""),
        Cow::Owned::<str>(String::new()),
        StringLike(""),
        Arc::new(""),
        Arc::new(String::new()),
    }
}
