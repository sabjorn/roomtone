use std::ops::{Index, IndexMut};

pub struct ReadHead {
    buffer : * const [f32],
    size : usize,
    head_position : usize,
}

impl ReadHead{
    pub fn new(write_head: &WriteHead, delay_samples: usize) -> ReadHead {
        ReadHead {buffer: write_head.buffer.as_slice(), size: write_head.buffer.len(), head_position: (write_head.buffer.len() - delay_samples) % write_head.buffer.len()}
    }
    pub fn seek(&mut self, position: usize){
        self.head_position = position % self.size;
    }
}

impl Iterator for ReadHead {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let sample: f32;
        unsafe {
            sample = (*self.buffer)[self.head_position];
        }
        self.head_position = (self.head_position + 1) % (self.size);

        Some(sample)
    }
}

impl Index<usize> for ReadHead {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        let current_position = i % self.size;
        unsafe {
            &(*self.buffer)[current_position]
        }
    }
}

unsafe impl Send for ReadHead {}

pub struct WriteHead {
    buffer : Vec<f32>,
    head_position : usize,
}

unsafe impl Send for WriteHead {}

impl WriteHead {
    pub fn new(size: usize) -> WriteHead {
        let buffer = vec![0.; size];
        WriteHead {buffer, head_position: 0}
    }

    pub fn push(&mut self, element: f32) {
        self.buffer[self.head_position] = element;
        self.increment();
    }
    
    pub fn increment(&mut self) {
        self.head_position = (self.head_position + 1) % self.buffer.len();
    }

    pub fn seek(&mut self, position: usize){
        self.head_position = if position > self.buffer.len() { 0 } else { position };
    }
    pub fn as_readhead(&self, delay_samples: usize) -> ReadHead {
        ReadHead::new(self, delay_samples)
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

impl Iterator for WriteHead {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.buffer[self.head_position];
        self.increment();

        Some(sample)
    }
}

impl Index<usize> for WriteHead {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        let current_position = i % self.buffer.len();
        &self.buffer[current_position]
    }
}
impl IndexMut<usize> for WriteHead {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        let current_position = i % self.buffer.len();
        &mut self.buffer[current_position]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn read_head_with_delay_output_equals_write_head() {
        let mut write_head = WriteHead::new(5);

        write_head.push(1.0);
       
        for n in 0..4 {
            let mut read_head = write_head.as_readhead(n);
            for j in 0..4 {
                let val = read_head.next().unwrap();
                if j == n {
                    assert_eq!(val, 1.0)
                }
            }
        }
    }

    #[test]
    pub fn multiple_read_head_with_delay_output_equals_write_head() {
        let mut write_head = WriteHead::new(5);

        write_head.push(1.0);
        
        for n in 0..4 {
            let mut read_head_1 = write_head.as_readhead(n);
            let mut read_head_2 = write_head.as_readhead(n+1);
            for j in 0..4 {
                let val_1 = read_head_1.next().unwrap();
                let val_2 = read_head_2.next().unwrap();
                if j == n {
                    assert_eq!(val_1, 1.0)
                }
                if j == (n + 1) {
                    assert_eq!(val_2, 1.0)
                }
            }
        }
    }
    
    #[test]
    pub fn read_head_is_circular() {
        let mut write_head = WriteHead::new(2);
        
        write_head.push(1.0);

        let mut read_head = write_head.as_readhead(0);
        
        assert_eq!(read_head.next().unwrap(), 1.0);
        assert_eq!(read_head.next().unwrap(), 0.0);
        assert_eq!(read_head.next().unwrap(), 1.0);
    }

    #[test]
    pub fn read_head_index_operator() {
        let mut write_head = WriteHead::new(5);

        for n in 0..4 {
            write_head.push(n as f32);
        }

        let read_head = write_head.as_readhead(0);
        for n in 0..4 {
            assert_eq!(read_head[n], n as f32);
        }
    }

    #[test]
    pub fn read_head_index_operator_is_circular() {
        let mut write_head = WriteHead::new(2);

        write_head.push(0.0);
        write_head.push(1.0);

        let read_head = write_head.as_readhead(0);
        assert_eq!(read_head[0], 0.0);
        assert_eq!(read_head[1], 1.0);
        assert_eq!(read_head[2], 0.0);
        assert_eq!(read_head[3], 1.0);
        assert_eq!(read_head[4], 0.0);
    }
    
    #[test]
    pub fn write_head_is_circular() {
        let mut write_head = WriteHead::new(2);
        
        write_head.push(0.0);
        write_head.push(0.0);
        write_head.push(1.0); // wraps around

        let mut read_head = write_head.as_readhead(0);
        
        assert_eq!(read_head.next().unwrap(), 1.0);
        assert_eq!(read_head.next().unwrap(), 0.0);
    }
    
    #[test]
    pub fn write_head_index_operator() {
        let mut write_head = WriteHead::new(5);
        
        for n in 0..4 {
            write_head[n] = n as f32;
        }

        let mut read_head = write_head.as_readhead(0);
        for n in 0..4 {
            assert_eq!(read_head.next().unwrap(), n as f32);
        }
    }

    #[test]
    pub fn write_head_index_operator_is_circular() {
        let mut write_head = WriteHead::new(2);
        
        write_head[0] = 0.0;
        write_head[1] = 1.0;
        write_head[2] = 2.0;
        write_head[3] = 3.0;

        let mut read_head = write_head.as_readhead(0);
        assert_eq!(read_head.next().unwrap(), 2.0);
        assert_eq!(read_head.next().unwrap(), 3.0);
    }
}
