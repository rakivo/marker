#![feature(let_chains)]

use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
    BufWriter
};

mod states;
use states::*;

macro_rules! read_buf {
    ($buf: ident <- $rbuf: ident) => { // read from user input
        $rbuf.read_line(&mut $buf).ok();
        let $buf = $buf.trim().to_owned();
    };
}

macro_rules! fwrite_line {
    ($wbuf: ident <- $input: ident) => { // write line to file
        $input.iter().for_each(|line| $wbuf.write_all(format!("{line}\n").as_bytes()).expect("Failed to write"));
    };
}

macro_rules! hselect_state {
    ($h_state: ident, $h: ident) => { // select heading state to enter 
        $h_state.select_state(&($h.to_digit(10).unwrap_or(0) as usize - 1))
    }
}

macro_rules! cpush {
    ($c_state: ident, $input: ident <-v $($cbuf: expr), *) => { // push vector
        $($input.extend($cbuf);)*
        $c_state.off();
    };
    ($c_state: ident, $input: ident <- $($arg: expr), *) => { // push not a vector
        $($input.push($arg);)*
        $c_state.off();
    };
}

macro_rules! ECMASB { // ECMASB -> Exit Code Mode After Single Backtick
    ($cbuf: ident <- $arg: expr) => {
        coloredprintln!(r."exited multi-line code mode");
        $cbuf.push($arg);
        $cbuf.push("```".to_owned());
    };
}

macro_rules! coloredprintln { // in this macro we're using ANSI escape codes
    (r.$args: expr) => { // r -> red
        println!("\x1b[31m{}\x1b[0m", $args)
    };
    (g.$args: expr) => { // g -> gray
        println!("\x1b[90m{}\x1b[0m", $args)
    };
}

/* TODO:
add text formatting things like: bold(**), italic(*);

blockquotes, lists, links, images and other things
*/

// h: in h state, nm: no mode state, c: code 
fn input_loop(mut h_state: HState, mut c_state: CState, mut input: Vec<String>, f: File) -> std::io::Result<()> {
    let mut wbuf = BufWriter::new(f); // write buf
    let mut rbuf = BufReader::new(std::io::stdin().lock()); // read buf
    let mut cbuf = Vec::new(); // collect multi-line code inside ``` through iterations
    let mut buf  = String::new(); // simple buf for input

    loop {
        h_state.off_all();
        buf.clear();
        read_buf!(buf <- rbuf);

        if buf.eq("q") { break; }

        let first = buf.chars().nth(0);
        let end   = buf.len() - 1;
        let cstate_on = c_state.if_on();
        if cstate_on && first.ne(&Some('`')) { // collect input inside ``` 
            if buf.chars().nth(end).eq(&Some('`')) {
                ECMASB!(cbuf <- buf[0..end].to_owned());
                cpush!(c_state, input <-v cbuf.clone());
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
                    hselect_state!(h_state, h).map_err(|err| {
                        coloredprintln!(r.err);
                    }).ok();
                } else { // interpret # without number after it as single #
                    h_state.select_state(&0).ok(); 
                }
            }
            '`' => { 
                if end.eq(&0) { // if input is a single char '`'
                    cbuf.push("```".to_owned());
                    if cstate_on {
                        cpush!(c_state, input <-v cbuf.clone());
                        coloredprintln!(g."exited multi-line code mode");
                        cbuf.clear();
                    } else { 
                        coloredprintln!(g."entered multi-line code mode");
                        c_state.on(&true); 
                    }
                    continue;
                } else if let Some(last) = buf.chars().nth(end) {
                    if last.eq(&'`') { // if single line of code
                        let code = &buf[1..end];
                        cpush!(c_state, input <- format!("```{code}```"));
                    } else if !cstate_on { // adding extension to ``` things like rs, cpp, c & etc
                        let extens = &buf[1..=end];
                        input.push(format!("```{extens}"));
                        coloredprintln!(g."entered multi-line code mode");
                        c_state.on(&true); 
                    }
                    continue;
                }
            }
            _  => {}
        }

        }
        if let Some(h) = h_state.if_any_on() { 
            let hinput = format!("{h} {input}", 
                 h = (0..=h).map(|_| "#").collect::<String>(), 
                 input = &buf[3..].to_owned());
            input.push(hinput);
        } else { // input without any modes
            input.push(buf);
        }
    }
    fwrite_line!(wbuf <- input);
    Ok(())
}

const FILE_NAME: &str = "test"; // if file already exist we're just rewriting it
fn main() -> std::io::Result<()> {
    let h_state = HState::new();
    let c_state = CState::new();
    let input   = Vec::new();
    let f       = File::create(&format!("{FILE_NAME}.md"))?;
    input_loop(h_state, c_state, input, f)
}
