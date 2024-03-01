use std::fs::File;
use std::io::{
    Write,
    BufRead,
    BufReader,
    BufWriter
};

#[macro_use]
mod macros;
mod states;
use states::{
    CState, // code (block) state
    HState //  heading state
};

/* TODO:
feature to read args and writing to end of the file or 
rewriting this file, based on the provided args;
 
text formatting things like: bold(**), italic(*);

blockquotes, lists, links, images and other things;
*/

macro_rules! flush { // little macro to use flush!() instead of std::io::stdout().flush().unwrap()
    () => { std::io::stdout().flush().unwrap() }
}

fn input_loop(mut hstate: HState, mut cstate: CState, mut input: Vec<String>, f: File) -> std::io::Result<()> {
    let mut wbuf = BufWriter::new(f); // write buf
    let mut rbuf = BufReader::new(std::io::stdin().lock()); // read buf
    let mut cbuf = Vec::new(); // collect multi-line code inside ``` through iterations
    let mut buf  = String::new(); // simple buf for input

    loop {
        print!(">");
        flush!();
        hstate.off_all();
        buf.clear();
        read_buf!(buf <- rbuf);

        if buf.eq("q") { 
            colored!(pg | "exiting..");
            flush!(); 
            break; 
        }

        let first = buf.chars().nth(0);
        let end   = buf.len() - 1;
        let cstate_on = cstate.if_on();
        if cstate_on && first.ne(&Some('`')) { // collect input inside ``` 
            if buf.chars().nth(end).eq(&Some('`')) { // if in code mode your input ends like that: something`
                ECMASB!(cbuf <- buf[0..end].to_owned()); // ECMASB -> Exit Code Mode After Single Backtick
                flush!(); 
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
                    } else if !cstate_on { // adding extension to ``` things like rs, cpp, c & etc
                        input.push(format!("```{extens}", extens = &buf[1..=end]));
                        colored!(pg | "entered multi-line code mode");
                        cstate.on(); 
                    }
                    flush!(); 
                    continue;
                }
            }
            _  => {}
        }
        }
        if let Some(h) = hstate.if_any_on() { 
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

const FILE_NAME: &str = "test"; // if file already exists we're just rewriting it
fn main() -> std::io::Result<()> {
    let hstate = HState::new();
    let cstate = CState::new();
    let input  = Vec::new();
    let f      = File::create(format!("{FILE_NAME}.md"))?;
    input_loop(hstate, cstate, input, f)
}
