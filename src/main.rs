#![feature(let_chains)]
#[allow(unused_imports)]
use std::io::{
    prelude::*,
    BufReader,
    BufWriter
};
use std::fs::File;

mod states;

use crate::states::{
    HState,
    CState
};

// TODO:
// States for md. attrs.

macro_rules! read_buf {
    ($buf: ident <- $rbuf: ident) => { // read from input
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
    ($c_state: ident, $input: ident <-v $cbuf: expr) => { // push vector
        $input.extend($cbuf);
        $c_state.off();
    };
    ($c_state: ident, $input: ident <- $($arg: expr), *) => { // push not a vector
        $($input.push($arg);)*
        $c_state.off();
    };
}

// h: in h state, d: in def state
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
            println!("c: {buf}");
            if buf.chars().nth(end).eq(&Some('`')) {
                println!("exited multi-line code mode");
                cbuf.push(buf[0..end].to_owned());
                cbuf.push("```".to_owned());
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
                    match hselect_state!(h_state, h) {
                        Err(e) => {
                            eprintln!("ERROR: {e:?}");
                            continue;
                        }
                        Ok(_)  => {}
                    }
                } else { // interpret # without number after it as single #
                    h_state.select_state(&0).ok(); 
                }
            }
            '`' => { // for now we're just ignoring language specification after '`' symbol
                if end.eq(&0) { // if input is a single char '`'
                    cbuf.push("```".to_owned());
                    if cstate_on {
                        cpush!(c_state, input <-v cbuf.clone());
                        println!("exited multi-line code mode");
                        cbuf.clear();
                    } else { 
                        println!("entered multi-line code mode");
                        c_state.on(&true); 
                    }
                    continue;
                } else if let Some(last) = buf.chars().nth(end) {
                    if last.eq(&'`') { // if single line of code
                        let code = &buf[1..end];
                        println!("c: {code}");
                        cpush!(c_state, input <- format!("```{code}```"));
                        continue;
                    }
                }
            }
            _  => {}
        }

        }
        if let Some(h) = h_state.if_any_on() { 
            let hinput = format!("{h} {input}", 
                 h = (0..=h).map(|_| "#").collect::<String>(), 
                 input = &buf[3..].to_owned());
            println!("h: {hinput}");
            input.push(hinput);
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
