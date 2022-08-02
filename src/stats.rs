use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
pub fn stats01(dt: &str,rsp4: &mut [u16],) {

               println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt, 0.01 * rsp4[0] as f32, 0.01 * rsp4[1] as f32,0.01 * rsp4[2] as f32,0.01 * rsp4[3] as f32,0.01 * rsp4[4] as f32,0.01 * rsp4[6] as f32,0.01 * rsp4[8] as f32,0.01 * rsp4[10] as f32,0.01 * rsp4[12] as f32,0.01 * rsp4[14] as f32,0.01 * rsp4[16] as f32,0.01 * rsp4[18] as f32,0.01 * rsp4[26] as f32,rsp4[27] as f32,0.01 * rsp4[29] as f32);

            }

pub fn stats02(dt: &str,rsp4: &mut [u16],) {
    let sdata = format!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt, 0.01 * rsp4[0] as f32, 0.01 * rsp4[1] as f32,0.01 * rsp4[2] as f32,0.01 * rsp4[3] as f32,0.01 * rsp4[4] as f32,0.01 * rsp4[6] as f32,0.01 * rsp4[8] as f32,0.01 * rsp4[10] as f32,0.01 * rsp4[12] as f32,0.01 * rsp4[14] as f32,0.01 * rsp4[16] as f32,0.01 * rsp4[18] as f32,0.01 * rsp4[26] as f32,rsp4[27] as f32,0.01 * rsp4[29] as f32);

     let rs;
     let now: DateTime<Utc> = Utc::now();
     let spath = format!("solar_stat_data_{}.csv",now.format("%b-%Y"));
     rs = Path::new(&spath).exists();
    if rs == false{
        File::create(&spath).ok();
        let data_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&spath)
        .expect("unable to open file");
        let mut write_file = BufWriter::new(data_file);
        write!(write_file, "datetime, max_PV_volt_today,min_PV_volt_today,maximum_battery_volt_today,min_battery_volt_today,consumed_energy_today,consumed_energy_this_month,consumed_energy_this_year,total_consumed_energy,total_generated_energy_today,total_generated_energy_month,total_generated_energy_year,total_generated_energy,battery_voltage,battery_current,battery_temp\n").ok();
    }
    else {
        let data_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&spath)
        .expect("unable to open file");
        let mut write_file = BufWriter::new(data_file);
        write!(write_file, "{}\n", sdata).ok();
        }

              //  println!("datetime, max_PV_volt_today,min_PV_volt_today,maximum_battery_volt_today,min_battery_volt_today,consumed_energy_today,consumed_energy_this_month,consumed_energy_this_year,total_consumed_energy,total_generated_energy_today,total_generated_energy_month,total_generated_energy_year,total_generated_energy,battery_voltage,battery_current,battery_temp");

               //println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt, 0.01 * rsp4[0] as f32, 0.01 * rsp4[1] as f32,0.01 * rsp4[2] as f32,0.01 * rsp4[3] as f32,0.01 * rsp4[4] as f32,0.01 * rsp4[6] as f32,0.01 * rsp4[8] as f32,0.01 * rsp4[10] as f32,0.01 * rsp4[12] as f32,0.01 * rsp4[14] as f32,0.01 * rsp4[16] as f32,0.01 * rsp4[18] as f32,0.01 * rsp4[26] as f32,rsp4[27] as f32,0.01 * rsp4[29] as f32);

           }

pub fn stats03(rsp4: &mut [u16],) {


           println!("max_PV_volt_today: {:?}v", 0.01 * rsp4[0] as f32); 
           println!("min_PV_volt_today: {:?}v", 0.01 * rsp4[1] as f32);
           println!("maximum_battery_volt_today: {:?}v", 0.01 * rsp4[2] as f32);
           println!("min_battery_volt_today: {:?}v", 0.01 * rsp4[3] as f32);
           println!("consumed_energy_today: {:?}kWh", 0.01 * rsp4[4] as f32);
           println!("consumed_energy_this_month: {:?}kWh",0.01 * rsp4[6] as f32);
           println!("consumed_energy_this_year: {:?}kWh", 0.01 * rsp4[8] as f32);
           println!("total_consumed_energy: {:?}kWh", 0.01 * rsp4[10] as f32);
           println!("total_generated_energy_today: {:?}kWh", 0.01 * rsp4[12] as f32);
           println!("total_generated_energy_month: {:?}kWh", 0.01 * rsp4[14] as f32);
           println!("total_generated_energy_year: {:?}kWh", 0.01 * rsp4[16] as f32);
           println!("total_generated_energy: {:?}kWh", 0.01 * rsp4[18] as f32);
           println!("battery_voltage: {:?}v", 0.01 * rsp4[26] as f32);
           println!("battery_current: {:?}a", rsp4[27] as f32);
           println!("battery_temp: {:?}c", 0.01 * rsp4[29] as f32);


           
}

pub fn stats04(rsp1: &mut [u16],) {

    println!("Max input volt (PV) today: {:?}v",0.01 * rsp1[0] as f32);
    println!("Min input volt (PV) today: {:?}v",0.01 * rsp1[1] as f32);
    println!("Max battery volt today: {:?}v",0.01 * rsp1[12] as f32);
    println!("Min battery volt today: {:?}v",0.01 * rsp1[13] as f32);
    println!("Consumed energy today: {:?}kWh",0.01 * rsp1[4] as f32);
    println!("Consumed energy this month: {:?}kWh",0.01 * rsp1[6] as f32);
    println!("Consumed energy year: {:?}kWh",0.01 * rsp1[8] as f32);
    println!("Total consumed energy: {:?}kWh",0.01 * rsp1[10] as f32);
    println!("Generated energy today: {:?}kWh",0.01 * rsp1[12] as f32);
    println!("Generated energy this month: {:?}kWh",0.01 * rsp1[14] as f32);
    println!("Generated energy this year: {:?}kWh",0.01 * rsp1[16] as f32);
    println!("Total generated energy : {:?}kWh",0.01 * rsp1[18] as f32);
    println!("Battery voltage: {:?}v",0.01 * rsp1[26] as f32);
    println!("Battery temp: {:?}c",0.01 * rsp1[29] as f32);
    
    }
