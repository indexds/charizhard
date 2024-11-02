use heapless::String;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Default)]
pub struct HeaplessString<const N: usize>(String<N>);

impl<const N: usize> HeaplessString<N> {
    pub fn new() -> Self {
        Self(String::<N>::new())
    }

    pub fn inner(&self) -> String<N> {
        self.0.clone()
    }

    pub fn push_str(&mut self, s: &str) -> anyhow::Result<()> {
        if s.len() > N {
            return Err(anyhow::anyhow!("String too long."));
        }

        _ = self.0.push_str(s);

        Ok(())
    }

    pub fn from_utf8(s: &[u8]) -> anyhow::Result<Self> {
        let mut heapless_string = HeaplessString::<N>::new();

        _ = heapless_string.0.push_str(core::str::from_utf8(s)?);

        Ok(heapless_string)
    }

    pub fn from_str(s: &str) -> Self {
        let mut heapless_string = HeaplessString::new();

        _ = heapless_string.push_str(s);

        heapless_string
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn chars(&self) -> std::str::Chars<'_> {
        self.0.chars()
    }

    pub fn trim(&self) -> anyhow::Result<HeaplessString<N>> {
        let trimmed = self.0.trim();

        let mut heapless_string = HeaplessString::new();

        heapless_string.push_str(trimmed)?;

        Ok(heapless_string)
    }

    pub fn clean_string(&self) -> HeaplessString<N> {
        self.chars()
            .filter(|&c| c.is_ascii() && (c.is_ascii_graphic() || c.is_ascii_whitespace()))
            .collect()
    }
}

impl<const N: usize> FromIterator<char> for HeaplessString<N> {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut heapless_string = HeaplessString::<N>::new();

        for c in iter {
            if heapless_string.0.len() < N {
                if let Err(_) = heapless_string.0.push(c) {
                    break;
                }
            } else {
                break;
            }
        }

        heapless_string
    }
}

impl<const N: usize> TryInto<HeaplessString<N>> for &str {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<HeaplessString<N>> {
        let mut heapless_string = HeaplessString::<N>::new();
        heapless_string.push_str(&self)?;

        Ok(heapless_string)
    }
}

impl<const N: usize> TryInto<HeaplessString<N>> for heapless::String<N> {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<HeaplessString<N>> {
        let mut heapless_string = HeaplessString::<N>::new();
        heapless_string.push_str(&self)?;

        Ok(heapless_string)
    }
}

impl<const N: usize> TryInto<heapless::String<N>> for HeaplessString<N> {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<heapless::String<N>> {
        Ok(self.0)
    }
}

impl<const N: usize> fmt::Display for HeaplessString<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0.as_str())
    }
}