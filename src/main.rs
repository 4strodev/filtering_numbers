use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_to_file = &args[1];
    let output_file = &args[2];

    let content = fs::read_to_string(path_to_file)
        .expect("Unable to read file")
        .trim()
        .to_string();

    let lines = get_lines(&content);

    // Filtering numbers
    let mut filtered_numbers: Vec<String> = Vec::new();
    for line in lines {
        let vec_line: Vec<String> = line.trim().split(" ").map(|s| s.to_string()).collect();
        filtered_numbers.append(
            &mut vec_line
                .into_iter()
                .filter(|str_num| str_num.parse::<f32>().unwrap() % 2.0 == 0.0)
                .collect(),
        );
    }

    // Writing filtered numbers
    write_file(&output_file, filtered_numbers);
}

fn write_file(file_name: &str,data: Vec<String>) {
    let mut out_file = fs::File::create(file_name).expect("Unable to create file");

    for chunk in data.chunks(10) {
        let line = format_chunk(chunk.to_vec());
        out_file.write_all(format!("{}\n", line).as_bytes()).expect("Unable to write file");
    }
}

fn format_chunk(chunk: Vec<String>) -> String {
    chunk
        .into_iter()
        .reduce(|acum: String, element: String| {
            format!("{} {}", &acum, &element)
        })
        .unwrap()
}

fn get_lines(content: &String) -> Vec<String> {
    let lines = content.split("\n").map(|s| s.to_string()).collect();

    return lines;
}
