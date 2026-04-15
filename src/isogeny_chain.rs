use fp2::traits::Fq as FqTrait;

/// Theta Point in with a projective representation for dimension 2
#[derive(Clone, Copy, Debug)]
pub struct ThetaPoint<Fq: FqTrait> {
    x: Fq,
    y: Fq,
    z: Fq,
    t: Fq,
}

impl<Fq: FqTrait> ThetaPoint<Fq> {
    /// Compile time, create a new theta point from Fq elements
    pub const fn new(x: &Fq, y: &Fq, z: &Fq, t: &Fq) -> Self {
        Self {
            x: *x,
            y: *y,
            z: *z,
            t: *t,
        }
    }

    /// Recover the coordinates of the element
    pub fn coords(&self) -> (Fq, Fq, Fq, Fq) {
        (self.x, self.y, self.z, self.t)
    }

    /// Compute the Hadamard transformation of the point in place
    /// Cost: 8a
    #[inline(always)]
    pub fn set_hadamard(&mut self) {
        let t1 = self.x + self.y;
        let t2 = self.x - self.y;
        let t3 = self.z + self.t;
        let t4 = self.z - self.t;

        self.x = t1 + t3;
        self.y = t2 + t4;
        self.z = t1 - t3;
        self.t = t2 - t4;
    }

    /// Return the Hadamard transform of this point
    /// Cost: 8a
    pub fn hadamard(&self) -> Self {
        let mut p = *self;
        p.set_hadamard();
        p
    }

    /// Compute the squaring transform of the point in place
    /// Cost: 4s
    #[inline(always)]
    pub fn set_square(&mut self) {
        self.x.set_square();
        self.y.set_square();
        self.z.set_square();
        self.t.set_square();
    }

    /// Compute H(S(P)) of the point in place
    /// Cost: 4s + 8a
    pub fn set_square_hadamard(&mut self) {
        self.set_square();
        self.set_hadamard();
    }

    /// Return H(S(P)) of this point
    /// Cost: 4s + 8a
    pub fn square_hadamard(&self) -> Self {
        let mut p = *self;
        p.set_square_hadamard();
        p
    }

    /// Multiply each coordinate of self by the coordinates of other in place
    /// Cost: 4m
    pub fn set_pointwise_mul(&mut self, other: &Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.t *= other.t;
    }
}

/// Default element used for initialization
impl<Fq: FqTrait> Default for ThetaPoint<Fq> {
    fn default() -> Self {
        Self {
            x: Fq::ZERO,
            y: Fq::ZERO,
            z: Fq::ZERO,
            t: Fq::ZERO,
        }
    }
}

impl<Fq: FqTrait> ::std::fmt::Display for ThetaPoint<Fq> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        writeln!(f, "ThetaPoint: (")?;
        writeln!(f, "    {},", self.x)?;
        writeln!(f, "    {},", self.y)?;
        writeln!(f, "    {},", self.z)?;
        writeln!(f, "    {},", self.t)?;
        write!(f, ")")
    }
}

/// Theta Structure for dimension 2
#[derive(Clone, Copy, Debug)]
pub struct ThetaStructure<Fq: FqTrait> {
    null_point: ThetaPoint<Fq>,
    arithmetic_precom: [ThetaPoint<Fq>; 2],
}

impl<Fq: FqTrait> ThetaStructure<Fq> {
    /// Given the coordinates of a null point, create a null point and
    /// precompute the 8 Fp2 elements which are used for doubling and isogeny
    /// computations.
    pub fn new_from_coords(x: &Fq, y: &Fq, z: &Fq, t: &Fq) -> Self {
        let null_point = ThetaPoint::new(x, y, z, t);
        Self {
            null_point: null_point,
            arithmetic_precom: ThetaStructure::arithmetic_precomputation(&null_point),
        }
    }

    /// Given a null point, store the null point and precompute 8 Fp2
    /// elements which are used for doubling and isogeny computations.
    pub fn new_from_point(null_point: &ThetaPoint<Fq>) -> Self {
        Self {
            null_point: *null_point,
            arithmetic_precom: ThetaStructure::arithmetic_precomputation(null_point),
        }
    }

    /// Return the null point of the curve
    pub fn null_point(&self) -> ThetaPoint<Fq> {
        self.null_point
    }

    /// Compute projective precomputed ratios used in differential addition.
    fn arithmetic_precomputation(null_point: &ThetaPoint<Fq>) -> [ThetaPoint<Fq>; 2] {
        // Compute projectively a/b = a^2*c*d, etc.
        let (a, b, c, d) = null_point.coords();
        let t1 = a * b;
        let t2 = c * d;

        let x0 = t2 * b;
        let x1 = t2 * a;
        let x2 = t1 * d;
        let x3 = t1 * c;

        let lam_1 = ThetaPoint::new(&x0, &x1, &x2, &x3);

        // Compute projectively A^2/B^2 = A^4*C^2*D^2, etc.
        let (aa, bb, cc, dd) = null_point.square_hadamard().coords();
        let t1 = aa * bb;
        let t2 = cc * dd;

        let y0 = t2 * bb;
        let y1 = t2 * aa;
        let y2 = t1 * dd;
        let y3 = t1 * cc;

        let lam_2 = ThetaPoint::new(&y0, &y1, &y2, &y3);

        [lam_1, lam_2]
    }

