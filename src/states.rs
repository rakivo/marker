#[derive(Debug)]
// NOC -> not overcomplicated
pub enum NOCError {
    IndexOutOfBounds
}

// CState -> Code state (in-``` mode)
pub struct CState {
    active:     bool,
    multi_line: bool // let's just keep it for now
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

    pub fn off(&mut self) {
        self.active     = false;
        self.multi_line = false;
    }

    pub fn if_on(&self) -> bool {
        self.active
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
            self.off_others(&which);
            Ok(())
        } else {
            Err(NOCError::IndexOutOfBounds)
        }
    }

    pub fn off_others(&mut self, which: &usize) {
        for (i, h) in self.hs.iter_mut().enumerate() {
            if i != *which {
                *h = false
            }
        }
    }

    pub fn off_all(&mut self) {
        self.hs.iter_mut().for_each(|h| *h = false);
    }

    pub fn if_any_on(&self) -> Option<usize> {
        for (i, &h) in self.hs.iter().enumerate() {
            if h { return Some(i); }
        } None
    }
}
