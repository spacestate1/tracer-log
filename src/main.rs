mod data;
mod stats;
mod status;
mod rating; 

#[cfg(feature = "rtu")]
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use chrono::prelude::*;
    use clap::{App, Arg};
    use std::time::Duration;
    use tokio::time::timeout;
    ////////Get datetime ///////////
    let utc: DateTime<Utc> = Utc::now();
    let format = "%s";
    let dt = utc.format(format).to_string();
    
    //////////Load Serial RS485lib //////////
    use tokio_serial::{Serial, SerialPortSettings};
    use tokio_modbus::prelude::*;
    let tty_path = "/dev/ttyUSB0";
    let client01 = Slave(0x01);
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 115200;
    settings.stop_bits = tokio_serial::StopBits::One;
    let port = Serial::from_path(tty_path, &settings).unwrap();
   
    ////////Connect /////////// 
    let mut ctx = rtu::connect_slave(port, client01).await?;
    

////////////////// Parse CLI arguments //////////////////
    let matches = App::new("tracer-log")
        .arg(
            Arg::new("basic-data")
                .about("eLog01 CSV data") 
                .short('a') 
                .long("data") 
                .multiple_occurrences(true)
                )

        .arg(
         Arg::new("basic-data-header")
                .about("eLog01 CSV data with header")
                .short('b') 
                .long("dataheader") 
                .multiple_occurrences(true)

                 )

        .arg(
         Arg::new("basic-data-readable")
                .about("eLog01 Readable data") 
                .short('c') 
                .long("dataread") 
                .multiple_occurrences(true)

                 )
        .arg(
         Arg::new("csvstats")
                .about("eLog01 CSV stat data")
                .short('d')
                .long("stats")
                .multiple_occurrences(true)

                 )
        .arg(
         Arg::new("stats2")
                .about("eLog01 CSV stat data with header")
                .short('e')
                .long("statsheader")
                .multiple_occurrences(true)

                 )

    .arg(
         Arg::new("stats3")
                .about("eLog01 Readable stats")
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
         Arg::new("livestatus1")
                .about("eLog01 Live panel/battery voltage status")
                .short('i')
                .long("livestatus")
                .multiple_occurrences(true)

                 )

        .arg(
         Arg::new("rating")
                .about("eLog01 Equipment rating")
                .short('j')
                .long("rating")
                .multiple_occurrences(true)
                )

        .arg(
         Arg::new("ratings-direct")
                .about("Direct EP device rating")
                .short('r')
                .long("ratingsdirect")
                .multiple_occurrences(true)

                 )

        .arg(
         Arg::new("status-direct")
                .about("Direct EP device status")
                .short('s')
                .long("statusdirect")
                .multiple_occurrences(true)

                 )

        .arg(
         Arg::new("stats-data-direct")
                .about("Direct stats from EP device")
                .short('u')
                .long("statsdirect")
                .multiple_occurrences(true)

                 )

        .arg(
         Arg::new("live-data-direct")
                .about("Direct data direct from EP device")
                .short('w')
                .long("datadirect")
                .multiple_occurrences(true)

                 )
        .arg(
         Arg::new("csv-live-data-direct")
                .about("Direct data CSV")
                .short('x')
                .long("datadirectcsv")
                .multiple_occurrences(true)

                 )

        .arg(
         Arg::new("csv-header-live-data-direct")
                .about("Direct data CSV with header")
                .short('y')
                .long("ddatadirectcsvheader")
                .multiple_occurrences(true)

                 )

