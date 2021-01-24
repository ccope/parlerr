#![allow(unused_imports)]
use std::{borrow::Borrow, collections::HashSet, fmt::Debug, fmt::Display};
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;
use std::fs::{
    read_dir,
    read_to_string,
    write
};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use color_eyre::eyre::{Result, eyre};
use color_eyre::{eyre::Report, eyre::WrapErr, Section};
use indicatif::ParallelProgressIterator;
use parking_lot::RwLock;
use rayon::prelude::*;
use serde_json::Value;
use tracing::{Level, Subscriber, event, instrument, span};
use tracing_subscriber;

//use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Copied from HashSet.is_subset, adapted for refs on left side
fn is_left_ref_subset(left: &HashSet<&String, RandomState>, right: &HashSet<String, RandomState>) -> bool {
    if left.len() <= right.len() { left.iter().all(|v: &&String| right.contains(*v)) } else { false }
}

//fn left_not_in_right<'a>(left: &'a HashSet<&T, S>, right: &'a HashSet<T, S>) -> Vec<&'a T> {
//    *left.filter(|v| !*right.contains(*v)).collect()
//}

#[instrument]
fn get_inputs(dir: &Path) -> Result<Vec<PathBuf>> {
    span!(Level::TRACE, "get_inputs");
    Ok(read_dir(dir)?
        .map(|f| f.unwrap().path())
        .collect())
}

#[instrument]
fn process_inputs(paths: Vec<PathBuf>, keys: &RwLock<HashSet<String>>) {
    span!(Level::TRACE, "iter_paths");
    paths.into_par_iter()
        .progress()
        .for_each(|path|
            match parse(&path, &keys) {
                Ok(_) => (),
                Err(e) => drop(e.wrap_err(format!("Path: {:?}", &path))),
        }
    )
}

#[instrument]
fn parse(path: &PathBuf, keys: &RwLock<HashSet<String>>) -> Result<()> {
    let s = read_to_string(path)?;
    let json: Value = serde_json::from_str(&s)?;
    if !(json.is_array() && json[0].is_object()) {
        event!(Level::TRACE, "Not a map");
        return Err(eyre!("not a map"))
    }
    //let new_keys = HashSet::<String>::from_iter(json.as_object().unwrap().keys().cloned());
    let new_keys = HashSet::<&String>::from_iter(json[0].as_object().unwrap().keys());
    event!(Level::TRACE, "Length of new keys is {}", new_keys.len());
    if is_left_ref_subset(&new_keys, &keys.read()) {
        return Ok(())
    }
    keys.write().extend(new_keys.iter().map(|x| x.to_string()));

    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

fn main() -> Result<()> {
    install_tracing();
    color_eyre::install()?;
    let keys: HashSet<String> = HashSet::new();
    let locked_set = RwLock::new(keys);
    let dir = Path::new("./metadata");
    let paths = get_inputs(&dir)?;
    process_inputs(paths, &locked_set);
    write("result.txt", format!("{:?}", locked_set.read()))?;
    Ok(())
}
