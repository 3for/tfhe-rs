use crate::core_crypto::commons::ciphertext_modulus::CiphertextModulus;
use crate::core_crypto::commons::math::decomposition::{
    DecompositionLevel, DecompositionTerm, DecompositionTermNonNative,
};
use crate::core_crypto::commons::numeric::UnsignedInteger;
use crate::core_crypto::commons::parameters::{DecompositionBaseLog, DecompositionLevelCount};

/// An iterator that yields the terms of the signed decomposition of an integer.
///
/// # Warning
///
/// This iterator yields the decomposition in reverse order. That means that the highest level
/// will be yielded first.
pub struct SignedDecompositionIter<T>
where
    T: UnsignedInteger,
{
    // The base log of the decomposition
    base_log: usize,
    // The number of levels of the decomposition
    level_count: usize,
    // The internal state of the decomposition
    state: T,
    // The current level
    current_level: usize,
    // A mask which allows to compute the mod B of a value. For B=2^4, this guy is of the form:
    // ...0001111
    mod_b_mask: T,
    // A flag which store whether the iterator is a fresh one (for the recompose method)
    fresh: bool,
}

impl<T> SignedDecompositionIter<T>
where
    T: UnsignedInteger,
{
    pub(crate) fn new(
        input: T,
        base_log: DecompositionBaseLog,
        level: DecompositionLevelCount,
    ) -> Self {
        Self {
            base_log: base_log.0,
            level_count: level.0,
            state: input >> (T::BITS - base_log.0 * level.0),
            current_level: level.0,
            mod_b_mask: (T::ONE << base_log.0) - T::ONE,
            fresh: true,
        }
    }

    pub(crate) fn is_fresh(&self) -> bool {
        self.fresh
    }

    /// Return the logarithm in base two of the base of this decomposition.
    ///
    /// If the decomposition uses a base $B=2^b$, this returns $b$.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::SignedDecomposer;
    /// use tfhe::core_crypto::commons::parameters::{DecompositionBaseLog, DecompositionLevelCount};
    /// let decomposer =
    ///     SignedDecomposer::<u32>::new(DecompositionBaseLog(4), DecompositionLevelCount(3));
    /// let val = 1_340_987_234_u32;
    /// let decomp = decomposer.decompose(val);
    /// assert_eq!(decomp.base_log(), DecompositionBaseLog(4));
    /// ```
    pub fn base_log(&self) -> DecompositionBaseLog {
        DecompositionBaseLog(self.base_log)
    }

    /// Return the number of levels of this decomposition.
    ///
    /// If the decomposition uses $l$ levels, this returns $l$.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::SignedDecomposer;
    /// use tfhe::core_crypto::commons::parameters::{DecompositionBaseLog, DecompositionLevelCount};
    /// let decomposer =
    ///     SignedDecomposer::<u32>::new(DecompositionBaseLog(4), DecompositionLevelCount(3));
    /// let val = 1_340_987_234_u32;
    /// let decomp = decomposer.decompose(val);
    /// assert_eq!(decomp.level_count(), DecompositionLevelCount(3));
    /// ```
    pub fn level_count(&self) -> DecompositionLevelCount {
        DecompositionLevelCount(self.level_count)
    }
}

impl<T> Iterator for SignedDecompositionIter<T>
where
    T: UnsignedInteger,
{
    type Item = DecompositionTerm<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // The iterator is not fresh anymore
        self.fresh = false;
        // We check if the decomposition is over
        if self.current_level == 0 {
            return None;
        }
        // We decompose the current level
        let output = decompose_one_level(self.base_log, &mut self.state, self.mod_b_mask);
        self.current_level -= 1;
        // We return the output for this level
        Some(DecompositionTerm::new(
            DecompositionLevel(self.current_level + 1),
            DecompositionBaseLog(self.base_log),
            output,
        ))
    }
}

fn decompose_one_level<S: UnsignedInteger>(base_log: usize, state: &mut S, mod_b_mask: S) -> S {
    let res = *state & mod_b_mask;
    *state >>= base_log;
    let mut carry = (res.wrapping_sub(S::ONE) | *state) & res;
    carry >>= base_log - 1;
    *state += carry;
    res.wrapping_sub(carry << base_log)
}

/// An iterator that yields the terms of the signed decomposition of an integer.
///
/// # Warning
///
/// This iterator yields the decomposition in reverse order. That means that the highest level
/// will be yielded first.
#[derive(Clone, Debug)]
pub struct SignedDecompositionIterNonNative<T>
where
    T: UnsignedInteger,
{
    // The base log of the decomposition
    base_log: usize,
    // The number of levels of the decomposition
    level_count: usize,
    // The internal state of the decomposition
    state: T,
    // The current level
    current_level: usize,
    // A mask which allows to compute the mod B of a value. For B=2^4, this guy is of the form:
    // ...0001111
    mod_b_mask: T,
    // Ciphertext modulus
    ciphertext_modulus: CiphertextModulus<T>,
    // A flag which store whether the iterator is a fresh one (for the recompose method)
    fresh: bool,
    shift: T,
}

