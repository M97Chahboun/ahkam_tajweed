//! Rule detection modules for Tajweed
//!
//! This module contains sub-modules for detecting different categories of Tajweed rules:
//! - [`noon_mim`]: Noon Sakinah, Mim Sakinah, and Tanwin rules
//! - [`lam_al_tarif`]: Lam Al-Ta'rif (definite article) rules
//! - [`madd`]: Madd (vowel prolongation) rules
//! - [`qalqalah`]: Qalqalah (bouncing) rules
//! - [`ra`]: Ra emphasis and Allah name emphasis rules

pub mod lam_al_tarif;
pub mod madd;
pub mod noon_mim;
pub mod qalqalah;
pub mod ra;

pub use lam_al_tarif::detect_lam_al_tarif_rules;
pub use madd::detect_madd_rules;
pub use noon_mim::detect_noon_mim_rules;
pub use qalqalah::detect_qalqalah_rules;
pub use ra::{detect_allah_name_rules, detect_ra_rules};
