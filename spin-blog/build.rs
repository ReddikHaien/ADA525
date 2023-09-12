use std::{env, path::Path, fs::{self, File, OpenOptions}};
use std::io::Write;

fn main(){
    println!("cargo:rerun-if-changed=posts/");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("information.rs");

    let paths = get_post_names();

    let mut file = File::create(&dest_path).unwrap();

    writeln!(file, "pub fn posts() -> &'static [(&'static str, &'static str)]{{ &[").unwrap();
    for path in paths{
        writeln!(file, "{},",path).unwrap();
    }

    writeln!(file, "]}}").unwrap();
}

fn get_post_names() -> Vec<String>{
    let dir = fs::read_dir("./posts").unwrap();
    let mut out = Vec::new();
    for entry in dir{
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file(){
            

            let markdown = fs::read_to_string(entry.path()).unwrap();

            let mut lines = markdown.lines().skip(1);

            let mut attributes = Vec::new();

            loop{
                let line = lines.next().unwrap();
                if line == "---"{
                    break;
                }

                let parts = line.trim().splitn(2,':').take(2).collect::<Vec<_>>();
                let key = parts[0].trim();
                let value = parts[1].trim();
                attributes.push((key, value));
            }

            let name = entry.file_name();
            
            let name = &name.to_string_lossy()[..(name.len()-3)].to_string();

            out.push(format!("(\"{}\", {})",name, attributes.iter().find(|(k,_)| (*k).eq("title")).map(|(_,n)| n).unwrap()));

        }
    }
    out
}