    /// Given a point P, compute it's double [2]P in place.
    /// Cost 8S + 8M + 16a
    #[inline(always)]
    pub fn double_into(&self, p: &mut ThetaPoint<Fq>) {
        p.set_square();
        p.set_hadamard();
        p.set_square();
        p.set_pointwise_mul(&self.arithmetic_precom[1]);
        p.set_hadamard();
        p.set_pointwise_mul(&self.arithmetic_precom[0]);
    }

    /// Compute [2^n] * self
    #[inline]
    pub fn double_iter(&self, p: &ThetaPoint<Fq>, n: usize) -> ThetaPoint<Fq> {
        let mut r = *p;
        for _ in 0..n {
            self.double_into(&mut r)
        }
        r
    }

    /// Given the 8-torsion above the kernel, compute the codomain of the
    /// (2,2)-isogeny and the image of all points in `image_points`.
    /// Cost:
    ///   Codomain: 8S + 9M
    ///   Image: 4S + 4M per point
    pub fn two_isogeny(
        k1: &ThetaPoint<Fq>,
        k2: &ThetaPoint<Fq>,
        image_points: &mut [ThetaPoint<Fq>],
        hadamard: [bool; 2],
    ) -> Self {
        // Conditionally hadamard to move to the dual, TODO make this cleaner
        let (k1, k2) = if hadamard[0] {
            (k1.hadamard(), k2.hadamard())
        } else {
            (*k1, *k2)
        };

        // coordinates of H(S(P))
        let (xa, xb, _, _) = k1.square_hadamard().coords();
        let (za, tb, zc, td) = k2.square_hadamard().coords();

        let xa_tb = xa * tb;
        let za_xb = za * xb;
        let zc_td = zc * td;

        let a = za * xa_tb;
        let b = tb * za_xb;
        let c = zc * xa_tb;
        let d = td * za_xb;
        let mut codomain = ThetaPoint::new(&a, &b, &c, &d);

        let a_inv = xb * zc_td;
        let b_inv = xa * zc_td;
        let c_inv = d;
        let d_inv = c;
        let inverses = ThetaPoint::new(&a_inv, &b_inv, &c_inv, &d_inv);

        if hadamard[1] {
            codomain.set_hadamard();
        }

        for p in image_points.iter_mut() {
            if hadamard[0] {
                p.set_hadamard()
            }
            p.set_square_hadamard();
            p.set_pointwise_mul(&inverses);
            if hadamard[1] {
                p.set_hadamard();
            }
        }

        Self::new_from_point(&codomain)
    }

    /// Advance the balanced strategy by doubling kernel points until
    /// the top of the stack has order 2^1. Returns the new stack depth.
    fn advance_strategy(
        &self,
        pts: &mut [ThetaPoint<Fq>],
        orders: &mut [usize],
        k: usize,
    ) -> usize {
        let mut k = k;
        while orders[k] != 1 {
            k += 1;
            let m = orders[k - 1] >> 1;
            pts[2 * k] = self.double_iter(&pts[2 * k - 2], m);
            pts[2 * k + 1] = self.double_iter(&pts[2 * k + 1 - 2], m);
            orders[k] = orders[k - 1].saturating_sub(m);
        }
        k
    }

    /// Compute a 2,2 chain
    pub fn two_two_isogeny_chain(
        &self,
        ker_1: &ThetaPoint<Fq>,
        ker_2: &ThetaPoint<Fq>,
        len: usize,
    ) -> Self {
        // Compute the amount of space we need for the balanced strategy.
        let space = (usize::BITS - len.leading_zeros() + 1) as usize;

        // Store points of order 2^i for the balanced strategy. We need two
        // vectors here, as the first step computes with elements of type
        // ProductPoint, while every other step computes points of type
        // ThetaPoint.
        let mut kernel_pts: Vec<ThetaPoint<Fq>> = vec![ThetaPoint::default(); 2 * space];

        // The values i such that each point in stategy_points has order 2^i
        let mut orders: Vec<usize> = vec![0; space];

        // Include the kernel points into the strategy array
        kernel_pts[0] = *ker_1;
        kernel_pts[1] = *ker_2;

        // Initalise the orders list, points in the above vectors have order
        // 2^(orders[i] + 2), as we use the 8-torsion above.
        orders[0] = len;

        let mut domain = *self;
        let mut k = 0;
        for _ in 0..len {
            // Perform doublings of the kernel elements, decreasing the values of orders
            k = domain.advance_strategy(&mut kernel_pts, &mut orders, k);

            // Extract out the kernel for this step.
            let k1 = kernel_pts[2 * k];
            let k2 = kernel_pts[2 * k + 1];

            // Perform one step of the (2,2) isogeny and push through all points.
            domain =
                ThetaStructure::two_isogeny(&k1, &k2, &mut kernel_pts[..(2 * k)], [true, false]);

            // Reduce the order of the points we evaluated
            for ord in orders.iter_mut().take(k) {
                *ord -= 1;
            }
            k = k.saturating_sub(1);
        }

        domain
    }
}