////////////////// Match input, get data from registers //////////////////


               .get_matches();
             if matches.is_present("basic-data") {


             //   let mut rsp = ctx.read_input_registers(0x311A, 2).await?;
             //   let mut rsp2 = ctx.read_input_registers(0x311D, 1).await?;
             //   let mut rsp3 = ctx.read_input_registers(0x3100, 19).await?;
                  let mut rsp = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 2)).await.expect("Connection timeout").unwrap(); 
                  let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.unwrap().unwrap();
                  let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 19)).await.unwrap().unwrap();


                data::data01(&dt,rsp.as_mut_slice(), rsp2.as_mut_slice(),rsp3.as_mut_slice());
                
        
         }

             if matches.is_present("basic-data-csv") {

             //   let mut rsp = ctx.read_input_registers(0x311A, 2).await?;
             //   let mut rsp2 = ctx.read_input_registers(0x311D, 1).await?;
             //   let mut rsp3 = ctx.read_input_registers(0x3100, 19).await?;
                  let mut rsp = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 2)).await.expect("Connection timeout").unwrap();
                  let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.unwrap().unwrap();
                  let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 19)).await.unwrap().unwrap();

                data::data02(&dt,rsp.as_mut_slice(), rsp2.as_mut_slice(),rsp3.as_mut_slice());

         }

             if matches.is_present("basic-data-readable") {

              //  let mut rsp = ctx.read_input_registers(0x311A, 2).await?;
              //  let mut rsp2 = ctx.read_input_registers(0x311D, 1).await?;
              //  let mut rsp3 = ctx.read_input_registers(0x3100, 19).await?;
                let mut rsp = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 2)).await.expect("Connection timeout").unwrap();
                let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.unwrap().unwrap();
                let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 19)).await.unwrap().unwrap();

                data::data03(rsp.as_mut_slice(), rsp2.as_mut_slice(),rsp3.as_mut_slice());

         }

             if matches.is_present("stats-data-direct") {

                 let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3300, 31)).await.expect("Connection timeout").unwrap();

                 stats::stats04(rsp1.as_mut_slice());
                

         }
             if matches.is_present("live-data-direct" ) { 
                 let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 20)).await.expect("Connection timeout").unwrap();
                 let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.expect("Connection timeout").unwrap();
                 
                 data::data05(rsp1.as_mut_slice(),rsp2.as_mut_slice(),rsp3.as_mut_slice());

                 
             }

             if matches.is_present("csv-live-data-direct" ) {
                 let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 20)).await.expect("Connection timeout").unwrap();
                 let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.expect("Connection timeout").unwrap();

                 data::data06(&dt, rsp1.as_mut_slice(),rsp2.as_mut_slice(),rsp3.as_mut_slice());

             }

             if matches.is_present("csv-header-live-data-direct" ) {


                 let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 20)).await.expect("Connection timeout").unwrap();
                 let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.expect("Connection timeout").unwrap();

                 data::data07(&dt, rsp1.as_mut_slice(),rsp2.as_mut_slice(),rsp3.as_mut_slice());



             }


             if matches.is_present("ratings-direct" ) {

               //  let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3000, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3001, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3002, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp4 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3003, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp5 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3004, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp6 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3005, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp7 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3006, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp8 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3007, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp9 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3008, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp10 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x300E, 1)).await.expect("Connection timeout").unwrap();
                   let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3000, 9)).await.expect("Connection timeout").unwrap();
                
                // rating::drate01(&dt,rsp1.as_mut_slice(),rsp2.as_mut_slice(),rsp3.as_mut_slice(),rsp4.as_mut_slice(),rsp5.as_mut_slice(),rsp6.as_mut_slice(),rsp7.as_mut_slice(),rsp8.as_mut_slice(),rsp9.as_mut_slice(),rsp10.as_mut_slice());
                rating::drate02(&dt,rsp1.as_mut_slice());
             }
             if matches.is_present("status-direct" ) {

                 let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3200, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3201, 1)).await.expect("Connection timeout").unwrap();
                 println!("rsp1  - {:?}", rsp1); 
                 println!("rsp1  - {:?}", rsp2);
                 status::dstatus01(&dt,rsp1.as_mut_slice(), rsp2.as_mut_slice());
             }


             if matches.is_present("csvstats") {
               
                //let mut rsp4 = ctx.read_input_registers(0x3300, 31).await?;
                let mut rsp4 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3300, 31)).await.expect("Connection timeout").unwrap();
                stats::stats01(&dt,rsp4.as_mut_slice());

             }

             if matches.is_present("stats2") {

                //let mut rsp4 = ctx.read_input_registers(0x3300, 31).await?;
                let mut rsp4 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3300, 31)).await.expect("Connection timeout").unwrap();
                stats::stats02(&dt,rsp4.as_mut_slice());

             }

             if matches.is_present("stats3") {

                //let mut rsp4 = ctx.read_input_registers(0x3300, 31).await?;
                let mut rsp4 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3300, 31)).await.expect("Connection timeout").unwrap();
                stats::stats03(rsp4.as_mut_slice());

             }

             if  matches.is_present("rs485test") {

                 let rsp = ctx.read_input_registers(0x311A, 2).await?;
                println!("Basic data");
                println!("Battery SOC: {:?} percent, Battery voltage: {:?}v detected",rsp[0] as f32, 0.01 * rsp[1] as f32);
                println!("RS485 connection working")
             }

             if matches.is_present("livestatus1") { 
                //let mut rsp5 = ctx.read_input_registers(0x3200, 3).await?;
                let mut rsp5 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3200, 3)).await.expect("Connection timeout").unwrap();
                status::status01(&dt,rsp5.as_mut_slice());
             }

             if matches.is_present("rating") {
                //let mut rsp5 = ctx.read_input_registers(0x3000, 9).await?;
                let mut rsp5 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3200, 3)).await.expect("Connection timeout").unwrap();
                rating::rating01(&dt,rsp5.as_mut_slice());
             }

    Ok(())   
    }
#[cfg(not(feature = "rtu"))]
pub fn main() {
    println!("feature `rtu` is required to run this example");
    std::process::exit(1);
}
