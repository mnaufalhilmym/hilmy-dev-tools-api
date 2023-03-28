use core::fmt;

pub struct ServiceName<'a>(&'a str);

impl fmt::Display for ServiceName<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

impl ServiceName<'_> {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn account() -> Self {
        Self("account")
    }

    pub fn link() -> Self {
        Self("link")
    }

    pub fn apprepo() -> Self {
        Self("apprepo")
    }
}
