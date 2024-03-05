use std::env::args;
use std::io::{
    Write,
    BufRead,
    BufReader,
    BufWriter
};
use std::fs::{
    File,
    OpenOptions
};

#[macro_use]
mod macros;
mod states;
use states::{
    CState, // code (block) state
    HState  // heading state
};

/* TODO:

blockquotes, lists, links, images and other things;

*/

macro_rules! flush { // little macro to use flush!() instead of std::io::stdout().flush().unwrap()
    () => { std::io::stdout().flush().unwrap() }
}

fn input_loop(mut hstate: HState, mut cstate: CState, mut input: Vec<String>, f: File) -> std::io::Result<()> {
    let mut wbuf = BufWriter::new(f); // write buf
    let mut rbuf = BufReader::new(std::io::stdin().lock());
    let mut cbuf = Vec::new();        // collect multi-line code inside ``` through iterations
    let mut buf  = String::new();     // simple buf for input

    let mut editing = false;          // edit mode
    let mut line: usize = 0;          // current line (in edit mode)
    let mut n:    usize = 0;          // buf len

    loop {
        print!(">");
        flush!();
        hstate.off_all();
        buf.clear();
        read_buf!(buf <- rbuf);

        match buf.as_str() {
            "q"      => {
                colored!(pg | "exiting..");
                flush!();
                break;
            }
            "ls"     => {
                input.iter()
                     .enumerate()
                     .for_each(|(i, line)| colored!(pg | "{i} {line}"));
                flush!();
                continue;
            }
            "\x1B[A" => { // ↑
                if line > 0 {
                    line -= 1;
                }
                editing = true;
                if let Some(line) = input.get(line) {
                    println!("{}", colored!(fg | ">{line}"));
                }
                continue;
            }
            "\x1B[B" => { // ↓
                if line < n - 1 && !input.is_empty() {
                    line += 1;
                }
                editing = true;
                if let Some(line) = input.get(line) {
                    println!("{}", colored!(fg | ">{line}"));
                }
                continue;
            }
            _        => {}
        }

        let first = buf.chars().nth(0);
        let end   = buf.len() - 1;
        let cstate_on = cstate.if_on();
        if cstate_on && first.ne(&Some('`')) { // collect input inside ```
            if buf.chars().nth(end).eq(&Some('`')) { // if in code mode your input ends like that: something`
                ECMASB!(cbuf <- buf[0..end].to_owned()); // ECMASB -> Exit Code Mode After Single Backtick
                flush!();
                n += cbuf.len(); line += n - 3;
                cpush!(cstate, input <-v cbuf.clone());
                cbuf.clear();
            } else {
                cbuf.push(buf.clone());
            }
            continue;
        }

        if let Some(first) = first
        { // if there's any special-symbols at the start
        match first {
            '#' => if let Some(h) = buf.chars().nth(1) {
                if h.is_digit(10) {
                    hselect_state!(hstate.h).map_err(|err| {
                        colored!(pr | "{err}");
                    }).ok();
                } else { // interpret # without number after it as single #
                    hstate.select_state(&0).ok();
                }
                colored!(pg | "heading state catched");
                flush!();
            }
            '`' => {
                if end.eq(&0) { // if input is a single char '`'
                    cbuf.push("```".to_owned());
                    if cstate_on {
                        n += cbuf.len(); line += n - 2;
                        cpush!(cstate, input <-v cbuf.clone());
                        colored!(pg | "exited multi-line code mode");
                        cbuf.clear();
                    } else {
                        colored!(pg | "entered multi-line code mode");
                        cstate.on();
                    }
                    flush!();
                    continue;
                } else if let Some(last) = buf.chars().nth(end) {
                    if last.eq(&'`') { // if single line of code
                        cpush!(cstate, input <- format!("```{code}```", code = &buf[1..end]));
                        n += 1; line += 1;
                    } else if !cstate_on { // adding extension to ``` things like rs, cpp, c & etc
                        cbuf.push(format!("```{extens}", extens = &buf[1..=end]));
                        n += 1;
                        colored!(pg | "entered multi-line code mode");
                        cstate.on();
                    }
                    flush!();
                    continue;
                }
            }
            'e' => {
                if editing && line > 0 && line < n {
                    input[line] = buf[2..].to_owned();
                }
                editing = false;
                continue;
            }
            _   => {}
        }
        }
        if let Some(h) = hstate.if_any_on() {
            let hinput = format!("{h} {input}",
                 h = (0..=h).map(|_| "#").collect::<String>(),
                 input = &buf[3..].to_owned());
            input.push(hinput); n += 1; line += 1;
        } else { // input without any modes
            input.push(buf); n += 1; line += 1;
            if editing {
                editing = false;
            }
        }
    }
    fwrite!(wbuf <- input);
    Ok(())
}

macro_rules! format_file_name {
    ($file_name_: ident) => {
        if let Some(mut file_name) = $file_name_ {
            if file_name.ends_with(".md") {
                file_name
            } else {
                file_name.push_str(".md");
                file_name
            }
        } else { "test".to_owned() }
    }
}

fn fargs(file_name_: Option<String>, wa: Option<String>) -> File {
    let file_name = format_file_name!(file_name_);
    println!("{file_name}");
    match wa.expect(&colored!(fr | "Invalid arguments: file name: {file_name:?} ..")).as_str() {
        "w" | "write " => {
            File::create(file_name).expect("Failed to create file: {file_name}")
        },
        "a" | "append" => {
            OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_name.clone())
                .unwrap_or_else(|err| {
                    panic!("{}", colored!(fr | "ERROR: {err} while appending to the file: {file_name}"));
                })
        },
        _ => { panic!("{}", colored!(fr | "Invalid arguments: file name: {file_name:?} ..")) }
    }
}

fn main() -> std::io::Result<()> {
    let args = args().collect::<Vec<_>>();
    if args.len() < 2 {
        colored!(pr | "usage: ./marker <w | a>");
        colored!(pr | "w: overwrite the file");
        colored!(pr | "a: push input to the end of the file");
        colored!(pg | "optionally you can add file name flag: ");
        colored!(pg | "for instance: ./marker w hello");
        return Ok(())
    }

    let hstate = HState::new();
    let cstate = CState::new();
    let input  = Vec::new();
    input_loop(hstate, cstate, input, fargs(args.get(2).cloned(), args.get(1).cloned()))
}
