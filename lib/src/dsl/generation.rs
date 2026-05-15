use std::num::NonZeroU8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Generation(NonZeroU8);

impl Generation {
    pub fn new(generation: NonZeroU8) -> Self {
        Self(generation)
    }

    pub fn get(&self) -> u8 {
        self.0.get()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GenSpecific<T> {
    Equal {
        equal: Generation,
        value: T,
        otherwise: Box<GenSpecific<T>>,
    },
    From {
        from: Generation,
        value: T,
        otherwise: Box<GenSpecific<T>>,
    },
    Until {
        until: Generation,
        value: T,
        otherwise: Box<GenSpecific<T>>,
    },
    Always(T),
}

impl<T> GenSpecific<T> {
    pub fn get(&self, g: Generation) -> &T {
        match self {
            GenSpecific::Always(v) => v,
            GenSpecific::Equal {
                equal,
                value,
                otherwise,
            } => {
                if g.get() == equal.get() {
                    value
                } else {
                    otherwise.get(g)
                }
            }
            GenSpecific::From {
                from,
                value,
                otherwise,
            } => {
                if g.get() >= from.get() {
                    value
                } else {
                    otherwise.get(g)
                }
            }
            GenSpecific::Until {
                until,
                value,
                otherwise,
            } => {
                if g.get() < until.get() {
                    value
                } else {
                    otherwise.get(g)
                }
            }
        }
    }
}
