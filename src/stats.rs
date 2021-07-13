pub fn stats01(dt: &str,ar02:&str,rsp4: &mut [u16],) {
           if ar02 == "-s" {

               println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt, 0.01 * rsp4[0] as f32, 0.01 * rsp4[1] as f32,0.01 * rsp4[2] as f32,0.01 * rsp4[3] as f32,0.01 * rsp4[4] as f32,0.01 * rsp4[6] as f32,0.01 * rsp4[8] as f32,0.01 * rsp4[10] as f32,0.01 * rsp4[12] as f32,0.01 * rsp4[14] as f32,0.01 * rsp4[16] as f32,0.01 * rsp4[18] as f32,0.01 * rsp4[26] as f32,rsp4[27] as f32,0.01 * rsp4[29] as f32);

            }

           else if ar02 == "-sh" {
                println!("datetime, max_PV_volt_today,min_PV_volt_today,maximum_battery_volt_today,min_battery_volt_today,consumed_energy_today,consumed_energy_this_month,consumed_energy_this_year,total_consumed_energy,total_generated_energy_today,total_generated_energy_month,total_generated_energy_year,total_generated_energy,battery_voltage,battery_current,battery_temp");

               println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt, 0.01 * rsp4[0] as f32, 0.01 * rsp4[1] as f32,0.01 * rsp4[2] as f32,0.01 * rsp4[3] as f32,0.01 * rsp4[4] as f32,0.01 * rsp4[6] as f32,0.01 * rsp4[8] as f32,0.01 * rsp4[10] as f32,0.01 * rsp4[12] as f32,0.01 * rsp4[14] as f32,0.01 * rsp4[16] as f32,0.01 * rsp4[18] as f32,0.01 * rsp4[26] as f32,rsp4[27] as f32,0.01 * rsp4[29] as f32);

           }


           else if  ar02 == "-sr" {

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
            else {
                println!("Argument not understood");
            }
}
