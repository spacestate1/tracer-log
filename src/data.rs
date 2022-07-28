use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;

pub fn data01(dt: &str,rsp: &mut [u16], rsp2: &mut [u16],rsp3: &mut [u16],) {

            println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,rsp[0] as f32, 0.01 * rsp[1] as f32, 0.01 * rsp2[0] as f32,0.01 * rsp3[16] as f32,0.01 * rsp3[17] as f32, 0.01 * rsp3[18] as f32,0.01 * rsp3[12] as f32,0.01 * rsp3[13] as f32,0.01 * rsp3[14] as f32,0.01 * rsp3[2] as f32,0.01 * rsp3[1] as f32,0.01 * rsp3[0] as f32,0.01 * rsp3[4] as f32,0.01 * rsp3[5] as f32,0.01 * rsp3[6] as f32);
}
            pub fn data02(dt: &str,rsp: &mut [u16], rsp2: &mut [u16],rsp3: &mut [u16],) {
            println!("Time,BatterySOC,Remote_battery_temperature,battery_real_rated_power,battery_temp,temperature_inside_equipment,power_components_temperature,Discharging_equipment_output_voltage,Discharging_equipment_output_current,Discharging_equipment_output_power,Charging_equipment_input_power,Charging_equipment_input_current,Charging_equipment_input_voltage,Charging_equipment_output_voltage,Charging_equipment_output_current,Charging_equipment_output_power");
            println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,rsp[0] as f32, 0.01 * rsp[1] as f32, 0.01 * rsp2[0] as f32,0.01 * rsp3[16] as f32,0.01 * rsp3[17] as f32, 0.01 * rsp3[18] as f32,0.01 * rsp3[12] as f32,0.01 * rsp3[13] as f32,0.01 * rsp3[14] as f32,0.01 * rsp3[2] as f32,0.01 * rsp3[1] as f32,0.01 * rsp3[0] as f32,0.01 * rsp3[4] as f32,0.01 * rsp3[5] as f32,0.01 * rsp3[6] as f32);
            }

pub fn data03(rsp: &mut [u16], rsp2: &mut [u16],rsp3: &mut [u16],) {

        println!("Basic EPSolar Tracer system info eLog01:");
        println!("================================");
        println!("Battery SOC: {:?}", rsp[0] as f32);
        println!("Remote battery temperature: {:?}",0.01 * rsp[1] as f32);
        println!("================================");
        println!("Battery's real rated power: {:?}", 0.01 *rsp2[0] as f32);
        println!("================================");
        println!("Battery Temperature: {:?}",0.01 * rsp3[16] as f32);
        println!("Temperature inside equipment: {:?}",0.01 * rsp3[17] as f32);
        println!("Power components temperature: {:?}",0.01 * rsp3[18] as f32);
        println!("================================");
        println!("Discharging equipment output voltage:: {:?}",0.01 * rsp3[12] as f32);
        println!("Discharging equipment output current: {:?}",0.01 * rsp3[13] as f32);
        println!("Discharging equipment output power: {:?}",0.01 * rsp3[14] as f32);
        println!("================================");
        println!("Charging equipment input power:{:?}",0.01 * rsp3[2] as f32);
        println!("Charging equipment input current: {:?}",0.01 * rsp3[1] as f32);
        println!("Charging equipment input voltage: {:?}",0.01 * rsp3[0] as f32);
        println!("================================");
        println!("Charging equipment output voltage: {:?}",0.01 * rsp3[4] as f32);
        println!("Charging equipment output current: {:?}",0.01 * rsp3[5] as f32);
        println!("Charging equipment output power: {:?}",0.01 * rsp3[6] as f32);



}


    pub fn data05(rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16])
                  {

    println!("PV array voltage: {:?}v", 0.01 * rsp1[0] as f32);
    println!("PV array current: {:?}a", 0.01 * rsp1[1] as f32);
    println!("PV array power: {:?}w", 0.01 * rsp1[2] as f32);
    println!("Charging equipment input power High: {:?}w", 0.01 * rsp1[3] as f32);
    println!("Charging equipment output voltage: {:?}v", 0.01 * rsp1[4] as f32);
    println!("Charging equipment output current: {:?}a", 0.01 * rsp1[5] as f32);
    println!("Charging equipment output power: {:?}w", 0.01 * rsp1[16] as f32);
    println!("Charging equipment output power High: {:?}w", 0.01 * rsp1[7] as f32);
    println!("Discharging equipment output voltage: {:?}v", 0.01 * rsp1[8] as f32);
    println!("Discharging equipment output current: {:?}a", 0.01 * rsp1[9] as f32);
    println!("Discharging equipment output power: {:?}w", 0.01 * rsp1[10] as f32);
    println!("Discharging equipment output power High: {:?}w", 0.01 * rsp1[11] as f32);
    println!("Battery Temperature: {:?}c", 0.01 * rsp1[16] as f32);
    println!("Temperature inside equipment: {:?}c", 0.01 * rsp1[17] as f32);
    println!("Power components temperature( heat Sink): {:?}c", 0.01 * rsp1[18] as f32);
    println!("Battery SOC: {:?} percent",rsp2[0] as f32);
    println!("Remote battery temperature: {:?}c", 0.01 * rsp1[19] as f32);
    println!("Current system rated voltage. 1200, 2400 represent 12V, 24V:{:?}v", 0.01 * rsp3[0] as f32);

    }


    pub fn data06(dt: &str,rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16])
     {

        println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,0.01 * rsp1[0] as f32, 0.01 * rsp1[1] as f32, 0.01 * rsp1[2] as f32,0.01 * rsp1[3] as f32,0.01 * rsp1[4] as f32, 0.01 * rsp1[5] as f32,0.01 * rsp1[6] as f32,0.01 * rsp1[7] as f32,0.01 * rsp1[8] as f32,0.01 * rsp1[9] as f32,0.01 * rsp1[10] as f32,0.01 * rsp1[11] as f32,0.01 * rsp1[16] as f32,0.01 * rsp1[17] as f32,0.01 * rsp1[18] as f32, rsp2[0] as f32, 0.01 * rsp1[19] as f32, 0.01 * rsp3[0] as f32);

}

