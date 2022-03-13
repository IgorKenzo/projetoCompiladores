// use std::collections::HashMap;

// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;

// struct MaqEst {
//     qaceita :i32,
//     qatual: i32,
//     trasitions: HashMap<String, String>
// }

// impl MaqEst {
//     fn new() -> MaqEst {
//         MaqEst {
//             qatual : 0,
//             qaceita: 0,
//             trasitions: HashMap::new()
//         }
//     }

//     fn config(&mut self, file_name: &str) {
//         let v = read_file(file_name);
//         self.qaceita = v[0].parse::<i32>().unwrap();
//         let vc : &[String] = &v[1..];
//         for conf in vc {
//             let pos:Vec<_> = conf.split_whitespace().collect();
//             self.trasitions.insert(format!("{}{}",pos[0],pos[1]), String::from(pos[2]));
//         }
//     }

//     fn run(mut self, input: &str) -> bool {
//         let mut aceita = true;

//         for c in input.chars() {
//             let k = format!("{}{}",self.qatual, if c == ' ' { '_'} else {c});
//             let res = self.trasitions.get(&k);
//             if res != None{
//                 self.qatual = res.unwrap().parse::<i32>().unwrap();
//             }
//             else{
//                 aceita = false;
//                 break;
//             }
//         }

//         if self.qatual != self.qaceita { aceita = false }

//         aceita
//     }
    
// }

// fn read_file(file_name: &str) -> Vec<String>{
//     let mut v : Vec<String> = vec![];
//     // File hosts must exist in current path before this produces output
//     if let Ok(lines) = read_lines(file_name) {
//         // Consumes the iterator, returns an (Optional) String
//         for line in lines {
//             if let Ok(ip) = line {
//                 // println!("{}", ip);
//                 v.push(ip);
//             }
//         }
//     }
    
//     v
// }

// // The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }


// pub fn run(file_name: &str, input: &str) {
//     let mut maq = MaqEst::new();
//     maq.config(file_name);
//     // println!("{:?}", maq.trasitions);
//     println!("{}", maq.run(input));
// }