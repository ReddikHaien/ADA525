use std::{env, path::Path, fs::{self, File, OpenOptions}, collections::HashMap};
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
            
            let mut body = String::new();

            for line in lines{
                body.push_str(line);
                body.push('\n');
            }

            let (table, buffer) = compress_string(&body);

            let mut data = String::new();

            data.push_str("{\"table\":{");
            let l = table.len();
            for (i, (key, id)) in table.into_iter().enumerate(){
                data.push('"');
                data.push_str(&key);
                data.push_str("\":");
                data.push_str(&id.to_string());
                if i < l - 1{
                    data.push(',');
                }
            }
            data.push_str("},\"data\":[");
            let l = buffer.len();
            for (i, x) in buffer.into_iter().enumerate(){
                data.push_str(&x.to_string());
                if i < l - 1{
                    data.push(',');
                }
            }

            fs::create_dir_all("./postsrenamed").unwrap();
            fs::write(format!("./postsrenamed/{}.txt", name), body).unwrap();
        }
    }
    out
}

pub fn compress_string(data: &str) -> (HashMap<String, u32>, Vec<u32>){

    let chars = data.chars().collect::<Vec<_>>();

    let mut map = HashMap::new();
    for i in 0..=255{
        unsafe { map.insert( char::from_u32_unchecked(i).to_string(), i as u32); }
    }

    let mut code = 256;

    let mut p = String::new();
    let mut c = '\u{0000}';
    
    p.push(chars[0]);
    
    let mut out = Vec::new();

    for i in 0..chars.len(){
        if i != chars.len()-1{
            c = chars[i+1];
        }
        let mut concat = p.clone();
        concat.push(c);
        if map.contains_key(&concat){
            p.push(c);
        }
        else{
            out.push(*map.get(&p).unwrap());
            map.insert(concat, code);
            code+=1;
            p = String::from(c);
        }
    }

    (map, out)
}