pub fn data07(dt: &str,rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16])
     {
        println!("timestamp,pv_array_voltage,pv_array_current,pv_array_power,charging_equipment_input_power_high,charging_equipment_output_voltage,charging_equipment_output_current,charging_equipment_output_power,charging_equipment_output_power_high,discharging_equipment_output_voltage,discharging_equipment_output_current,discharging_equipment_output_power,discharging_equipment_output_power_high,battery_temp,temperature_inside_equipment,heat_sink_temp,battery_soc,remote_battery_temperature,current_system_rated_voltage");


        println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,0.01 * rsp1[0] as f32, 0.01 * rsp1[1] as f32, 0.01 * rsp1[2] as f32,0.01 * rsp1[3] as f32,0.01 * rsp1[4] as f32, 0.01 * rsp1[5] as f32,0.01 * rsp1[6] as f32,0.01 * rsp1[7] as f32,0.01 * rsp1[8] as f32,0.01 * rsp1[9] as f32,0.01 * rsp1[10] as f32,0.01 * rsp1[11] as f32,0.01 * rsp1[16] as f32,0.01 * rsp1[17] as f32,0.01 * rsp1[18] as f32, rsp2[0] as f32, 0.01 * rsp1[19] as f32, 0.01 * rsp3[0] as f32);
}



    pub fn data08(dt: &str,rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16])
     {
         let xdata = format!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}\n",dt,0.01 * rsp1[0] as f32, 0.01 * rsp1[1] as f32, 0.01 * rsp1[2] as f32,0.01 * rsp1[3] as f32,0.01 * rsp1[4] as f32, 0.01 * rsp1[5] as f32,0.01 * rsp1[6] as f32,0.01 * rsp1[7] as f32,0.01 * rsp1[8] as f32,0.01 * rsp1[9] as f32,0.01 * rsp1[10] as f32,0.01 * rsp1[11] as f32,0.01 * rsp1[16] as f32,0.01 * rsp1[17] as f32,0.01 * rsp1[18] as f32, rsp2[0] as f32, 0.01 * rsp1[19] as f32, 0.01 * rsp3[0] as f32);
    let path = "data01";
     let rs;
    rs = Path::new(path).exists();
    if rs == false{
        File::create(path).ok();
         }
    else {
        let data_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("unable to open file");
        let mut write_file = BufWriter::new(data_file);
        write!(write_file, "{}", xdata).ok();
        }
    }
