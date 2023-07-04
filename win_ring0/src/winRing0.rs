use win_kernel_driver::WinKernelDriver;
use win_kernel_driver::DriverBuilder;
use super::ioctl::IOCTL;
use winapi::shared::minwindef::{DWORD};
use std::mem;

/// WinRing0 driver
pub struct WinRing0 { 
    driver: WinKernelDriver
}

#[repr(C)]
#[repr(packed(4))]
#[derive(Clone)]
struct OlsWriteIoPortInput {
    port_number: u32,
    char_data: u8,
}

impl<'a> WinRing0 {
    pub fn new() -> Self {
        #[cfg(target_pointer_width = "64")]
            let driver = include_bytes!("../winRing0x64.sys");
        #[cfg(target_pointer_width = "32")]
            let driver = include_bytes!("../winRing0.sys");

        let driver = DriverBuilder::new()
            .set_device_description("Rust winRing0 driver")
            .set_device_id("WinRing0_1_2_0")
            .set_device_type(40000)
            .set_driver_bin(driver.to_vec())
            .build().unwrap();

        WinRing0 {
            driver: driver
        }
    }

    /// Install the winRing0 driver.
    pub fn install(&self) -> Result<(), String> {
        return self.driver.install();
    }

    /// Open the winRing0 driver for communication
    pub fn open(&mut self) -> Result<(), String> {
        return self.driver.open();
    }

    /// Close the winRing0 driver handle
    pub fn close(&mut self) -> Result<(), String> {
        self.driver.close()
    }

    /// Uninstall the winRing0 driver
    pub fn uninstall(&mut self) -> Result<(), String> {
        self.driver.uninstall()
    }

    /// Read an MSR register
    pub fn readMsr(&self, msr: DWORD) -> Result<u64, String> {
        match self.driver.io(IOCTL::OLS_READ_MSR as u32, msr) {
            Ok(res) => { return Ok(res); }
            Err(err) => { return Err(format!("Error reading msr: {}", err)); }
        }
    }

    /// Raw IO function. See [WinKernelDriver::io] for more information
    pub fn io(&self, ioctl: IOCTL, in_buffer: u32) -> Result<u64, String> {
        match self.driver.io(ioctl as u32, in_buffer) {
            Ok(res) => { return Ok(res); },
            Err(err) => { return Err(format!("Error doing IO: {}", err)); }
        }
    }

    pub fn read_io_port_byte(&self, port: u32) -> Result<u64, String> {
        self.driver.io(IOCTL::OLS_READ_IO_PORT_BYTE as u32, port)
    }

    pub fn write_io_port_byte(&self, port: u32, value: u32) -> Result<u64, String> {
        let input = OlsWriteIoPortInput {
            port_number: port,
            char_data: value as u8,
        };

        let size = mem::size_of_val(&input.port_number) + mem::size_of_val(&input.char_data);
        let buffer: [u8; 8] = unsafe { mem::transmute(input.clone()) };

        self.driver.io_buffer(IOCTL::OLS_WRITE_IO_PORT_BYTE as u32, &buffer[0..size])
    }

}
