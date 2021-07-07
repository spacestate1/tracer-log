pub fn stats01(dt: &str,ar02:&str,rsp4: &mut [u16],) {
           if ar02 == "-s" {
            println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,rsp4[0] as f32, 0.01 * rsp4[1] as f32, 0.01 * rsp4[2] as f32,0.01 * rsp4[3] as f32,0.01 * rsp4[4] as f32, 0.01 * rsp4[5] as f32,0.01 * rsp4[6] as f32,0.01 * rsp4[7] as f32,0.01 * rsp4[8] as f32,0.01 * rsp4[9] as f32,0.01 * rsp4[10] as f32,0.01 * rsp4[11] as f32,0.01 * rsp4[12] as f32,0.01 * rsp4[13] as f32,0.01 * rsp4[14] as f32, 0.01 * rsp4[15] as f32,0.01 * rsp4[16] as f32,0.01 * rsp4[17] as f32,0.01 * rsp4[18] as f32,0.01 * rsp4[19] as f32,0.01 * rsp4[20] as f32,0.01 * rsp4[21] as f32,0.01 * rsp4[22] as f32,0.01 * rsp4[23] as f32,0.01 * rsp4[24] as f32,0.01 * rsp4[25] as f32,0.01 * rsp4[26] as f32,0.01 * rsp4[27] as f32,0.01 * rsp4[28] as f32,0.01 * rsp4[29] as f32,0.01 * rsp4[30] as f32);
            }

           else if ar02 == "-sh" {
                println!("minimum_input_volt(PV)_today,maximum_battery_volt_today,minimum_battery_volt_today,consumed_energy_today,consumed_energy_this_month,consumed_energy_this_year,total_consumed_energy,generated_energy_today,generated_energy_this_month,generated_energy_this_year,total_generated_energy,carbon_dioxide_reduction,battery_Voltage,battery_Current,battery_Temp,ambient_Temp");

           println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,rsp4[0] as f32, 0.01 * rsp4[1] as f32, 0.01 * rsp4[2] as f32,0.01 * rsp4[3] as f32,0.01 * rsp4[4] as f32, 0.01 * rsp4[5] as f32,0.01 * rsp4[6] as f32,0.01 * rsp4[7] as f32,0.01 * rsp4[8] as f32,0.01 * rsp4[9] as f32,0.01 * rsp4[10] as f32,0.01 * rsp4[11] as f32,0.01 * rsp4[12] as f32,0.01 * rsp4[13] as f32,0.01 * rsp4[14] as f32, 0.01 * rsp4[15] as f32,0.01 * rsp4[16] as f32,0.01 * rsp4[17] as f32,0.01 * rsp4[18] as f32,0.01 * rsp4[19] as f32,0.01 * rsp4[20] as f32,0.01 * rsp4[21] as f32,0.01 * rsp4[22] as f32,0.01 * rsp4[23] as f32,0.01 * rsp4[24] as f32,0.01 * rsp4[25] as f32,0.01 * rsp4[26] as f32,0.01 * rsp4[27] as f32,0.01 * rsp4[28] as f32,0.01 * rsp4[29] as f32,0.01 * rsp4[30] as f32);

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
}
