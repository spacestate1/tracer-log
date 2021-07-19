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

//pub fn data04(rsp: &mut [u16], rsp2: &mut [u16],rsp3: &mut [u16],) {
    pub fn data04(rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16],rsp4: &mut [u16],rsp5: &mut [u16],rsp6: &mut [u16],rsp7: &mut [u16],rsp8: &mut [u16],rsp9: &mut [u16],rsp10: &mut [u16],rsp11: &mut [u16],rsp12: &mut [u16],rsp13: &mut [u16],rsp14: &mut [u16],rsp15: &mut [u16],rsp16: &mut [u16],rsp17: &mut [u16],rsp18: &mut [u16],rsp19: &mut [u16],rsp20: &mut [u16],rsp21: &mut [u16],rsp22: &mut [u16],rsp23: &mut [u16],rsp24: &mut [u16],rsp25: &mut [u16],rsp26: &mut [u16],rsp27: &mut [u16],rsp28: &mut [u16],rsp29: &mut [u16],rsp30: &mut [u16],rsp31: &mut [u16],rsp32: &mut [u16],rsp33: &mut [u16],rsp34: &mut [u16],rsp35: &mut [u16],rsp36: &mut [u16]) {
//pub fn data04(rsp2: &mut [u16], rsp3: &mut [u16]) {
//pub fn data04(rsp: &mut [u16]) {


        println!("Maximum input volt (PV) today: {:?}",0.01 * rsp1[0] as f32);
     println!("Minimum input volt (PV) today: {:?}",0.01 * rsp2[0] as f32);
     println!("Maximum battery volt today{:?}", 0.01 * rsp3[0] as f32);
     println!("Minimum battery volt today {:?}", 0.01 * rsp4[0] as f32);

     println!("Consumed energy today: {:?}", 0.01 * rsp5[0] as f32);
     println!("Consumed energy today low: {:?}", 0.01 * rsp6[0] as f32);
     println!("Consumed energy today high: {:?}", 0.01 * rsp7[0] as f32);
     println!("Consumed energy this month: {:?}", 0.01 * rsp8[0] as f32);
     println!("Consumed energy this month low:{:?}", 0.01 * rsp9[0] as f32);
     println!("Consumed energy this month high:{:?}", 0.01 * rsp10[0] as f32);
     println!("Consumed energy this year{:?}", 0.01 * rsp11[0] as f32);
     println!("Consumed energy this year low: {:?}", 0.01 * rsp12[0] as f32);
     println!("Consumed energy this year high: {:?}", 0.01 * rsp13[0] as f32);
     println!("Total consumed energy: {:?}", 0.01 * rsp14[0] as f32);
     println!("Total consumed energy low: {:?}", 0.01 * rsp15[0] as f32);
     println!("Total consumed energy high:{:?}", 0.01 * rsp16[0] as f32);
     println!("Generated energy today: {:?}", 0.01 * rsp17[0] as f32);
     println!("Generated energy today Low: {:?}", 0.01 * rsp18[0] as f32);
     println!("Generated energy today high:{:?}", 0.01 * rsp19[0] as f32);
     println!("Generated energy this month: {:?}", 0.01 * rsp20[0] as f32);
     println!("Generated energy this month Low: {:?}", 0.01 * rsp21[0] as f32);
     println!("Generated energy this month High: {:?}", 0.01 * rsp22[0] as f32);
     println!("Generated energy this year: {:?}", 0.01 * rsp23[0] as f32);
     println!("Generated energy this year Low: {:?}", 0.01 * rsp24[0] as f32);
     println!("Generated energy this year High: {:?}", 0.01 * rsp25[0] as f32);
     println!("Total generated energy: {:?}", 0.01 * rsp26[0] as f32);
     println!("Total generated energy Low: {:?}", 0.01 * rsp27[0] as f32);
     println!("Total Generated energy High: {:?}", 0.01 * rsp28[0] as f32);
     println!("Carbon dioxide reduction: {:?}", 0.01 * rsp29[0] as f32);
     println!("Carbon dioxide reduction Low: {:?}", 0.01 * rsp30[0] as f32);
     println!("Carbon dioxide reduction High: {:?}", 0.01 * rsp31[0] as f32);
     println!("Battery Current: {:?}", 0.01 * rsp32[0] as f32);
     println!("Battery Current Low: {:?}", 0.01 * rsp33[0] as f32);
     println!("Battery Current High: {:?}", 0.01 * rsp34[0] as f32);
     println!("Battery Temp: {:?}", 0.01 * rsp35[0] as f32);
     println!("Ambient Temp: {:?}", 0.01 * rsp36[0] as f32);
     
     }


    pub fn data05(rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16],rsp4: &mut [u16],rsp5: &mut [u16],rsp6: &mut [u16],rsp7: &mut [u16],rsp9: &mut [u16],rsp10: &mut [u16],rsp11: &mut [u16],rsp12: &mut [u16],rsp14: &mut [u16],rsp15: &mut [u16],rsp16: &mut [u16],rsp17: &mut [u16],rsp18: &mut [u16],rsp19: &mut [u16],rsp20: &mut [u16]) {

        println!("Solar charge controller--PV array voltage: {:?}",0.01 * rsp1[0] as f32);
        println!("Solar charge controller--PV array current: {:?}",0.01 * rsp2[0] as f32);
        println!("Solar charge controller--PV array power: {:?}",0.01 * rsp3[0] as f32);
        println!("Charging equipment input power High: {:?}",0.01 * rsp4[0] as f32);
        println!("Charging equipment output voltage: {:?}",0.01 * rsp5[0] as f32);
        println!("Charging equipment output current: {:?}",0.01 * rsp6[0] as f32);
        println!("Charging equipment output power: {:?}",0.01 * rsp7[0] as f32);
        println!("Charging equipment output power High: {:?}",0.01 * rsp9[0] as f32);
        println!("Discharging equipment output voltage: {:?}",0.01 * rsp10[0] as f32);
        println!("Discharging equipment output current: {:?}",0.01 * rsp11[0] as f32);
        println!("Discharging equipment output power: {:?}",0.01 * rsp12[0] as f32);
        println!("Discharging equipment output power High: {:?}",0.01 * rsp14[0] as f32);
        println!("Battery Temperature: {:?}",0.01 * rsp15[0] as f32);
        println!("Temperature inside equipment: {:?}",0.01 * rsp16[0] as f32);
        println!("Power components temperature( heat Sink): {:?}",0.01 * rsp17[0] as f32);
        println!("Battery SOC: {:?}",rsp18[0] as f32);
        println!("Remote battery temperature: {:?}",0.01 * rsp19[0] as f32);
        println!("Current system rated voltage. 1200, 2400 represent 12V, 24V: {:?}",0.01 * rsp20[0] as f32);



    }

