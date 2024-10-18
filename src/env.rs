use heapless::String as HeaplessString;

#[derive(Debug)]
pub struct EnvVars{
    pub sta_ssid: HeaplessString<32>,
    pub sta_passwd: HeaplessString<64>,
    pub ap_ssid: HeaplessString<32>,
    pub ap_passwd: HeaplessString<64>,
}

impl EnvVars{
    fn env_var(env_var: &str) -> Result<&str, anyhow::Error>{
        let var = include_str!(".env").lines()
                .find(|line| line.starts_with(&env_var))
                .unwrap_or("")
                .split('=')
                .nth(1)
                .unwrap_or("");
        
        Ok(var)
    }

    fn construct_field<const N: usize>(var: &str) -> anyhow::Result<HeaplessString<N>> {
        let var_env = EnvVars::env_var(var)?;
        let mut var = HeaplessString::<N>::new();

        let _ = var.push_str(&var_env);

        Ok(var)
    }

    pub fn new() -> Result<Self, anyhow::Error>{
        Ok(Self { 
            sta_ssid: EnvVars::construct_field::<32>("STA_SSID")?,
            sta_passwd: EnvVars::construct_field::<64>("STA_PASSWD")?, 
            ap_ssid: EnvVars::construct_field::<32>("AP_SSID")?,
            ap_passwd: EnvVars::construct_field::<64>("AP_PASSWD")?,
        })
    }   
}