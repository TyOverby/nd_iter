use std::marker::PhantomData;

pub struct IteratorContainer2d<A, B, AIter, BIter> {
    x: AIter,
    y: BIter,

    x_last: Option<A>,
    y_start: BIter,
    _phantom: PhantomData<B>
}


impl <A, B, AIter, BIter> Iterator for IteratorContainer2d<A, B, AIter, BIter>
where A: Clone,
      AIter: Iterator<Item=A>,
      BIter: Iterator<Item=B> + Clone {
    type Item = (A, B);

    fn next(&mut self) -> Option<(A, B)> {
        let y_val = match self.y.next() {
            Some(y) => Some(y),
            None => {
                self.y = self.y_start.clone();
                self.x_last = None;
                self.y.next()
            }
        };
        let x_val = match self.x_last.take() {
            Some(x) => Some(x),
            None => self.x.next()
        };
        self.x_last = x_val.clone();

        match (x_val, y_val) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None
        }
    }
}


//pub fn iter_2d<A: Clone, AIter: Iterator<A>, B, BIter: Iterator<B> + Clone>
//(a_iter: AIter, b_iter: BIter) -> IteratorContainer2d<A, B, AIter, BIter> {
pub fn iter_2d<A, B, AIter, BIter> (a_iter: AIter, b_iter: BIter) ->
IteratorContainer2d<A, B, AIter, BIter>
where A: Clone,
      AIter: Iterator<Item=A>,
      BIter: Iterator<Item=B> + Clone {
    IteratorContainer2d {
        x: a_iter,
        y: b_iter.clone(),

        x_last: None,
        y_start: b_iter,
        _phantom: PhantomData
    }
}





pub struct IteratorContainer3d<A, B, C, AIter, BIter, CIter> {
    x: AIter,
    y: BIter,
    z: CIter,

    x_last: Option<A>,
    y_last: Option<B>,

    y_start: BIter,
    z_start: CIter,
    _phantom: PhantomData<C>
}

impl <A, B, C, AIter, BIter, CIter> Iterator
for IteratorContainer3d<A, B, C, AIter, BIter, CIter>
where A: Clone,
      B: Clone,
      AIter: Iterator<Item=A>,
      BIter: Iterator<Item=B> + Clone,
      CIter: Iterator<Item=C> + Clone {
    type Item = (A, B, C);
    fn next(&mut self) -> Option<(A, B, C)>{
        let z_val = match self.z.next() {
            Some(z) => Some(z),
            None => {
                self.z = self.z_start.clone();
                self.y_last = None;
                self.z.next()
            }
        };
        let y_val = match self.y_last.take() {
            Some(y) => Some(y),
            None => {
                match self.y.next() {
                    Some(y) => Some(y),
                    None => {
                        self.y = self.y_start.clone();
                        self.x_last = None;
                        self.y.next()
                    }
                }
            }
        };
        self.y_last = y_val.clone();
        let x_val = match self.x_last.take() {
            Some(x) => Some(x),
            None => self.x.next()
        };
        self.x_last = x_val.clone();

        match (x_val, y_val, z_val) {
            (Some(x), Some(y), Some(z)) => Some((x, y, z)),
            _ => None
        }
    }
}


pub fn iter_3d<A, B, C, AIter, BIter, CIter>
(a_iter: AIter, b_iter: BIter, c_iter: CIter) -> IteratorContainer3d<A, B, C, AIter, BIter, CIter>
where A: Clone,
      B: Clone,
      BIter: Iterator<Item=B> + Clone,
      CIter: Iterator<Item=C> + Clone
  {
    IteratorContainer3d {
        x: a_iter,
        y: b_iter.clone(),
        z: c_iter.clone(),

        x_last: None,
        y_last: None,
        y_start: b_iter,
        z_start: c_iter,
        _phantom: PhantomData
    }
}
