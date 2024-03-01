macro_rules! read_buf {
    ($buf: ident <- $rbuf: ident) => { // read from user input
        $rbuf.read_line(&mut $buf).ok();
        let $buf = $buf.trim().to_owned();
    };
}

macro_rules! fwrite {
    ($wbuf: ident <- $input: ident) => { // write array to file
        $input.iter().for_each(|line| $wbuf.write_all(format!("{line}\n").as_bytes()).expect("Failed to write"));
    };
}

macro_rules! hselect_state {
    ($hstate: ident.$h: ident) => { // select heading state to enter 
        $hstate.select_state(&($h.to_digit(10).unwrap_or(0) as usize - 1))
    }
}

macro_rules! cpush {
    ($cstate: ident, $input: ident <-v $($cbuf: expr), *) => { // push vector
        $($input.extend($cbuf);)*
        $cstate.off();
    };
    ($cstate: ident, $input: ident <- $($arg: expr), *) => { // push not a vector
        $($input.push($arg);)*
        $cstate.off();
    };
}

macro_rules! ECMASB { // ECMASB -> Exit Code Mode After Single Backtick
    ($cbuf: ident <- $arg: expr) => {
        colored!(pg | "exited multi-line code mode");
        $cbuf.push($arg);
        $cbuf.push("```".to_owned());
    };
}

macro_rules! colored { 
    (pr | $($args: tt), *) => { // pr -> print red 
        println!("\x1b[31m{}\x1b[0m", format_args!($($args)*))
    };
    (fr | $($args: tt), *) => { // fr -> format red
        format!("\x1b[31m{}\x1b[0m", format_args!($($args)*))
    };
    (pg | $($args: tt), *) => { // pg -> print gray 
        println!("\x1b[90m{}\x1b[0m", format_args!($($args)*))
    };
    (fg | $($args: tt), *) => { // fg -> format gray
        format!("\x1b[90m{}\x1b[0m", format_args!($($args)*))
    };
}
