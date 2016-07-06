use super::{NsConfig, NsResult};


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
        if Nest::is_unix() {
            println!("Unix: MacOS FreeBSD");
        } else if Nest::is_linux() {
            println!("Linux");
        } else if Nest::is_win() {
            println!("Windows");
        }

        Ok(0)
    }

    pub fn is_unix() -> bool
    {
        cfg!(target_os = "macos") || cfg!(target_os = "ios")
        || cfg!(target_os = "freebsd") || cfg!(target_os = "openbsd") || cfg!(target_os = "netbsd")
    }

    pub fn is_linux() -> bool
    {
        cfg!(target_os = "linux") || cfg!(target_os = "android")
    }

    pub fn is_win() -> bool
    {
        cfg!(target_os = "windows")
    }
}
