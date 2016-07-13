
mod consts;
mod worker;


pub use self::consts::NsOS as NsConsts;
pub use self::worker::NsWorker;


pub fn init()
{
    println!("Win32 init ");
}
