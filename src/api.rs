
use tokio::*;
use std::path::Path;


use io::Error;

use requestty::{
  Question,
  prompt_one
};

use crossterm::style::{
  Color,
  style,
  Stylize
};



pub(crate) fn confirm(msg: &str,default: bool)-> bool {
  let q=Question::confirm(msg).default(default).build();
  
  match prompt_one(q) {
    Ok(res)=> res.as_bool().unwrap(),
    _=> default
  }
}


pub async fn ensure_fresh_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  let path=path.as_ref();

  if fs::read_dir(path).await?.next_entry().await?.is_none() {
    return Ok(());
  }

  let msg=format!("{}: {path:?} is not an empty directory. Do you want to continue?",style("warning").with(Color::Yellow));
  let prompt=confirm(&msg,false);

  match prompt {
    true=> Ok(()),
    false=> Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{path:?} is not an empty directory").as_str()
    ))
  }
}

pub async fn ensure_dir<P: AsRef<Path>>(path: P)-> io::Result<()> {
  if fs::try_exists(&path).await? {
    return Ok(());
  }
  fs::create_dir_all(path).await
}


pub(crate) fn url(template: &str,ts: bool)-> String {
  format!("https://github.com/proton-xd-templates/{template}-template-{}",lang(ts))
}


fn lang<'a>(ts: bool)-> &'a str {
  match ts {
    true=> "ts",
    false=> "js",
  }
}

pub fn clone_repo<P: AsRef<Path>>(url: &str,into: P)-> io::Result<()> {
  match git2::Repository::clone(url,into) {
    Ok(_)=> Ok(()),
    Err(err)=> Err(Error::from_raw_os_error(err.raw_code())),
  }
}


