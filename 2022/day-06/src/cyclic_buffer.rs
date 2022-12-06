pub struct CyclicBuffer<T, const N: usize> {
    buf: [T; N],
    pos_zero: usize,
}

impl<T, const N: usize> CyclicBuffer<T, N> {
    pub fn new(init_buf: [T; N]) -> Self {
        let buf: [T; N] = init_buf;
        Self {
            buf,
            pos_zero: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        let next_pos = (self.pos_zero + 1) % N;
        self.buf[self.pos_zero] = elem;
        self.pos_zero = next_pos;
    }

    pub fn get(&self, i: usize) -> &T {
        let real_i = (self.pos_zero + i) % N;
        &self.buf[real_i]
    }
    
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        let real_i = (self.pos_zero + i) % N;
        &mut self.buf[real_i]
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a CyclicBuffer<T, N> {
    type Item = &'a T;

    type IntoIter = CyclicBufferIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        CyclicBufferIter {
            buf: self,
            pos: 0,
        }
    }
}

pub struct CyclicBufferIter<'a, T, const N: usize> {
    buf: &'a CyclicBuffer<T, N>,
    pos: usize,
}

impl<'a, T, const N: usize> Iterator for CyclicBufferIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= N {
            None
        } else {
            let cur_pos = self.pos; 
            self.pos += 1;
            Some(self.buf.get(cur_pos))
        }
    }
}

#[cfg(test)]
#[test]
fn cyclic_buffer_works() {
    let init = [1, 2, 3, 4];
    let mut buf = CyclicBuffer::<i32, 4>::new(init);
    buf.push(5);
    buf.push(6);
    let v = buf.into_iter()
        .copied()
        .collect::<Vec<i32>>();
    assert_eq!(vec![3, 4, 5, 6], v);
    // todo: why does it allow this push, shouldn't buf be consumed by to_iter()?
    buf.push(7);
    let v = buf.into_iter()
        .copied()
        .collect::<Vec<i32>>();
    assert_eq!(vec![4, 5, 6, 7], v);
}
