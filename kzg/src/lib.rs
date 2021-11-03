pub trait Fr: Clone {
    // Assume that Fr can't fail on creation

    fn default() -> Self; // -> Result<Self, String>;

    fn zero() -> Self; // -> Result<Self, String>;

    fn one() -> Self; // -> Result<Self, String>;

    fn rand() -> Self; // -> Result<Self, String>;

    fn from_u64_arr(u: &[u64; 4]) -> Self;

    fn from_u64(u: u64) -> Self;

    fn is_one(&self) -> bool;

    fn is_zero(&self) -> bool;

    fn sqr(&self) -> Self;

    fn mul(&self, b: &Self) -> Self;

    fn add(&self, b: &Self) -> Self;

    fn sub(&self, b: &Self) -> Self;

    fn eucl_inverse(&self) -> Self;

    fn negate(&self) -> Self;

    fn inverse(&self) -> Self;

    fn pow(&self, n: usize) -> Self;

    fn equals(&self, b: &Self) -> bool;

    // Other teams, aside from the c-kzg bindings team, may as well leave its body empty
    fn destroy(&mut self);
}

pub trait G1: Clone {
    fn default() -> Self;

    fn rand() -> Self;

    fn add_or_double(&mut self, b: &Self) -> Self;

    fn equals(&self, b: &Self) -> bool;

    // Other teams, aside from the c-kzg bindings team, may as well leave its body empty
    fn destroy(&mut self);
}

pub trait G2: Clone {
    // TODO: populate with needed fns
}

pub trait FFTFr<Coeff: Fr> {
    fn fft_fr(&self, data: &[Coeff], inverse: bool) -> Result<Vec<Coeff>, String>;
}

pub trait FFTG1<Coeff: G1> {
    fn fft_g1(&self, data: &[Coeff], inverse: bool) -> Result<Vec<Coeff>, String>;
}

pub trait DAS<Coeff: Fr> {
    fn das_fft_extension(&self, evens: &[Coeff]) -> Result<Vec<Coeff>, String>;
}

pub trait ZeroPoly<Coeff: Fr, Polynomial: Poly<Coeff>> {
    fn do_zero_poly_mul_partial(&self, idxs: &[usize], stride: usize)
        -> Result<Polynomial, String>;

    fn reduce_partials(
        &self,
        domain_size: usize,
        partials: &[Polynomial],
    ) -> Result<Polynomial, String>;

    fn zero_poly_via_multiplication(
        &self,
        domain_size: usize,
        idxs: &[usize],
    ) -> Result<(Vec<Coeff>, Polynomial), String>;
}

pub trait FFTSettings<Coeff: Fr>: Clone {
    fn default() -> Self;

    fn new(scale: usize) -> Result<Self, String>;

    fn get_max_width(&self) -> usize;

    fn get_expanded_roots_of_unity_at(&self, i: usize) -> Coeff;

    fn get_expanded_roots_of_unity(&self) -> &[Coeff];

    fn get_reverse_roots_of_unity_at(&self, i: usize) -> Coeff;

    fn get_reversed_roots_of_unity(&self) -> &[Coeff];

    // Other teams, aside from the c-kzg bindings team, may as well leave its body empty
    fn destroy(&mut self);
}

pub trait Poly<Coeff: Fr>: Clone {
    fn default() -> Self;

    fn new(size: usize) -> Result<Self, String>;

    fn get_coeff_at(&self, i: usize) -> Coeff;

    fn set_coeff_at(&mut self, i: usize, x: &Coeff);

    fn get_coeffs(&self) -> &[Coeff];

    fn len(&self) -> usize;

    fn eval(&self, x: &Coeff) -> Coeff;

    fn scale(&mut self);

    fn unscale(&mut self);

    fn inverse(&mut self, new_len: usize) -> Result<Self, String>;

    fn div(&mut self, x: &Self) -> Result<Self, String>;

    fn mul_direct(&mut self, x: &Self, len: usize) -> Result<Self, String>;

    // Other teams, aside from the c-kzg bindings team, may as well leave its body empty
    fn destroy(&mut self);
}

pub trait KZGSettings<
    Coeff1: Fr,
    Coeff2: G1,
    Coeff3: G2,
    Fs: FFTSettings<Coeff1>,
    Polynomial: Poly<Coeff1>,
>: Clone
{
    fn default() -> Self;

    fn new(secret_g1: &Vec<Coeff2>, secret_g2: &Vec<Coeff3>, length: usize, fs: Fs) -> Self;

    fn commit_to_poly(&self, p: &Polynomial) -> Result<Coeff2, String>;

    fn compute_proof_single(&self, p: &Polynomial, x: &Coeff1) -> Coeff2;

    fn check_proof_single(&self, com: &Coeff2, proof: &Coeff2, x: &Coeff1, value: &Coeff1) -> bool;

    fn compute_proof_multi(&self, p: &Polynomial, x: &Coeff1, n: usize) -> Coeff2;

    fn check_proof_multi(
        &self,
        com: &Coeff2,
        proof: &Coeff2,
        x: &Coeff1,
        values: &Vec<Coeff1>,
        n: usize,
    ) -> bool;

    fn get_expanded_roots_of_unity_at(&self, i: usize) -> Coeff1;

    // Other teams, aside from the c-kzg bindings team, may as well leave its body empty
    fn destroy(&mut self);
}