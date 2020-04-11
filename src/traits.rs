use {
    crate::TextSize,
    std::{borrow::Cow, convert::TryInto, rc::Rc, sync::Arc},
};

/// Text-like structures that have a text size.
pub trait TextLen: Copy {
    /// The size of this text-alike.
    fn text_len(self) -> TextSize;
}

impl TextLen for &'_ str {
    #[inline]
    fn text_len(self) -> TextSize {
        self.len().try_into().unwrap()
    }
}

impl TextLen for char {
    #[inline]
    fn text_len(self) -> TextSize {
        (self.len_utf8() as u32).into()
    }
}

impl<D> TextLen for &'_ D
where
    D: TextLen + Copy,
{
    #[inline]
    fn text_len(self) -> TextSize {
        D::text_len(*self)
    }
}

impl TextLen for &'_ String {
    #[inline]
    fn text_len(self) -> TextSize {
        <&str>::text_len(self)
    }
}

impl TextLen for &'_ Box<str> {
    #[inline]
    fn text_len(self) -> TextSize {
        <&str>::text_len(self)
    }
}

macro_rules! impl_textlen_for_smartptr {
    ($(&$ty:ident),+ $(,)?) => {$(
        impl<'a, T: ?Sized> TextLen for &'a $ty<T>
        where
            &'a T: TextLen,
        {
            #[inline]
            fn text_len(self) -> TextSize {
                <&'a T>::text_len(self)
            }
        }
    )+};
}

// <https://internals.rust-lang.org/t/_/12139>
// = node: downstream crates may implement trait `std::marker::Copy` for type `std::boxed::Box<_>`
impl_textlen_for_smartptr!(/*&Box,*/ &Arc, &Rc);

impl<'a, B: ?Sized> TextLen for &'a Cow<'a, B>
    where
        B: ToOwned,
        &'a B: TextLen
{
    fn text_len(self) -> TextSize {
        <&'a B>::text_len(&**self)
    }
}
