#![allow(unused_imports)]
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::RwLock;
use std::fs::{read_dir, read_to_string};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde_json::Value;
use tracing;
//use rayon::iter::{IntoParallelRefIterator, ParallelIterator};


fn do_work(dir: &Path, mut keys: RwLock<HashSet<String>>) -> anyhow::Result<()> {
    let paths: Vec<PathBuf> = read_dir(dir)?
        .map(|f| f.unwrap().path())
        .collect();
    let r = paths.into_par_iter()
        .progress()
        .for_each(|path|
        match parse(path, &keys) {
            Ok(_) => (),
            Err(e) => tracing::error!("fuck"),
        }
    );
    Ok(())
}

fn parse(path: PathBuf, keys: &RwLock<HashSet<String>>) -> anyhow::Result<Box<Value>, anyhow::Error> {
    let s = read_to_string(path)?;
    let json: Value = serde_json::from_str(&s)?;
    let test = HashSet::<String>::from_iter(vec!("a".to_string(),"b".to_string()).into_iter());
    if !json.is_object() {
        return Err(anyhow!("not a map"))
    }
    let obj = json.as_object().unwrap().keys();
    //let new_keys = HashSet::<String>::from_iter();

    //Ok(Box(new_keys))
    return Err(anyhow!("not a map"))
}

    /* let res: Result<Value, Box<std::Error>> = read_to_string(path)
         .map_err(Into::into)
         .and_then(|s| serde_json::from_str(&s))
         .map_err(Into::into)
         .and_then(|j| if j.is_object() {
             Ok(HashSet::from_iter(j.as_object().unwrap().keys()))
         } else {
             Err(anyhow!("not a map"))
         })
         .and_then(|k| if !k.is_subset(keys.read().unwrap()) {
             keys.write().unwrap().extend(k);
         });
    };
    match res {
      Ok(d) => Ok(todo!()),
      Err(e) => return Err(e)
    }*/
    //Ok(())

fn main() {
    println!("Hello, world!");
    let mut keys: HashSet<String> = HashSet::new();
    let mut locked_set = RwLock::new(keys);
    let dir = Path::new("./metadata");
    do_work(&dir, locked_set);
}
