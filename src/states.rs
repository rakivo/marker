#[derive(Debug)]
// NOC -> not overcomplicated
pub enum NOCError {
    IndexOutOfBounds
}

pub trait State {
    fn off_all(&mut self);
    fn if_any_on(&self) -> Option<usize>;
}

// CState -> Code state (in-``` mode)
pub struct CState {
    active:     bool,
    multi_line: bool
}

impl CState {
    pub fn new() -> CState {
        CState {
            active:     false,
            multi_line: false
        }
    }

    pub fn on(&mut self, m: &bool) {
        self.active = true;
        self.multi_line = *m;
    }
}

impl State for CState {
    fn off_all(&mut self) {
        self.active     = false;
        self.multi_line = false;
    }

    // 0 -> active, 1 -> multi_line
    fn if_any_on(&self) -> Option<usize> {
        if self.active {
            return Some(0)
        } else if self.multi_line {
            return Some(1)
        } None
    }
}

// HState -> header state (# things)
pub struct HState {
    hs: [bool; 6]
}

impl HState {
    pub fn new() -> HState {
        HState {
            hs: [false; 6]
        }
    }

    pub fn select_state(&mut self, which: &usize) -> Result<(), NOCError> {
        let which = *which;
        if which < 6 {
            self.hs[which] = true;
            self.off_other(&which);
            Ok(())
        } else {
            Err(NOCError::IndexOutOfBounds)
        }
    }

    pub fn off_other(&mut self, which: &usize) {
        for (i, h) in self.hs.iter_mut().enumerate() {
            if i != *which {
                *h = false
            }
        }
    }
}

impl State for HState {
    fn off_all(&mut self) {
        self.hs.iter_mut().for_each(|h| *h = false);
    }

    fn if_any_on(&self) -> Option<usize> {
        for (i, &h) in self.hs.iter().enumerate() {
            if h { return Some(i); }
        } None
    }
}
