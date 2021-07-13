pub fn status01(dt: &str,ar02:&str,rsp5: &mut [u16],) {
           if ar02 == "-x" {
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

           else if ar02 == "-xh" {

               println!("PALCEHOLDR"); 


           }


           else if  ar02 == "-xa" {

               println!("{},0:{:?}",dt,rsp5[0]);
               println!("{},1:{:?}",dt,rsp5[1]);
               println!("{},2:{:?}",dt,rsp5[2]);
               println!("{},7:{:?}",dt,rsp5[0007]);
               println!("{},8:{:?}",dt,rsp5[8]);



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
