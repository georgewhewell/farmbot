// use linux_embedded_hal::I2cdev;
// use std::sync::Arc;
// use i2cdev::linux::LinuxI2CError;
// use std::sync::Mutex;
//
// #[derive(Clone)]
// pub struct I2cProxy(Arc<Mutex<I2cdev>>);
//
// impl i2c::Write for I2cProxy {
//     type Error = LinuxI2CError;
//
//     fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
//         let mut i2c = self.0.lock().unwrap();
//         i2c.write(addr, bytes)
//     }
// }
//
// impl i2c::Read for I2cProxy {
//     type Error = LinuxI2CError;
//
//     fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
//         let mut i2c = self.0.lock().unwrap();
//         i2c.read(addr, buffer)
//     }
// }
//
// impl i2c::WriteRead for I2cProxy {
//     type Error = linux_embedded_hal::i2cdev::linux::LinuxI2CError;
//
//     fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
//         let mut i2c = self.0.lock().unwrap();
//         i2c.write_read(addr, bytes, buffer)
//     }
// }