pub fn data06(dt: &str,rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16],rsp4: &mut [u16],rsp5: &mut [u16],rsp6: &mut [u16],rsp7: &mut [u16],rsp9: &mut [u16],rsp10: &mut [u16],rsp11: &mut [u16],rsp12: &mut [u16],rsp14: &mut [u16],rsp15: &mut [u16],rsp16: &mut [u16],rsp17: &mut [u16],rsp18: &mut [u16],rsp19: &mut [u16],rsp20: &mut [u16]) {


       println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,0.01 * rsp1[0] as f32, 0.01 * rsp2[0] as f32, 0.01 * rsp3[0] as f32,0.01 * rsp4[0] as f32,0.01 * rsp5[0] as f32, 0.01 * rsp6[0] as f32,0.01 * rsp7[0] as f32,0.01 * rsp9[0] as f32,0.01 * rsp10[0] as f32,0.01 * rsp11[0] as f32,0.01 * rsp12[0] as f32,0.01 * rsp14[0] as f32,0.01 * rsp15[0] as f32,0.01 * rsp16[0] as f32,0.01 * rsp17[0] as f32, rsp18[0] as f32, 0.01 * rsp19[0] as f32, 0.01 * rsp20[0] as f32);

}

pub fn data07(dt: &str,rsp1: &mut [u16],rsp2: &mut [u16],rsp3: &mut [u16],rsp4: &mut [u16],rsp5: &mut [u16],rsp6: &mut [u16],rsp7: &mut [u16],rsp9: &mut [u16],rsp10: &mut [u16],rsp11: &mut [u16],rsp12: &mut [u16],rsp14: &mut [u16],rsp15: &mut [u16],rsp16: &mut [u16],rsp17: &mut [u16],rsp18: &mut [u16],rsp19: &mut [u16],rsp20: &mut [u16]) {

    println!("pv_array_voltage,pv_array_current,pv_array_power,charging_equipment_input_power_high,charging_equipment_output_voltage,charging_equipment_output_current,charging_equipment_output_power,charging_equipment_output_power_high,discharging_equipment_output_voltage,discharging_equipment_output_current,discharging_equipment_output_power,discharging_equipment_output_power_high,battery_temp,temperature_inside_equipment,heat_sink_temp,battery_soc,remote_battery_temperature,current_system_rated_voltage");


       println!("{},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",dt,0.01 * rsp1[0] as f32, 0.01 * rsp2[0] as f32, 0.01 * rsp3[0] as f32,0.01 * rsp4[0] as f32,0.01 * rsp5[0] as f32, 0.01 * rsp6[0] as f32,0.01 * rsp7[0] as f32,0.01 * rsp9[0] as f32,0.01 * rsp10[0] as f32,0.01 * rsp11[0] as f32,0.01 * rsp12[0] as f32,0.01 * rsp14[0] as f32,0.01 * rsp15[0] as f32,0.01 * rsp16[0] as f32,0.01 * rsp17[0] as f32, rsp18[0] as f32, 0.01 * rsp19[0] as f32, 0.01 * rsp20[0] as f32);

}

