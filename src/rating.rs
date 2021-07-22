pub fn rating01(dt: &str,rsp5: &mut [u16],) {
           println!("datetime: {:?}", dt);  
           println!("charging_equipment_rated_input_voltage: {:?}v", 0.01 * rsp5[0] as f32); 
           println!("charging_equipment_rated_input_current: {:?}a", 0.01 * rsp5[1] as f32);
           println!("charging_equipment_rated_input_power: {:?}w", 0.01 * rsp5[2] as f32);
           println!("charging_equipment_rated_output_voltage: {:?}v", 0.01 * rsp5[4] as f32);
           println!("charging_equipment_rated_output_current: {:?}a", 0.01 * rsp5[5] as f32);
           println!("charging_equipment_rated_output_power: {:?}w", 0.01 * rsp5[6] as f32);
}


/*pub fn drate01(dt: &str,rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16],rsp4: &mut [u16],rsp5: &mut [u16],rsp6: &mut [u16],rsp7: &mut [u16],rsp8: &mut [u16],rsp9: &mut [u16],rsp10: &mut [u16]) {
           println!("datetime: {:?}", dt);
           println!("PV array rated voltage: {:?}v", 0.01 * rsp1[0] as f32);
           println!("PV array rated current: {:?}a", 0.01 * rsp2[0] as f32);
           println!("PV array rated power: {:?}w", 0.01 * rsp3[0] as f32);
           println!("PV array rated power High: {:?}w", 0.01 * rsp4[0] as f32);
           println!("Battery's voltage: {:?}v", 0.01 * rsp5[0] as f32);
           println!("Rated charging current to battery: {:?}a", 0.01 * rsp6[0] as f32);
           println!("Rated charging power to battery: {:?}w", 0.01 * rsp7[0] as f32);
           println!("Charging equipment rated output power High: {:?}w", 0.01 * rsp8[0] as f32);
           println!("Chargin mode: {:?}", 0.01 * rsp9[0] as f32);
           println!("Rated output current of load: {:?}w", 0.01 * rsp10[0] as f32);
}
*/ 
//TESTING
pub fn drate02(dt: &str,rsp1: &mut [u16]) {
           println!("datetime: {:?}", dt);
           println!("PV array rated voltage: {:?}v", 0.01 * rsp1[0] as f32);
           println!("PV array rated current: {:?}a", 0.01 * rsp1[1] as f32);
           println!("PV array rated power: {:?}w", 0.01 * rsp1[2] as f32);
           println!("PV array rated power High: {:?}w", 0.01 * rsp1[3] as f32);
           println!("Battery's voltage: {:?}v", 0.01 * rsp1[4] as f32);
           println!("Rated charging current to battery: {:?}a", 0.01 * rsp1[5] as f32);
           println!("Rated charging power to battery: {:?}w", 0.01 * rsp1[6] as f32);
           println!("Charging equipment rated output power High: {:?}w", 0.01 * rsp1[7] as f32);
           println!("Chargin mode: {:?}", 0.01 * rsp1[8] as f32);
          // println!("Rated output current of load: {:?}w", 0.01 * rsp1[9] as f32);
}
