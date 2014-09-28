use std::num::One;
pub struct Range2dIter<A> {
    y_start: A,

    x_step: A,
    y_step: A,

    x: A,
    y: A,

    x_end: A,
    y_end: A
}

impl <A: Add<A, A> + PartialOrd + Clone> Iterator<(A, A)> for Range2dIter<A> {
    fn next(&mut self) -> Option<(A, A)> {
        if self.y >= self.y_end {
            self.y = self.y_start.clone();
            self.x = self.x + self.x_step;
        }
        if self.x >= self.x_end {
            return None;
        }

        let ret = (self.x.clone(), self.y.clone());
        self.y =  self.y + self.y_step;
        return Some(ret);
    }

    // TODO: Implement size-hint
}

pub struct Range2dInclusiveIter<A> {
    y_start: A,

    x_step: A,
    y_step: A,

    x: A,
    y: A,

    x_end: A,
    y_end: A
}

impl <A: Add<A, A> + PartialOrd + Clone> Iterator<(A, A)> for Range2dInclusiveIter<A> {
    fn next(&mut self) -> Option<(A, A)> {
        if self.y > self.y_end {
            self.y = self.y_start.clone();
            self.x = self.x + self.x_step;
        }
        if self.x > self.x_end {
            return None;
        }

        let ret = (self.x.clone(), self.y.clone());
        self.y =  self.y + self.y_step;
        return Some(ret);
    }

    // TODO: Implement size-hint
}

pub fn range_2d<A: One + Add<A, A> + PartialOrd + Clone>(x_rng: (A, A), y_rng: (A, A)) -> Range2dIter<A> {
    let (xs, xe) = x_rng;
    let (ys, ye) = y_rng;
    Range2dIter {
        y_start: ys.clone(),

        x_step: One::one(),
        y_step: One::one(),

        x: xs,
        y: ys,

        x_end: xe,
        y_end: ye
    }
}

pub fn range_2d_step<A: Add<A, A> + PartialOrd + Clone>(x_rng: (A, A, A), y_rng: (A, A, A)) -> Range2dIter<A> {
    let (xs, xe, xp) = x_rng;
    let (ys, ye, yp) = y_rng;
    Range2dIter {
        y_start: ys.clone(),

        x_step: xp,
        y_step: yp,

        x: xs,
        y: ys,

        x_end: xe,
        y_end: ye
    }
}

pub fn range_2d_inclusive<A: One + Add<A, A> + PartialOrd + Clone>(x_rng: (A, A), y_rng: (A, A)) -> Range2dInclusiveIter<A> {
    let (xs, xe) = x_rng;
    let (ys, ye) = y_rng;
    Range2dInclusiveIter {
        y_start: ys.clone(),

        x_step: One::one(),
        y_step: One::one(),

        x: xs,
        y: ys,

        x_end: xe,
        y_end: ye
    }
}

pub fn range_2d_step_inclusive<A: One + Add<A, A> + PartialOrd + Clone>(x_rng: (A, A, A), y_rng: (A, A, A)) -> Range2dInclusiveIter<A> {
    let (xs, xe, xp) = x_rng;
    let (ys, ye, yp) = y_rng;
    Range2dInclusiveIter {
        y_start: ys.clone(),

        x_step: xp,
        y_step: yp,

        x: xs,
        y: ys,

        x_end: xe,
        y_end: ye
    }
}
