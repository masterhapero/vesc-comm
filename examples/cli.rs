use embedded_hal::serial::{Read, Write};
use serialport::*;
use std::time::Duration;
use vesc_comm::VescConnection;

fn main() {
    let (port1, port2) = {
        let mut port = serialport::new("/dev/ttyACM0", 115200)
            .timeout(Duration::from_millis(100))
            .open()
            .unwrap();
        (Port::new(port.try_clone().unwrap()), Port::new(port))
    };

    let mut conn = VescConnection::new(port1, port2);
    println!("Attempting get fw version");
    dbg!(conn.get_fw_version()).ok();
    println!("Attempting get values");
    dbg!(conn.get_values()).ok();
    let mut i = 0;
    while i < 20 {
        println!("Sending speed 3000");
        conn.set_rpm(3000);
        std::thread::sleep_ms(500);
        i += 1;
    }
    println!("Sending speed 0");
    conn.set_rpm(0);

}

struct Port {
    inner: Box<dyn SerialPort>,
}

impl Port {
    fn new(inner: Box<dyn SerialPort>) -> Self {
        Port { inner }
    }
}

impl Read<u8> for Port {
    type Error = std::io::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buf = [0u8];
        println!("Attempting read");
        match (*self).inner.read(&mut buf) {
            Ok(1) => {
                println!("Read {}", buf[0]);
                Ok(buf[0])
            }
            Ok(_) => {
                println!("Read wrong number of bytes");
                Err(nb::Error::Other(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "read wrong number of bytes",
                )))
            }
            Err(e) => {
                println!("Other read issue {}", e);
                Err(nb::Error::Other(e))
            }
        }
    }
}

impl Write<u8> for Port {
    type Error = std::io::Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // println!("Writing {}", word);
        match (*self).inner.write(&[word]) {
            Ok(1) => {
                // println!("Wrote {}", word);
                Ok(())
            }
            Ok(_) => Err(nb::Error::WouldBlock),
            Err(e) => match e.kind() {
                std::io::ErrorKind::WouldBlock => Err(nb::Error::WouldBlock),
                _ => {
                    println!("Write error {}", e);
                    Err(nb::Error::Other(e))
                }
            }
        }
    }
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}
