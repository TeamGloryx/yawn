#[derive(Copy, Clone)]
pub enum Platform {
    MobileAndroid,
    MobileIos,
    Web,
    DesktopWindows,
    DesktopLinux,
    DesktopMac,
}

pub trait Owner {
    fn platform(&self) -> Platform;
}
