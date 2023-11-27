use crate::linux::port::KPort;

pub struct RPort{
    k_port: KPort,
}

impl RPort{
    pub fn new(k_port: KPort) -> Self{



        RPort{
            k_port,
        }
    }

}