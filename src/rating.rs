pub fn rating01(dt: &str,rsp5: &mut [u16],) {
           println!("datetime: {:?}", dt);  
           println!("charging_equipment_rated_input_voltage: {:?}v", 0.01 * rsp5[0] as f32); 
           println!("charging_equipment_rated_input_current: {:?}a", 0.01 * rsp5[1] as f32);
           println!("charging_equipment_rated_input_power: {:?}w", 0.01 * rsp5[2] as f32);
           println!("charging_equipment_rated_output_voltage: {:?}v", 0.01 * rsp5[4] as f32);
           println!("charging_equipment_rated_output_current: {:?}a", 0.01 * rsp5[5] as f32);
           println!("charging_equipment_rated_output_power: {:?}w", 0.01 * rsp5[6] as f32);
}
