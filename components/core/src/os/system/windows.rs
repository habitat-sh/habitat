use crate::{error::Result,
            os::system::Uname};

pub fn uname() -> Result<Uname> {
    Ok(Uname { sys_name:  String::from("Windows"),
               node_name: String::from("CHEF-WIN10"),
               release:   String::from("10.0.14915"),
               version:   String::from("Microsoft Windows 10 Enterprise Insider Preview"),
               machine:   String::from("x86_64"), })
}
