use std::io::Write;

fn main() {
  let a = vec!["Lager        ","Stout        ","Non alcoholic drinks"];
  let b = vec!["33 Export    ","Legend       ","Maltina"];
  let c = vec!["Desperados   ","Turbo King   ","Amstel malta"];
  let d = vec!["Goldberg     ","Williams     ","Malta Gold"];
  let e = vec!["Gulder       ","             ","Fayrouz"];
  let f = vec!["Heineken     "];
  let g = vec!["Star         "];
    
    let mut file = std::fs::File::create("project_1.txt").expect("create failed");
    file.write_all("Welcome to Nigerian Breweries PLC.!\nOur high quality categories of drinks include:".as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("Failed to write into file");
    for index in 0..a.len() {
        file.write_all(a[index].as_bytes()).expect("Failed to write into file");
    }

     file.write_all("\n".as_bytes()).expect("Failed to write into file");
    for index in 0..b.len() {
      file.write_all(b[index].as_bytes()).expect("Failed to write into file");
    }

     file.write_all("\n".as_bytes()).expect("Failed to write into file");
    for index in 0..c.len() {
      file.write_all(c[index].as_bytes()).expect("Failed to write into file");
    }

    file.write_all("\n".as_bytes()).expect("Failed to write into file");
    for index in 0..d.len() {
      file.write_all(d[index].as_bytes()).expect("Failed to write into file");
    }

     file.write_all("\n".as_bytes()).expect("Failed to write into file");
    for index in 0..e.len() {
      file.write_all(e[index].as_bytes()).expect("Failed to write into file");
    }

     file.write_all("\n".as_bytes()).expect("Failed to write into file");
    for index in 0..f.len() {
      file.write_all(f[index].as_bytes()).expect("Failed to write into file");
    }
     file.write_all("\n".as_bytes()).expect("Failed to write into file");
    for index in 0..g.len() {
      file.write_all(g[index].as_bytes()).expect("Failed to write into file");
    }

    println!("Data written to file.");
}