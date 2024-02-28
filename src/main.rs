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
    State,
    HState,
    CState
};

// TODO:
// States for md. attrs.

macro_rules! read_buf {
    ($buf: ident <- $rbuf: ident) => {
        $rbuf.read_line(&mut $buf).ok();
        let $buf = $buf.trim().to_owned();
    };
}

macro_rules! fwrite_line {
    ($wbuf: ident <- $input: ident) => {
        $input.iter().for_each(|inp| $wbuf.write_all(format!("{inp}\n").as_bytes()).expect("Failed to write"));
    };
}

macro_rules! hselect_state {
    ($h_state: ident, $h: ident) => {
        $h_state.select_state(&($h.to_digit(10).unwrap_or(0) as usize - 1))
    }
}

// h: in h state, d: in def state
fn input_loop(mut h_state: HState, mut c_state: CState, mut input: Vec<String>, f: File) -> std::io::Result<()> {
    let mut wbuf = BufWriter::new(f);
    let mut rbuf = BufReader::new(std::io::stdin().lock());
    let mut buf  = String::new();

    let mut code_input = Vec::new(); // let's just keep if for now

    loop {
        h_state.off_all();
        buf.clear();
        read_buf!(buf <- rbuf);

        if buf.eq("q") { break; }
        // if there's any mode-symbols at the start
        if let Some(first) = buf.chars().nth(0)
        {
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
                } else {
                    h_state.select_state(&0).ok(); // interpret # without number after it as single #
                }
            }
            '`' => {
                let end = buf.len() - 1;
                if let Some(last) = buf.chars().nth(end) {
                    if last.eq(&'`') {
                        c_state.on(&true);
                        let code = &buf[1..end];
                        println!("c: {code}");
                        code_input.push(format!("```{code}```"));
                        continue;
                    } else {
                        todo!()
                    }
                }
            }
            _  => {}
        }
        }
        if let Some(h) = h_state.if_any_on() {
            let inp = format!("{h} {input}",
                h = (0..=h).map(|_| "#").collect::<String>(),
                input = &buf[2..].split_whitespace().collect::<Vec<_>>().join(" "));
            println!("h: {inp}");
            input.push(inp);
        }
    }
    fwrite_line!(wbuf <- input);
    fwrite_line!(wbuf <- code_input); // let's just keep if for now
    Ok(())
}

const FILE_NAME: &str = "test";
fn main() -> std::io::Result<()> {
    let h_state = HState::new();
    let c_state = CState::new();
    let input   = Vec::new();
    let f       = File::create(&format!("{FILE_NAME}.md"))?;
    input_loop(h_state, c_state, input, f)
}
