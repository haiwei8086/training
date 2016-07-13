
use super::super::{NsConfig, NsResult};

pub struct Nest<'a>
{
    pub config: &'a NsConfig,
    pub modules: Vec<usize>
}

impl<'a> Nest<'a>
{
    pub fn new(config: &mut NsConfig) -> Nest
    {
        Nest {
            config: config,
            modules: Vec::new()
        }
    }

    pub fn modules(&mut self, m: usize) -> &mut Nest<'a>
    {
        self.modules.push(m);
        self
    }

    pub fn listen(&self) -> NsResult<usize>
    {
        println!("Listening...");

        Ok(0)
    }

}
