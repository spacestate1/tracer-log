use std::ops::{Bound, RangeBounds};

////////////////// String char parser //////////////////
trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

mod data;
mod stats;
#[cfg(feature = "rtu")]
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use chrono::prelude::*;
    ////////Get cli arguments ///////////
    let args: Vec<String> = env::args().collect();
    ////////Get dGatetime ///////////
    let utc: DateTime<Utc> = Utc::now();
    let format = "%s%6f";
    let dt = utc.format(format).to_string();
    //////////Load Serial RS485lib //////////
    use tokio_serial::{Serial, SerialPortSettings};
    use tokio_modbus::prelude::*;
    let tty_path = "/dev/ttyUSB0";
    let slave = Slave(0x01);
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 115200;
    settings.stop_bits = tokio_serial::StopBits::One;
    let port = Serial::from_path(tty_path, &settings).unwrap();
   ////////Connect /////////// 
    let mut ctx = rtu::connect_slave(port, slave).await?;
    //let rsp4 = ctx.read_input_registers(0x3300, 31).await?;
    
    //let c01 = ctx.read_coils(2, 1).await?;
    //let c02 = ctx.read_coils(3, 1).await?;
    //let c03 = ctx.read_coils(5, 1).await?;
    //let c04 = ctx.read_coils(6, 1).await?;

  //   println!("Maximum input volt (PV) today {:?}",rsp4[0] as f32);

    ////////Print information ///////////
    match args.len() {
        // no arguments passed
        1 => {
            println!("Argument not understood");
                println!("Available options");
                println!("-dr Read mode");
                println!("-dh CSV header mode");
                println!("-d CSV no header mode");
                println!("-s stats CSV no header mode");
                println!("-sh stats CSV no header mode");

        },
        // one argument passed
        2 => {
            let ar01 = &args[1]; 
            let ar02 = ar01.substring(0, 2); 
            if ar02 == "-d" { 
                ////////Read Device Registers for basic data ///////////
                let mut rsp = ctx.read_input_registers(0x311A, 2).await?;
                let mut rsp2 = ctx.read_input_registers(0x311D, 1).await?;
                let mut rsp3 = ctx.read_input_registers(0x3100, 19).await?;
                data::data01(&dt,ar01,rsp.as_mut_slice(), rsp2.as_mut_slice(),rsp3.as_mut_slice());
            }
            ////////Read Device Registers for secondary data ///////////
            else if ar02 == "-s" { 
                let mut rsp4 = ctx.read_input_registers(0x3300, 31).await?;
                stats::stats01(&dt,ar01,rsp4.as_mut_slice());
            }
            else {  
                println!("Argument not understood");
                println!("Available options");
                println!("-dr Read mode");
                println!("-dh CSV header mode");
                println!("-d CSV no header mode");
                println!("-s stats CSV no header mode");
                println!("-sh stats CSV no header mode");
            } 
            
     
    }, 
        _ => {
            // show a help message
           println!("help()");
        }
        }
    Ok(())   
    }
#[cfg(not(feature = "rtu"))]
pub fn main() {
    println!("feature `rtu` is required to run this example");
    std::process::exit(1);
}
