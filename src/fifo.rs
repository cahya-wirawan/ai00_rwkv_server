use std::cmp;

const FIFO_CAPACITY: usize = 5;

#[derive(Debug, Clone)]
pub struct Fifo {
    size: usize,
    write_idx: usize,
    buffer: [u16; FIFO_CAPACITY]
}

impl Fifo {
    pub(crate) fn iter(&self) -> FifoIterator {
        FifoIterator {
            fifo: self,
            index: 0,
        }
    }

    pub fn push(&mut self, item: u16) -> Result<(), &'static str> {
        self.buffer[self.write_idx % FIFO_CAPACITY] = item;

        self.write_idx = Fifo::increment_index(self.write_idx);
        self.size = cmp::min(self.size + 1, FIFO_CAPACITY);

        Ok(())
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn last(&self) -> u16 {
        self.buffer[(self.write_idx+FIFO_CAPACITY-1)%FIFO_CAPACITY]
    }

    fn increment_index(idx: usize) -> usize {
        (idx + 1) % FIFO_CAPACITY
    }
}

impl Default for Fifo {
    fn default() -> Fifo {
        Fifo {
            size: 0,
            write_idx: 0,
            buffer: [0; FIFO_CAPACITY],
        }
    }
}

pub struct FifoIterator<'a> {
    fifo: &'a Fifo,
    index: usize,
}

impl<'a> Iterator for FifoIterator<'a> {
    type Item = &'a u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.fifo.size {
            let result;
            if self.fifo.size == FIFO_CAPACITY {
                result = Some(&self.fifo.buffer[(self.index+self.fifo.write_idx)%FIFO_CAPACITY]);
            }
            else {
                result = Some(&self.fifo.buffer[(self.index)%FIFO_CAPACITY]);
            }
            self.index += 1;
            result
        } else {
            None
        }
    }
}