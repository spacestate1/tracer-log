mod data; 
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
   ////////Read Device Registers ///////////
    let mut rsp = ctx.read_input_registers(0x311A, 2).await?;
    let mut rsp2 = ctx.read_input_registers(0x311D, 1).await?;
    let mut rsp3 = ctx.read_input_registers(0x3100, 19).await?;
    let rsp4 = ctx.read_input_registers(0x3300, 31).await?;
    
    let c01 = ctx.read_coils(2, 1).await?;
    let c02 = ctx.read_coils(3, 1).await?;
    let c03 = ctx.read_coils(5, 1).await?;
    let c04 = ctx.read_coils(6, 1).await?;

    println!("c01-2 {:?}",c01[0] as bool);
    println!("c01-3 {:?}",c02[0] as bool);
    println!("c01-5 {:?}",c03[0] as bool);
    println!("c01-6 {:?}",c04[0] as bool);
     println!("Maximum input volt (PV) today {:?}",rsp4[0] as f32);

    ////////Print information ///////////
    match args.len() {
        // no arguments passed
        1 => {
            println!("Argument not understood");
                println!("Available options");
                println!("-r Read mode");
                println!("-dh CSV header mode");
                println!("-d CSV no header mode");

        },
        // one argument passed
        2 => {
            let ar01 = &args[1]; 
            data::test01(&dt,ar01,rsp.as_mut_slice(), rsp2.as_mut_slice(),rsp3.as_mut_slice());
            
     
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
