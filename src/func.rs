use num_complex::Complex32;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ComplexFunction<'a> {
    roots: &'a [Complex32],
}

impl<'a> ComplexFunction<'a> {
    pub(crate) fn new(roots: &'a [Complex32]) -> Self {
        Self { roots }
    }

    pub(crate) fn eval(&self, z: Complex32) -> Complex32 {
        self.roots.iter().map(|r| z - r).product()
    }

    pub(crate) fn derivative(&self, z: Complex32) -> Complex32 {
        // generalised form of product rule
        // sum of (d(this one) * prod(all the others))

        (0..self.degree())
            // .into_par_iter()
            .map(|i| {
                self.roots
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, r)| z - r)
                    .product::<Complex32>()
            })
            .sum::<Complex32>()
    }

    pub(crate) fn degree(&self) -> usize {
        self.roots.len()
    }

    pub(crate) fn identify_root(&self, z: Complex32, epsilon: f32) -> Option<usize> {
        (0..self.degree()).find(|&i| (z - self.roots[i]).norm_sqr() < epsilon * epsilon)
    }
}
