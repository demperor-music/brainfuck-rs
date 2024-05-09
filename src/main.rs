use console::Term;
use std::io::{self, stdout, Write};
use std::fs;

fn main() {
    let term = Term::stdout();

    print!(
        "
Scegli cosa fare:
1. Nuovo codice
2. Apri file
> "
    );
    stdout().flush().unwrap();
    let choice = input(&term);
    println!("\n");

    let code: Vec<u8> = match choice as char {
        '1' => {
            println!("\n===INSERISCI CODICE===\n");
            input_long()
        },
        '2' => {
            let filename = input_string("Inserisci nome completo del file: ");
            fs::read_to_string(filename.trim()).unwrap().as_bytes().to_vec()
        }
        _ => return,
    };
    
    println!("\n===CODICE===\n\n{}", String::from_utf8(code.clone()).expect("UTF-8 INVALIDO"));

    println!("\n===ESECUZIONE CODICE===");

    let mut mem: [u8; 256] = [0; 256];
    let mut pos: usize = 0;
    let mut pc: usize = 0; // er program counter

    while pc < code.len() {
        match code[pc] as char {
            '+' => mem[pos] = mem[pos].wrapping_add(1),
            '-' => mem[pos] = mem[pos].wrapping_sub(1),
            '.' => printchar(&(mem[pos] as char)),
            ',' => mem[pos] = input(&term),
            '<' => pos = if pos == 0 { 255 } else { pos - 1 },
            '>' => pos = if pos == 255 { 0 } else { pos + 1 },
            '[' => {
                if mem[pos] == 0 {
                    let mut nest = 1;
                    pc += 1;
                    while nest != 0 {
                        match code[pc] as char {
                            '[' => nest += 1,
                            ']' => nest -= 1,
                            _ => (),
                        }
                        pc += 1;
                    }
                }
            }
            ']' => {
                if mem[pos] != 0 {
                    let mut nest = 1;
                    pc -= 1;
                    while nest != 0 {
                        match code[pc] as char {
                            ']' => nest += 1,
                            '[' => nest -= 1,
                            _ => (),
                        }
                        pc -= 1;
                    }
                    pc += 1;
                }
            }
            _ => {},
        }
        pc += 1;
    }
}

fn input(term: &Term) -> u8 {
    let result = term.read_char().unwrap();
    print!("{result}");
    stdout().flush().unwrap();
    result as u8
}

fn printchar(c: &char) {
    print!("{c}");
    stdout().flush().unwrap();
}

fn input_long() -> Vec<u8> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.as_bytes().to_vec()
}

fn input_string(prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();

    input_long().into_iter().map(|i| i as char).collect()
}
