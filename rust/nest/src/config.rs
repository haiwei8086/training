
pub struct NsConfig {
    pub workers: usize
}


impl NsConfig
{
    pub fn new() -> NsConfig
    {
        NsConfig {
            workers: 2
        }
    }
}
