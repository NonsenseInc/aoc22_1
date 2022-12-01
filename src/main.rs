use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    /* read stdin till EOF into lines */
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input)?;
    let lines = input.lines();

    /* sum calories in input blocks into elfs */
    let mut elfs = vec![0];
    for line in lines {
        /* on an empty line, create a new elf */
        if line == "" {
            elfs.push(0);
            continue;
        }

        /* add the calories of the line to the last elf */
        *(elfs.last_mut().unwrap()) += line.parse::<i32>().unwrap();
    }

    println!("{:?}", elfs);

    Ok(())
}
