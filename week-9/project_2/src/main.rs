use std::io::Write;

fn main() {

    let students = vec![
        vec!["Oluchi Mordi", "ACC10211111", "Accounting", "300"],
        vec!["Adams Aliyu", "ECO10110101", "Economics", "100"],
        vec!["Shania Bolade", "CSC10328828", "Computer", "200"],
        vec!["Adekunle Gold", "EEE11020202", "Electrical", "200"],
        vec!["Blanca Edemoh", "MEE10202001", "Mechanical", "100"],
    ];


    let mut file = std::fs::File::create("pau_smis.csv").expect("create failed");
    //did not understand how to do this part so i used AI to learn and do it
    for students in students.iter() {


        let row = format!("{},{},{},{}\n",students[0], students[1], students[2], students[3]);

        file.write_all(row.as_bytes()).expect("write failed");
    
    }

    println!("\nData written to file.");
}
