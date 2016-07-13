
mod consts;
mod worker;

pub use self::consts::ns_os as NsConsts;
pub use self::worker::NsWorker;


pub type NsRawFd = i32;

pub fn init()
{
    println!("Unix init");
}
