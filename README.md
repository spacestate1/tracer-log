# tracer-log
Lightweight program to record data from TEpsolar/Epever Tracer BN MPPT solar charge controllers. This works with a USB to RS485 adapter to read and record data as a CSV file or easy to read data points. Output is suitable for a database import or processing with other programs.  This is also has options to extract data from an attached eLOG01.  

Install instructions: 

1. git clone https://github.com/spacestate1/tracer-log.git
2. cd tracer-log
3. cargo build --release 
4. sudo ./target/release/tracer-log -h
