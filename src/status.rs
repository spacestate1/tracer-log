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

pub fn dstatus01(dt: &str,rsp1: &mut [u16],rsp2: &mut [u16]) {

    println!("Datetime:{}", dt); 

        println!("D3-D0: 01H Overvolt \n 00H Normal  \n 02H Under Volt\n 03H Low Volt Disconnect\n 04H Fault D7-D4: 00H Normal\n 01H Over Temp.(Higher than the warning settings)\n 02H Low Temp.( Lower than the warning settings)\n D8: Battery inerternal resistance abnormal 1\n normal 0 D15: 1-Wrong identification for rated voltage\n ======\n Value: {:?}",rsp1[0] as f32);
        println!("\nD15-D14: Input volt status\n 00 normal\n 01 no power connected\n 02H Higher volt input, 03H Input volt error\n D13: Charging MOSFET is short\n D12: Charging or Anti-reverse MOSFET is short\n D11: Anti-reverse MOSFET is short\n D10: Input is over current\n D9: The load is Over current\n D8: The load is short\n D7: Load MOSFET is short\n D4: PV Input is short\n D3-2: Charging status\n 00 No charging\n01 Float\n02 Boost\n03 Equlization. D1: 0 Normal\n 1 Fault. D0: 1 Running\n 0 Standby\n======\n Value: {:?}",rsp2[0] as f32);

} 


