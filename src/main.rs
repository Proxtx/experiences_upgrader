use std::{
    env,
    fs::{read_dir, OpenOptions},
    io::{Read, Write},
};

use serde_json::Value;

fn main() {
    let path = env::args().nth(1).unwrap();
    let experiences = read_dir(&path).expect("Can't read dir");
    for file in experiences {
        let file = file.expect("Unable to read dir entry");
        let experience_path = file.path();
        let mut str = String::new();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open(&experience_path)
            .unwrap();
        file.read_to_string(&mut str)
            .expect("Cant read file to string");
        let mut exp_json: serde_json::Value =
            serde_json::from_str(&str).expect("unable to parse experience");
        println!("{:?}", exp_json);
        exp_json
            .as_object_mut()
            .unwrap()
            .get_mut("events")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .values_mut()
            .for_each(|v| {
                v.as_array_mut().unwrap().iter_mut().for_each(|v| {
                    let evt = v
                        .as_object_mut()
                        .unwrap()
                        .get_mut("event")
                        .unwrap()
                        .as_object_mut()
                        .unwrap();
                    *evt.get_mut("data").unwrap() =
                        serde_json::from_str::<Value>(evt.get("data").unwrap().as_str().unwrap())
                            .unwrap()
                });
            });
        OpenOptions::new()
            .write(true)
            .append(false)
            .truncate(true)
            .open(&experience_path)
            .unwrap()
            .write_all(serde_json::to_string(&exp_json).unwrap().as_bytes())
            .expect("Unable to write file");
    }
}
