use std::io::BufRead;
use std::io::Write;

fn main() {

    for entry in std::fs::read_dir("./mandlebrot_txt").unwrap() {
        let path = entry.unwrap().path();
        let file = std::fs::File::open(path.clone()).unwrap();
        let reader = std::io::BufReader::new(file);        
        let mut myimage = Vec::with_capacity(20000);

        _ = write!(myimage,"P3\n");
        _ = write!(myimage, "3000 2000\n");
        _ = write!(myimage, "255\n");

        for line in reader.lines() {
            let line_to_encode = line.unwrap();
            line_to_encode.split(",").for_each(|entry| {
                let value = entry.parse::<f64>().unwrap().min(255.0);
                let ig = (value * (1000.0/255.0) ) as i32;
                let ib = (value * (1000.0/255.0) ) as i32;
                _ = write!(myimage, "{} {} 0\n", ig, ib);
            });
        }
        let complete_path = format!("./mandlebrot_ppm/{}.ppm", path.clone().file_name().unwrap().to_str().unwrap().split(".").collect::<Vec<&str>>()[0]); 
        let mut file = std::fs::File::create(complete_path).unwrap();
        file.write_all(&myimage).unwrap();
    }
}
