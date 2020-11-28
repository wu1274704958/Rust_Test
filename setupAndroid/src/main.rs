use std::env::consts::OS;
use envmnt::{exists, set_or_remove};
use std::path::{Path, PathBuf};
use std::fmt;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use toml::Value;
use toml::map::Map;
use toml::value::Value::Table;

const ENV_NAMES:[&'static str; 3] = ["ANDROID_NDK","ANDROID_NDK_HOME","ANDROID_NDK_ROOT"];


fn main() {

    let ndk_path:String;

    if let Some(v) = get_ndk_path()
    {
        println!("find ndk path : {}",v);
        ndk_path = v;
    }else{
        println!("Not find NDK path!");
        return;
    }
    dbg!(OS);
    let platform = "windows-x86_64";
    if OS != "windows"
    {
        println!("Only support windows!");
        return;
    }

    let path = Path::new(&ndk_path).join("toolchains")
        .join("llvm")
        .join("prebuilt")
        .join(platform)
        .join("bin");

    if !path.exists()
    {
        println!("ndk toolchain dir {:?} not found!",path.as_os_str());
    }

    let archs:Vec<String> = vec!["armv7".to_string(),"aarch64".to_string()];

    let prefix_v = map_to_arch_prefix_v(&archs,21);
    let prefix = map_to_arch_prefix(&archs);

    let home_path = dirs::home_dir().unwrap().join(".cargo").join("config");
    dbg!(&home_path);

    let mut cons:String = String::new();
    {
        let f = if !home_path.exists()
        {
            OpenOptions::new().read(true).create(true).open(&home_path)
        } else {
            OpenOptions::new().read(true).open(&home_path)
        };

        if f.is_err() {
            println!("open cargo config failed!");
            return;
        }

        let mut conf = f.unwrap();

        conf.read_to_string(&mut cons);

        fs::remove_file(&home_path);
    }

    let targets = map_to_arch_target_key(&archs);

    let mut value = cons.parse::<toml::Value>().unwrap();
    //let mut src = value.get_mut("source").unwrap();
    let vs = map_to_arch_value(&prefix_v,&prefix,&path);
    let mut target_ = toml::Value::Table(toml::map::Map::new());


    let mut idx = 0;
    for t in &targets{
        if let Table(ref mut tab) = target_ {
            if tab.contains_key(t)
            {
                tab[t] = vs[idx].clone();
            }else{
                tab.insert(t.clone(),vs[idx].clone());
            }
        }
        idx += 1;
    }

    if let Table(ref mut tab) = value {
        if tab.contains_key("target")
        {
            tab[&"target".to_string()] = target_;
        }else{
            tab.insert("target".to_string(),target_);
        }
    }

    let mut f =  OpenOptions::new().append(false).create_new(true).write(true).open(&home_path).unwrap();

    f.write(value.to_string().as_bytes());
}

fn map_to_arch_prefix_v(archs: &Vec<String>,ver_id:i32) -> Vec<String>
{
    let mut res:Vec<String> = vec![];
    archs.iter().for_each(|it|{
        if it == "armv7"
        {
            res.push(fmt::format(format_args!("armv7a-linux-androideabi{}",ver_id)));
        }else if it == "aarch64" {
            res.push(fmt::format(format_args!("aarch64-linux-android{}",ver_id)));
        }
    });
    res
}
fn map_to_arch_prefix(archs: &Vec<String>) -> Vec<String>
{
    let mut res:Vec<String> = vec![];
    archs.iter().for_each(|it|{
        if it == "armv7"
        {
            res.push("arm-linux-androideabi".to_string());
        }else if it == "aarch64" {
            res.push("aarch64-linux-android".to_string());
        }
    });
    res
}

fn map_to_arch_value(arch_prefix_v: &Vec<String>,arch_prefix: &Vec<String>,ndk_path:&PathBuf) -> Vec<Value>
{
    let mut p = ndk_path.to_str().unwrap().to_string();
    p = p.replace("\\","/");
    let mut res:Vec<Value> = vec![];
    let mut i = 0;
    arch_prefix_v.iter().for_each(|it|{
        let mut tab = toml::map::Map::new();

        tab.insert("ar".to_string(), toml::Value::String( fmt::format(format_args!("{}/{}-ar",p,arch_prefix[i])) ));
        tab.insert("linker".to_string(),toml::Value::String( fmt::format(format_args!("{}/{}-clang.cmd",p,it)) ));
        res.push(toml::Value::Table(tab));
        i += 1;
    });
    res
}

fn map_to_arch_target_key(archs: &Vec<String>) -> Vec<String>
{
    let mut res:Vec<String> = vec![];
    archs.iter().for_each(|it|{
        if it == "armv7"
        {
            res.push(fmt::format(format_args!("armv7-linux-androideabi")));
        }else if it == "aarch64" {
            res.push(fmt::format(format_args!("aarch64-linux-android")));
        }
    });
    res
}

fn get_ndk_path()->Option<String> {
    for n in &ENV_NAMES {
        if envmnt::exists(n) {
            return Some(envmnt::get_or_panic(n));
        }
    }
    None
}
