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
                .about("CSV data") 
                .short('a') 
                .long("data") 
                .multiple_occurrences(true)
                )

        .arg(
         Arg::new("basic-data-header")
                .about("CSV data with header")
                .short('b') 
                .long("dataheader") 
                .multiple_occurrences(true)

                 )

        .arg(
         Arg::new("basic-data-readable")
                .about("Readable data") 
                .short('c') 
                .long("dataread") 
                .multiple_occurrences(true)

                 )
        .arg(
         Arg::new("csvstats")
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
         Arg::new("livestatus1")
                .about("Live panel/battery voltage status")
                .short('i')
                .long("livestatus")
                .multiple_occurrences(true)

                 )

        .arg(
         Arg::new("rating")
                .about("Equipment rating")
                .short('j')
                .long("rating")
                .multiple_occurrences(true)
                )

        .arg(
         Arg::new("stats-data-direct")
                .about("Stats direct from EP device")
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


                let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3300, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3301, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3302, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp4 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3303, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp5 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3304, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp6 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3304, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp7 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3305, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp8 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3306, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp9 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3306, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp10 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3307, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp11 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3308, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp12 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3308, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp13 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3309, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp14 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330A, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp15 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330A, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp16 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330B, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp17 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330C, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp18 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330C, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp19 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330D, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp20 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330E, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp21 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330E, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp22 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x330F, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp23 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3310, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp24 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3310, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp25 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3311, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp26 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3312, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp27 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3312, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp28 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3313, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp29 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3314, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp30 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3314, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp31 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3315, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp32 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x331B, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp33 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x331B, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp34 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x331C, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp35 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x331D, 1)).await.expect("Connection timeout").unwrap();
                let mut rsp36 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x331E, 1)).await.expect("Connection timeout").unwrap();
                
            data::data04(rsp1.as_mut_slice(),rsp2.as_mut_slice(),rsp3.as_mut_slice(),rsp4.as_mut_slice(),rsp5.as_mut_slice(),rsp6.as_mut_slice(),rsp7.as_mut_slice(),rsp8.as_mut_slice(),rsp9.as_mut_slice(),rsp10.as_mut_slice(),rsp11.as_mut_slice(),rsp12.as_mut_slice(),rsp13.as_mut_slice(),rsp14.as_mut_slice(),rsp15.as_mut_slice(),rsp16.as_mut_slice(),rsp17.as_mut_slice(),rsp18.as_mut_slice(),rsp19.as_mut_slice(),rsp20.as_mut_slice(),rsp21.as_mut_slice(),rsp22.as_mut_slice(),rsp23.as_mut_slice(),rsp24.as_mut_slice(),rsp25.as_mut_slice(),rsp26.as_mut_slice(),rsp27.as_mut_slice(),rsp28.as_mut_slice(),rsp29.as_mut_slice(),rsp30.as_mut_slice(),rsp31.as_mut_slice(),rsp32.as_mut_slice(),rsp33.as_mut_slice(),rsp34.as_mut_slice(),rsp35.as_mut_slice(),rsp36.as_mut_slice());

         }
             if matches.is_present("live-data-direct" ) { 
                 let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3101, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3102, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp4 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3103, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp5 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3104, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp6 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3105, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp7 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3106, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp8 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3106, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp9 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3107, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp10 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310C, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp11 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310D, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp12 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310E, 1)).await.expect("Connection timeout").unwrap();
                 //let mut rsp13 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310E, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp14 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310F, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp15 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3110, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp16 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3111, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp17 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3112, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp18 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp19 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311B, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp20 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.expect("Connection timeout").unwrap();
                 

            data::data05(rsp1.as_mut_slice(),rsp2.as_mut_slice(),rsp3.as_mut_slice(),rsp4.as_mut_slice(),rsp5.as_mut_slice(),rsp6.as_mut_slice(),rsp7.as_mut_slice(),rsp9.as_mut_slice(),rsp10.as_mut_slice(),rsp11.as_mut_slice(),rsp12.as_mut_slice(),rsp14.as_mut_slice(),rsp15.as_mut_slice(),rsp16.as_mut_slice(),rsp17.as_mut_slice(),rsp18.as_mut_slice(),rsp19.as_mut_slice(),rsp20.as_mut_slice());

             }


             if matches.is_present("csv-live-data-direct" ) {
                 let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3101, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3102, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp4 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3103, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp5 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3104, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp6 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3105, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp7 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3106, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp8 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3106, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp9 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3107, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp10 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310C, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp11 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310D, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp12 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310E, 1)).await.expect("Connection timeout").unwrap();
                 //let mut rsp13 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310E, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp14 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310F, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp15 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3110, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp16 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3111, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp17 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3112, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp18 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp19 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311B, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp20 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.expect("Connection timeout").unwrap();


            data::data06(&dt,rsp1.as_mut_slice(),rsp2.as_mut_slice(),rsp3.as_mut_slice(),rsp4.as_mut_slice(),rsp5.as_mut_slice(),rsp6.as_mut_slice(),rsp7.as_mut_slice(),rsp9.as_mut_slice(),rsp10.as_mut_slice(),rsp11.as_mut_slice(),rsp12.as_mut_slice(),rsp14.as_mut_slice(),rsp15.as_mut_slice(),rsp16.as_mut_slice(),rsp17.as_mut_slice(),rsp18.as_mut_slice(),rsp19.as_mut_slice(),rsp20.as_mut_slice());

             }

             if matches.is_present("csv-header-live-data-direct" ) {

                 let mut rsp1 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3100, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp2 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3101, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp3 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3102, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp4 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3103, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp5 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3104, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp6 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3105, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp7 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3106, 1)).await.expect("Connection timeout").unwrap();
               //  let mut rsp8 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3106, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp9 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3107, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp10 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310C, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp11 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310D, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp12 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310E, 1)).await.expect("Connection timeout").unwrap();
                 //let mut rsp13 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310E, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp14 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x310F, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp15 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3110, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp16 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3111, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp17 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x3112, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp18 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311A, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp19 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311B, 1)).await.expect("Connection timeout").unwrap();
                 let mut rsp20 = timeout(Duration::from_secs(5),ctx.read_input_registers(0x311D, 1)).await.expect("Connection timeout").unwrap();

                 data::data07(&dt,rsp1.as_mut_slice(),rsp2.as_mut_slice(),rsp3.as_mut_slice(),rsp4.as_mut_slice(),rsp5.as_mut_slice(),rsp6.as_mut_slice(),rsp7.as_mut_slice(),rsp9.as_mut_slice(),rsp10.as_mut_slice(),rsp11.as_mut_slice(),rsp12.as_mut_slice(),rsp14.as_mut_slice(),rsp15.as_mut_slice(),rsp16.as_mut_slice(),rsp17.as_mut_slice(),rsp18.as_mut_slice(),rsp19.as_mut_slice(),rsp20.as_mut_slice());

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
