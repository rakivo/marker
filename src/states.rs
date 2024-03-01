use std::fmt::{
    Display,
    Formatter
};

#[derive(Debug)]
// NOC -> Not OverComplicated
pub enum NOCError {
    IndexOutOfBounds
}

impl Display for NOCError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", colored!(fr | "NOC_ERROR: {self:?}"))
    }
}

// CState -> Code State (in-``` mode)
pub struct CState {
    active: bool
}

// HState -> Header State (# things)
pub struct HState {
    hs: [bool; 6]
}

impl CState {
    pub fn new() -> CState {
        CState {
            active: false,
        }
    }

    pub fn on(&mut self) {
        self.active = true;
    }

    pub fn off(&mut self) {
        self.active = false;
    }

    pub fn if_on(&self) -> bool {
        self.active
    }
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
