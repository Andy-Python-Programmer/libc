extern "C" {
    pub fn sceWlanDevIsPowerOn() -> i32;
    pub fn sceWlanGetSwitchState() -> i32;
    pub fn sceWlanGetEtherAddr(ether_addr: *mut u8) -> i32;
}

extern "C" {
    pub fn sceWlanDevAttach() -> i32;
    pub fn sceWlanDevDetach() -> i32;
}
