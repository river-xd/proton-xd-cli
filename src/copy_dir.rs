use tokio::*;


use std::{
  collections::LinkedList,
  path::Path
};




/// Copies a directory to specified `dest` path recursively. (actually iteratively)
/// 
/// `exceptions` is actually a regex expression that decides whether a file/directory should be copied.
/// # Example
/// ```rs
/// copy_dir_all("./test/repo","./test/xd",".git*").await?;
/// ```
#[allow(unused)]
pub async fn copy_dir_all<F: AsRef<Path>,T: AsRef<Path>>(from: F,to: T,exceptions: &str)-> io::Result<()> {
  let except=regex::Regex::new(exceptions).unwrap();
  let mut queue=LinkedList::from_iter([(from.as_ref().to_owned(),to.as_ref().to_owned())]);

  while let Some((src,dest))=queue.pop_front() {
    //ensuring destination path
    fs::create_dir_all(&dest).await?;
    let mut iter=fs::read_dir(src).await?;

    while let Some(entry)=iter.next_entry().await? {
      // skipping exceptions.
      if except.is_match(entry.path().to_str().unwrap_or_default()) {
        continue;
      }

      let entry_type=entry.file_type().await?;
      let entry_dest_path=dest.join(entry.file_name());

      // NOTE: It doesn't care about symlinks.
      match entry_type.is_file() {
        true=> {
          tokio::spawn(fs::copy(entry.path(),entry_dest_path)).await??;
        },
        _=> queue.push_back((entry.path(),entry_dest_path))
      }
    }
  }

  Ok(())
}




#[cfg(test)]
mod tests {
  use tokio::{test, fs};
  #[test]
  async fn xd() {
    fs::create_dir_all("./test/xd").await.unwrap();

    super::copy_dir_all("./test/repo","./test/xd",".git*").await.unwrap()
  }

  #[test]
  async fn resource() {
    let temp=std::env::temp_dir();
    fs::write(temp.join("./xd.ts"),"console.log(\"xd\");").await.unwrap();

    println!("{}",temp.display())
  }
}

