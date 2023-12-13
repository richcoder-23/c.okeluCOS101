use std::io::Write;

fn main(){
    let name = vec!["\n1)Oluchi Mordi\n","2)Adams Aliyu  \n","3)Shania Bolade\n","4)Adekunle Gold\n","5)Blanca Edemoh\n"];
    let matric_no = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let dept = vec!["Accounting","Economics","Computer science","Electrical eng.","Mechanical eng."];
    let level = vec!["300 \n","100 \n","200 \n","200 \n","100 \n"];

    let mut file = std::fs::File::create("project_2.txt").expect("create failed");

        file.write_all("Welcome to PAU SIMS portal!".as_bytes()).expect("write failed");
        
        file.write_all("\nThe information below displays information of several students.".as_bytes()).expect("write failed");


    for i in 0..name.len(){ 
        
        file.write_all(name[i].as_bytes()).expect("write failed");
        file.write_all("Matric no.:".as_bytes()).expect("write failed");
        file.write_all(matric_no[i].as_bytes()).expect("write failed");
        file.write_all("\nDepartment:".as_bytes()).expect("write failed");
        file.write_all(dept[i].as_bytes()).expect("write failed");
        file.write_all("\nLevel:".as_bytes()).expect("write failed");
        file.write_all(level[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
        
        
    }
    println!("\nData written to file.");

    

}