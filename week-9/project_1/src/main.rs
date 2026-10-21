use std::io::Write;

fn main() {

    let header ="Lager,Stout,Non-Alcoholic";
    let row_1 = "\n33 Export,Legend,Maltina";
    let row_2 = "\nDesperados,Turbo King,Amstel Malta";
    let row_3 = "\nGoldberg,Williams,Malta Gold";
    let row_4 = "\nGulder, ,Fayrouz";
    let row_5 = "\nHeineken, , ";
    let row_6 = "\nStar, ,";

    let mut file = std::fs::File::create("project1.csv").expect("create failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row_1.as_bytes()).expect("write failed");
    file.write_all(row_2.as_bytes()).expect("write failed");
    file.write_all(row_3.as_bytes()).expect("write failed");
    file.write_all(row_4.as_bytes()).expect("write failed");
    file.write_all(row_5.as_bytes()).expect("write failed");
    file.write_all(row_6.as_bytes()).expect("write failed");
    println!("\nData written to file.");
}
