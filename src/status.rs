pub fn status01(dt: &str,rsp5: &mut [u16],) {
                let vs01 = rsp5[0];
                let vs02 = vs01 & 0x0007;
                    match vs02 {

                    0 => println!("{},battery_voltage_status:normal", dt), 
                    1 => println!("{},battery_voltage_status:overvolt", dt),
                    2 => println!("{},battery_voltage_status:under-volt", dt),
                    3 => println!("{},battery_voltage_status:low-volt-disconnect", dt),
                    4 => println!("{},battery_voltage_status:fault", dt),
                    _ => {}
                    }
            }


