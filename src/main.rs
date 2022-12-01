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

    /* figure out elf with max calories (part 1 answer) */
    //let max_index = elfs.iter().enumerate().max_by_key(|val| val.1).unwrap();
    //println!("{:?}", max_index);

    /* figure out top three elfs by calories (part 2 answer) */
    let mut top = [0; 3];

    fn push_max(v: i32, s: &mut [i32]) {
        match s {
            [] => return,
            [x] => {
                if v > *x {
                    *x = v;
                }
            },
            _ => {
                if v > s[0] {
                    push_max(s[0], &mut s[1..]);
                    s[0] = v;
                } else {
                    push_max(v, &mut s[1..]);
                }
            }
        }
    }

    for elf in elfs {
        push_max(elf, &mut top);
    }
    println!("{:?}", top);
    
    // overcomplicate this!
    let sum = top.iter().fold(0, |sum, elf| sum + elf);
    println!("{}", sum);

    Ok(())
}
