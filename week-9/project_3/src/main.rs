use std::fs::File;
use std::io::Write;

fn main() {

    let commissioners = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye",];

    let ministries = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum",];

    let zones = vec!["South West","North East","South South","South West","South East",];

    let mut file = File::create("efcc_convicted_ministers.csv").expect("Failed to create destination CSV file");


      println!(" EFCC CONVICTED MINISTERS DATA MERGE (CSV) ");
    
    for i in 0..commissioners.len() {

        let record = format!("{},{},{},{}\n",i + 1,commissioners[i],ministries[i],zones[i]);

        print!("{}", record);

        file.write_all(record.as_bytes()).expect("Failed to write record to CSV file");
    }

       println!("Data merge complete! Merged records saved to 'efcc_convicted_ministers.csv'.");
}