impl<T> SignedDecompositionIterNonNative<T>
where
    T: UnsignedInteger,
{
    pub(crate) fn new(
        (state, shift): (T, T),
        base_log: DecompositionBaseLog,
        level: DecompositionLevelCount,
        ciphertext_modulus: CiphertextModulus<T>,
    ) -> Self {
        // To guarantee correctness of the custom fast modulus from native to q to work
        // we need abs(decomp_val) <= ciphertext_modulus / 2
        // the iter returns decomp_val such that abs(decomp_val) <= base / 2
        // so if base <= ciphertex_modulus
        // then abs(decomp_val) <= ciphertext_modulus / 2
        assert!((1 << base_log.0) <= ciphertext_modulus.get_custom_modulus());
        Self {
            base_log: base_log.0,
            level_count: level.0,
            state,
            current_level: level.0,
            mod_b_mask: (T::ONE << base_log.0) - T::ONE,
            ciphertext_modulus,
            fresh: true,
            shift,
        }
    }

    pub(crate) fn is_fresh(&self) -> bool {
        self.fresh
    }

    /// Return the logarithm in base two of the base of this decomposition.
    ///
    /// If the decomposition uses a base $B=2^b$, this returns $b$.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::SignedDecomposerNonNative;
    /// use tfhe::core_crypto::commons::parameters::{
    ///     CiphertextModulus, DecompositionBaseLog, DecompositionLevelCount,
    /// };
    /// let decomposer = SignedDecomposerNonNative::new(
    ///     DecompositionBaseLog(4),
    ///     DecompositionLevelCount(3),
    ///     CiphertextModulus::try_new((1 << 64) - (1 << 32) + 1).unwrap(),
    /// );
    /// let val = 9_223_372_036_854_775_808u64;
    /// let decomp = decomposer.decompose(val);
    /// assert_eq!(decomp.base_log(), DecompositionBaseLog(4));
    /// ```
    pub fn base_log(&self) -> DecompositionBaseLog {
        DecompositionBaseLog(self.base_log)
    }

    /// Return the number of levels of this decomposition.
    ///
    /// If the decomposition uses $l$ levels, this returns $l$.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::SignedDecomposerNonNative;
    /// use tfhe::core_crypto::commons::parameters::{
    ///     CiphertextModulus, DecompositionBaseLog, DecompositionLevelCount,
    /// };
    /// let decomposer = SignedDecomposerNonNative::new(
    ///     DecompositionBaseLog(4),
    ///     DecompositionLevelCount(3),
    ///     CiphertextModulus::try_new((1 << 64) - (1 << 32) + 1).unwrap(),
    /// );
    /// let val = 9_223_372_036_854_775_808u64;
    /// let decomp = decomposer.decompose(val);
    /// assert_eq!(decomp.level_count(), DecompositionLevelCount(3));
    /// ```
    pub fn level_count(&self) -> DecompositionLevelCount {
        DecompositionLevelCount(self.level_count)
    }
}

impl<T> Iterator for SignedDecompositionIterNonNative<T>
where
    T: UnsignedInteger,
{
    type Item = DecompositionTermNonNative<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // The iterator is not fresh anymore
        self.fresh = false;
        // We check if the decomposition is over
        if self.current_level == 0 {
            return None;
        }
        // We decompose the current level
        let output = decompose_one_level_non_native(
            self.base_log,
            &mut self.state,
            self.mod_b_mask,
            T::cast_from(self.ciphertext_modulus.get_custom_modulus()),
            self.shift,
        );
        self.current_level -= 1;
        // We return the output for this level
        Some(DecompositionTermNonNative::new(
            DecompositionLevel(self.current_level + 1),
            DecompositionBaseLog(self.base_log),
            output,
            self.ciphertext_modulus,
        ))
    }
}

fn decompose_one_level_w_shift<S: UnsignedInteger>(
    base_log: usize,
    state: &mut S,
    mod_b_mask: S,
    shift: S,
) -> S {
    let res = *state & mod_b_mask;
    *state >>= base_log;
    let mut carry = (res.wrapping_sub(S::ONE) | *state) & res;
    carry >>= base_log - 1;
    *state += carry;
    res.wrapping_sub(carry * shift)
}

pub(crate) fn decompose_one_level_non_native<S: UnsignedInteger>(
    base_log: usize,
    state: &mut S,
    mod_b_mask: S,
    ciphertext_modulus: S,
    shift: S,
) -> S {
    let res = decompose_one_level_w_shift(base_log, state, mod_b_mask, shift);

    /// Takes as input a native unsigned integer interpreting it in a signed way, i.e. using the
    /// most significant bit as a sign bit and returns the `rem_euclid` using modulus
    /// Only works if abs(input) <= modulus / 2 which should always be verified in our case
    #[inline(always)]
    fn rem_euclid_native_to_custom_mod<Scalar: UnsignedInteger>(
        input: Scalar,
        modulus: Scalar,
    ) -> Scalar {
        // Here is a branching version of the code below which performed better in synthetic
        // benchmarks keyswitch Solinas q test in 61 seconds on dev laptop

        // // This branching version proved faster than a branchless alternative in synthetic
        // benchmarks let sign_bit = (input >> (Scalar::BITS - 1)) & Scalar::ONE;
        // if sign_bit == Scalar::ZERO {
        //     input
        // } else {
        //     // As the sign bit is set and the integers are/should be 2's complement the addition
        // is     // actually computing the subtraction which we want in that case
        //     modulus.wrapping_add(input)
        // }

        // This branchless version, though a bit heavier in terms of computation proved faster in
        // actual TFHE-rs test workloads
        // keyswitch Solinas q test in 46 seconds on dev laptop
        let sign_bit = (input >> (Scalar::BITS - 1)) & Scalar::ONE;
        let pos_res = (Scalar::ONE - sign_bit) * input;
        // As the sign bit is set and the integers are/should be 2's complement the addition is
        // actually computing the subtraction which we want in that case
        let neg_res = sign_bit * ((modulus).wrapping_add(input));
        // One of the result is 0 so we can OR rather than add both
        pos_res | neg_res
    }
    rem_euclid_native_to_custom_mod(res, ciphertext_modulus)
}
