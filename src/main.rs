mod data;
mod stats;
mod status;
#[cfg(feature = "rtu")]
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
   // use std::env;
    use chrono::prelude::*;
    use clap::{App, Arg};
    ////////Get cli arguments ///////////
   // let args: Vec<String> = env::args().collect();
    ////////Get datetime ///////////
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
    

////////////////// Parse CLI arguments //////////////////
    let matches = App::new("tracer-log")
        .arg(
            Arg::new("basic-data")
                .about("CSV data") 
                .short('a') 
                .long("data") 
                .multiple_occurrences(true)
                )

              .arg(
         Arg::new("basic-data2")
                .about("CSV data with header")
                .short('b') 
                .long("dataheader") 
                .multiple_occurrences(true)

                 )

              .arg(
         Arg::new("basic-data-readable")
                .about("Easy read data") 
                .short('c') 
                .long("dataread") 
                .multiple_occurrences(true)

                 )
              .arg(
         Arg::new("stats")
                .about("CSV stat data") 
                .short('d') 
                .long("stats")
                .multiple_occurrences(true)

                 )
              .arg(
         Arg::new("stats2")
                .about("CSV stat data with header")
                .short('e')
                .long("statsheader")
                .multiple_occurrences(true)

                 )

              .arg(
         Arg::new("stats3")
                .about("Readable stats")
                .short('f')
                .long("statsreader")
                .multiple_occurrences(true)

                 )

              .arg(
         Arg::new("rs485test")
                .about("Basic RS485 Connection test")
                .short('g')
                .long("rstest")
                .multiple_occurrences(true)

                 )

              .arg(
         Arg::new("status1")
                .about("Panel/battery voltage status")
                .short('i')
                .long("status")
                .multiple_occurrences(true)

                 )


////////////////// Match input, get data from registers //////////////////


               .get_matches();
             if matches.is_present("basic-data") {

                let mut rsp = ctx.read_input_registers(0x311A, 2).await?;
                let mut rsp2 = ctx.read_input_registers(0x311D, 1).await?;
                let mut rsp3 = ctx.read_input_registers(0x3100, 19).await?;
                let ar01 = "-d"; 

                data::data01(&dt,ar01,rsp.as_mut_slice(), rsp2.as_mut_slice(),rsp3.as_mut_slice());
        
         }

             if matches.is_present("basic-data2") {

                let mut rsp = ctx.read_input_registers(0x311A, 2).await?;
                let mut rsp2 = ctx.read_input_registers(0x311D, 1).await?;
                let mut rsp3 = ctx.read_input_registers(0x3100, 19).await?;
                let ar01 = "-dh";

                data::data01(&dt,ar01,rsp.as_mut_slice(), rsp2.as_mut_slice(),rsp3.as_mut_slice());

         }

             if matches.is_present("basic-data-readable") {

                let mut rsp = ctx.read_input_registers(0x311A, 2).await?;
                let mut rsp2 = ctx.read_input_registers(0x311D, 1).await?;
                let mut rsp3 = ctx.read_input_registers(0x3100, 19).await?;
                let ar01 = "-dr";

                data::data01(&dt,ar01,rsp.as_mut_slice(), rsp2.as_mut_slice(),rsp3.as_mut_slice());

         }

             if matches.is_present("stats") {
               
                let mut rsp4 = ctx.read_input_registers(0x3300, 31).await?;
                let ar01 = "-s";
                stats::stats01(&dt,ar01,rsp4.as_mut_slice());

             }

             if matches.is_present("stats2") {

                let mut rsp4 = ctx.read_input_registers(0x3300, 31).await?;
                let ar01 = "-sh";
                stats::stats01(&dt,ar01,rsp4.as_mut_slice());

             }

             if matches.is_present("stats3") {

                let mut rsp4 = ctx.read_input_registers(0x3300, 31).await?;
                let ar01 = "-sr";
                stats::stats01(&dt,ar01,rsp4.as_mut_slice());

             }

             if  matches.is_present("rs485test") {

                 let rsp = ctx.read_input_registers(0x311A, 2).await?;
                println!("Basic data");
                println!("Battery SOC: {:?} percent, Battery voltage: {:?}v detected",rsp[0] as f32, 0.01 * rsp[1] as f32);
                println!("RS485 connection working")
             }

             if matches.is_present("status1") { 
                 let ar01 = "-x";
                let mut rsp5 = ctx.read_input_registers(0x3200, 3).await?;
                status::status01(&dt,ar01,rsp5.as_mut_slice());
             }

    Ok(())   
    }
#[cfg(not(feature = "rtu"))]
pub fn main() {
    println!("feature `rtu` is required to run this example");
    std::process::exit(1);
}
