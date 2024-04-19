use std::env;
use std::fs;
mod termsize;
use std::io;

fn get_max_len(vs: &Vec<String>) -> usize {
    vs.iter()
    .map(|p| p.len())
    .max()
    .unwrap_or(0usize)
}

fn str_pad_to(st: &str, output_len: usize) -> String {
    if st.len() >= output_len {
        return st.to_owned();
    }
    let mut output = st.to_owned();
    for _ in 0..(output_len - st.len()) {
        output.push(' ');
    }
    output
}

fn main() -> io::Result<()> {
    let root = env::args()
        .skip(1)
        .take(1)
        .last()
        .unwrap_or(".".to_owned());

    let items : Result<Vec<_>, _> = fs::read_dir(&root)?
        .into_iter()
        .collect();

    let file_names = items?
        .into_iter()
        .map(|p| 
            p.file_name()
            .to_str()
            .unwrap_or("")
            .to_owned()
        )
        .filter(|f| !f.starts_with("."))
        .collect();

    let screen_width = termsize::get_dimensions_out().ws_col;
    let col_size = get_max_len(&file_names) + 2;
    let num_cols = (screen_width as usize) / col_size;
    
    for (i, file_name) in file_names.iter().enumerate() {
        print!("{}", str_pad_to(file_name, col_size));
        if i < file_names.len() && i % num_cols == num_cols - 1 {
            println!("");
        }
    }
    println!("");
    Ok(())
}

