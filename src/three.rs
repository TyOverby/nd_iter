use std::num::One;

pub struct Range3dIter<A> {
    y_start: A,
    z_start: A,

    x_step: A,
    y_step: A,
    z_step: A,

    x: A,
    y: A,
    z: A,

    x_end: A,
    y_end: A,
    z_end: A
}

impl <A: Add<A, A> + PartialOrd + Clone> Iterator<(A, A, A)> for Range3dIter<A> {
    fn next(&mut self) -> Option<(A, A, A)> {
        if self.z >= self.z_end {
            self.z = self.z_start.clone();
            self.y = self.y + self.y_step;
        }
        if self.y >= self.y_end {
            self.y = self.y_start.clone();
            self.x = self.x + self.x_step;
        }
        if self.x >= self.x_end {
            return None;
        }

        let ret = (self.x.clone(), self.y.clone(), self.z.clone());
        self.z =  self.z + self.z_step;
        return Some(ret);
    }

    // TODO: Implement size-hint
}

pub struct Range3dInclusiveIter<A> {
    y_start: A,
    z_start: A,

    x_step: A,
    y_step: A,
    z_step: A,

    x: A,
    y: A,
    z: A,

    x_end: A,
    y_end: A,
    z_end: A
}

impl <A: Add<A, A> + PartialOrd + Clone> Iterator<(A, A, A)> for Range3dInclusiveIter<A> {
    fn next(&mut self) -> Option<(A, A, A)> {
        if self.z > self.z_end {
            self.z = self.z_start.clone();
            self.y = self.y + self.y_step;
        }
        if self.y > self.y_end {
            self.y = self.y_start.clone();
            self.x = self.x + self.x_step;
        }
        if self.x > self.x_end {
            return None;
        }

        let ret = (self.x.clone(), self.y.clone(), self.z.clone());
        self.z =  self.z + self.z_step;
        return Some(ret);
    }

    // TODO: Implement size-hint
}

pub fn range_3d<A: One + Add<A, A> + PartialOrd + Clone>
(x_rng: (A, A), y_rng: (A, A), z_rng: (A, A)) -> Range3dIter<A> {
    let (xs, xe) = x_rng;
    let (ys, ye) = y_rng;
    let (zs, ze) = z_rng;
    Range3dIter {
        y_start: ys.clone(),
        z_start: zs.clone(),

        x_step: One::one(),
        y_step: One::one(),
        z_step: One::one(),

        x: xs,
        y: ys,
        z: zs,

        x_end: xe,
        y_end: ye,
        z_end: ze
    }
}

pub fn range_3d_step<A: One + Add<A, A> + PartialOrd + Clone>
(x_rng: (A, A, A), y_rng: (A, A, A), z_rng: (A, A, A)) -> Range3dIter<A> {
    let (xs, xe, xp) = x_rng;
    let (ys, ye, yp) = y_rng;
    let (zs, ze, zp) = z_rng;
    Range3dIter {
        y_start: ys.clone(),
        z_start: zs.clone(),

        x_step: xp,
        y_step: yp,
        z_step: zp,

        x: xs,
        y: ys,
        z: zs,

        x_end: xe,
        y_end: ye,
        z_end: ze
    }
}

pub fn range_3d_inclusive<A: One + Add<A, A> + PartialOrd + Clone>
(x_rng: (A, A), y_rng: (A, A), z_rng: (A, A)) -> Range3dInclusiveIter<A> {
    let (xs, xe) = x_rng;
    let (ys, ye) = y_rng;
    let (zs, ze) = z_rng;
    Range3dInclusiveIter {
        y_start: ys.clone(),
        z_start: zs.clone(),

        x_step: One::one(),
        y_step: One::one(),
        z_step: One::one(),

        x: xs,
        y: ys,
        z: zs,

        x_end: xe,
        y_end: ye,
        z_end: ze
    }
}

pub fn range_3d_step_inclusive<A: One + Add<A, A> + PartialOrd + Clone>
(x_rng: (A, A, A), y_rng: (A, A, A), z_rng: (A, A, A)) -> Range3dInclusiveIter<A> {
    let (xs, xe, xp) = x_rng;
    let (ys, ye, yp) = y_rng;
    let (zs, ze, zp) = z_rng;
    Range3dInclusiveIter {
        y_start: ys.clone(),
        z_start: zs.clone(),

        x_step: xp,
        y_step: yp,
        z_step: zp,

        x: xs,
        y: ys,
        z: zs,

        x_end: xe,
        y_end: ye,
        z_end: ze
    }
}
