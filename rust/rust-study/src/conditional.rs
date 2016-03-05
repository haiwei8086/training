
pub fn run(){
    if cfg!(target_os = "macos"){
        // unresolved name `mac_print_powerpc`
        // mac_print_powerpc();
        mac_print();
    }

    if cfg!(target_os = "windows"){
        // unresolved name `windows_print`
        windows_print();
    }

    //foo();
}


#[cfg(all(unix, target_os = "macos",target_arch = "powerpc"))]
fn mac_print_powerpc(){
    println!("this is mac pc");
}

#[cfg(all(unix, target_os = "macos"))]
fn mac_print(){
    println!("this is mac pc");
}

// #[cfg(all(windows, target_os = "windows"))]
fn windows_print(){
    println!("this is windows pc");
}

#[cfg(feature = "foo")]
fn foo(){
    println!("run: cargo run --fetures");
}
