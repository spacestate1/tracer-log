pub fn data01(dt: &str,rsp: &mut [u16], rsp2: &mut [u16],rsp3: &mut [u16],) {

            println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,rsp[0] as f32, 0.01 * rsp[1] as f32, 0.01 * rsp2[0] as f32,0.01 * rsp3[16] as f32,0.01 * rsp3[17] as f32, 0.01 * rsp3[18] as f32,0.01 * rsp3[12] as f32,0.01 * rsp3[13] as f32,0.01 * rsp3[14] as f32,0.01 * rsp3[2] as f32,0.01 * rsp3[1] as f32,0.01 * rsp3[0] as f32,0.01 * rsp3[4] as f32,0.01 * rsp3[5] as f32,0.01 * rsp3[6] as f32);
}
            pub fn data02(dt: &str,rsp: &mut [u16], rsp2: &mut [u16],rsp3: &mut [u16],) {
            println!("Time,BatterySOC,Remote_battery_temperature,battery_real_rated_power,battery_temp,temperature_inside_equipment,power_components_temperature,Discharging_equipment_output_voltage,Discharging_equipment_output_current,Discharging_equipment_output_power,Charging_equipment_input_power,Charging_equipment_input_current,Charging_equipment_input_voltage,Charging_equipment_output_voltage,Charging_equipment_output_current,Charging_equipment_output_power");
            println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,rsp[0] as f32, 0.01 * rsp[1] as f32, 0.01 * rsp2[0] as f32,0.01 * rsp3[16] as f32,0.01 * rsp3[17] as f32, 0.01 * rsp3[18] as f32,0.01 * rsp3[12] as f32,0.01 * rsp3[13] as f32,0.01 * rsp3[14] as f32,0.01 * rsp3[2] as f32,0.01 * rsp3[1] as f32,0.01 * rsp3[0] as f32,0.01 * rsp3[4] as f32,0.01 * rsp3[5] as f32,0.01 * rsp3[6] as f32);
            }

pub fn data03(rsp: &mut [u16], rsp2: &mut [u16],rsp3: &mut [u16],) {

                println!("Basic EPSolar Tracer system info:");
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
