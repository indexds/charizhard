use heapless::String;

#[derive(Debug)]
//Wrapper around heapless::String to allow for TryFrom impl
pub struct HeaplessString<const N: usize>(String<N>);

impl<const N: usize> HeaplessString<N> {
    
    pub fn new() -> Self {
        Self(String::<N>::new())
    }

    pub fn push_str(&mut self, s: &str) -> anyhow::Result<()> {
        
        if s.len() > N {
            return Err(anyhow::anyhow!("String too long."))
        }
        
        _ = self.0.push_str(s);
        
        Ok(())
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