/// VCD routines

// Needs access to the symbol table
// don't need close() cos Rust?

use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use std::collections::HashMap;

use procedure::{Time, Value};

pub struct VcdWriter {
    file: Option<File>,
    lut: HashMap<String, String>,
}

impl VcdWriter {

    pub fn new(filename: &str) -> Option<VcdWriter> {
        //open file
        let path = Path::new(filename);
        let display = path.display();

        let mut writer = VcdWriter {
            file: None,
            lut: HashMap::new(),
        };

        // open the path to write
        writer.file = match File::create(&path) {
            Err(why) => {
                    println!("*ERROR* can't open VCD file {}: {}",
                        display, why.description() );
                    None
            },
            Ok(file) => Some(file),
        };

        Some(writer)
    }

    pub fn write_header(&mut self) {
        let header = "$date
   Date text. For example: November 11, 2009.
$end
$version
   Tiny Verilog (Rust)
$end
$comment
   Any comment text.
$end
$timescale 1ps $end
$scope module logic $end
";
        if let Some(ref mut file) = self.file {
            let _ = file.write_all(header.as_bytes());
        }
    }

    pub fn declare_vars(&mut self, vars: &Vec<String>) {
        let mut vcd_id = '#';
        let mut i = 0;
        for var in vars {
            // map the identifier names to a VCD single-letter
            self.lut.insert(var.clone(), vcd_id.to_string());

            // create the definition line
            let line = format!("$var wire 32 {} {} $end\n", vcd_id, var);
            if let Some(ref mut file) = self.file {
                let _ = file.write_all(line.as_bytes());
            }
            vcd_id = char::from(vcd_id as u8 + 1 );
            i += 1;
            if i > 32 {
                println!("*WARNING* Too many identifiers for VCD file");
                return
            }
        }

        // write the turnaround
        let turnaround = "$upscope $end
$enddefinitions $end
$dumpvars
$end\n";
        if let Some(ref mut file) = self.file {
            let _ = file.write_all(turnaround.as_bytes());
        }
    }


    pub fn dump(&mut self, time: Time, vars: &Vec<String>, data: &HashMap<String, Value> ){
        if let Some(ref mut file) = self.file {
            let timestamp = format!("\n#{}\n", time);
            let _ = file.write_all(timestamp.as_bytes());

            for var in vars {
                let vcd_id = self.lut.get(var).unwrap();
                if let Some(value) = data.get(var) {
                    let line = format!("b{:b} {}\n", value, vcd_id); 
                    let _ = file.write_all(line.as_bytes());
                }
            }
        }
    }


}


