# tracer-log
Lightweight program to record data from TEpsolar/Epever Tracer BN MPPT solar charge controllers. This works with a USB to RS485 adapter to read and record data as a CSV file or easy to read data points. Output is suitable for a database import or processing with other programs.  Also has options to extract data from an attached eLOG01 reader.  

Install instructions: 

1. git clone https://github.com/spacestate1/tracer-log.git
2. cd tracer-log
3. cargo build --release 
4. sudo ./target/release/tracer-log -h

#D3 dashboard. 
Simply put the index.html and the d3.v4.js files into a web hosting directory and point tracer-log's output to the same directory. 
The datafile's name is 711_2022.csv by default but can be changed to anything else in index.html.
Example: 
Apache's default web dir is /var/www/html
![d3-dashboard](https://user-images.githubusercontent.com/7908850/181916472-9f50b997-98f7-4d35-886b-6217487992b0.jpg)
