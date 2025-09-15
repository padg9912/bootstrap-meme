// src/embedding.rs

//! This module defines the core data structures for the Univalent Embedding Space.

// A 1746-bit integer, represented as an array of 28 u64s.
pub struct GodelNumber([u64; 28]);

// The full multivector representation for a term.
// For now, we will only include the scalar component.
pub struct CliffordMultivector {
    pub scalar: GodelNumber,
    // Vector, Bivector, and higher-grade components will be added later.
}

// The complete Univalent Embedding for a single term.
pub struct UnivalentEmbedding {
    pub term: String,
    pub reciprocal_harmonic: f64,
    pub multivector: CliffordMultivector,
}
