//! Cadent Geometry and Physics: The Three-Circle Spacetime
//! By Maui_The_Magnificent (Mauritz Nyfeldt)
//!
//! One formula: d²/K. Everything else emerges.
//!
//! Three circles: h, v, t. Each has 2^64 discrete points.
//! Curvature bends rates. Rates move position.
//! Even black holes emerge from arc space.

#![no_std]

// Cadent geometry is comprised of one fundamental constant (K), and one
// fundamental irreducible formula (d²/K). There are no other primitives.
//
// It was initially created to remove the need for vector normalization in graphics.
// It solves this by being an arc space geometry, making it distance and scale
// independent. A lot of physics seems to fall out from Cadent geometry aswell,
// which allows it to have gravity, acceleration, singularities and so on, as
// geometric expressions derived from the topology rather than needing seperate
// explainations.
// =============================================================================
// Constants: Everything derives from K = 2^63
// =============================================================================

// I extracted the geometry, there are some pre-defined constants
// and other artifacts that are not explicitly part of the geometry
// definition itself, but are left in as they are both practically useful
// and for context.
pub const QUARTER: u64 = 1u64 << 62;
pub const HALF: u64 = 1u64 << 63;
pub const HORIZON_K_SHIFT: u32 = 63;
pub const HORIZON_K: u64 = 1u64 << HORIZON_K_SHIFT;
pub const RADIUS_SHIFT: u32 = 62;
pub const RADIUS: u64 = 1u64 << RADIUS_SHIFT;
pub const FACING_K_SHIFT: u32 = 62;
pub const FACING_K: u64 = 1u64 << FACING_K_SHIFT;
pub const PLANET_METER_SHIFT: u32 = 39;
pub const PLANET_METER: u64 = 1u64 << PLANET_METER_SHIFT;

// =============================================================================
// The Foundational Cadent Formula: d²/K
// =============================================================================

//  Even though Cadent geametry does not have distance as a primitive, it can be
//  emergent by using the cadent drop formula.
/// Drop: d²/K. gives you how far the surface curves away.
#[inline]
pub fn drop_at(d: u64) -> u128 {
    (d as u128) * (d as u128) >> HORIZON_K_SHIFT
}

/// Cam: R - drop. height above the curve.
#[inline]
pub fn cam_at(d: u64) -> u128 {
    let r = RADIUS as u128;
    let drop = drop_at(d);
    if r > drop { r - drop } else { 0 }
}

/// remaining curvature: (Q-d)²/K. Same formula, measured from horizon.
/// maximum at center, zero at horizon. This is the cadent gravity.
#[inline]
pub fn remaining_curv(d: u64) -> u128 {
    if d < QUARTER {
        let rem = QUARTER - d;
        (rem as u128) * (rem as u128) >> HORIZON_K_SHIFT
    } else {
        0
    }
}

// =============================================================================
// Circle Geometry
// =============================================================================

/// Fold to first quadrant. Circle symmetry.
#[inline]
pub fn fold_to_quarter(v: u64) -> u64 {
    let v = if v > HALF { 0u64.wrapping_sub(v) } else { v };
    if v > QUARTER { HALF.wrapping_sub(v) } else { v }
}

/// Use drop to turn facing into (dh, dv).
#[inline]
pub fn circle_step(facing: u64, step: u64) -> (i64, i64) {
    let r = RADIUS as u128;

    // v-circle component
    let v_arc = if facing <= HALF { facing } else { 0u64.wrapping_sub(facing) };
    let v_near = if v_arc <= QUARTER { v_arc } else { HALF.wrapping_sub(v_arc) };
    let v_drop = (v_near as u128) * (v_near as u128) >> FACING_K_SHIFT;
    let v_cam = if r > v_drop { r - v_drop } else { 0 };
    let v_sign: i64 = if v_arc <= QUARTER { 1 } else { -1 };
    let dv = v_sign * (step as u128 * v_cam >> RADIUS_SHIFT) as i64;

    // h-circle component
    let h_offset = facing.wrapping_sub(QUARTER);
    let h_arc = if h_offset <= HALF { h_offset } else { 0u64.wrapping_sub(h_offset) };
    let h_near = if h_arc <= QUARTER { h_arc } else { HALF.wrapping_sub(h_arc) };
    let h_drop = (h_near as u128) * (h_near as u128) >> FACING_K_SHIFT;
    let h_cam = if r > h_drop { r - h_drop } else { 0 };
    let h_sign: i64 = if facing > 0 && facing < HALF { 1 } else { -1 };
    let dh = h_sign * (step as u128 * h_cam >> RADIUS_SHIFT) as i64;

    (dh, dv)
}

/// Cadent odd series: 2t-1. Sum of first n = n².
#[inline]
pub fn odd_increment(t: u64) -> u64 {
    t.wrapping_mul(2).wrapping_sub(1)
}

// =============================================================================
// Particle, Three circles, curvature bends rates
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub h: u64,
    pub v: u64,
    pub t: u64,
    pub h_rate: i64,
    pub v_rate: i64,
    pub t_rate: i64,
}

impl Particle {
    pub fn new(h_rate: i64, v_rate: i64, t_rate: i64) -> Self {
        Self { h: 0, v: 0, t: 0, h_rate, v_rate, t_rate }
    }

    pub fn at(h: u64, v: u64, t: u64, h_rate: i64, v_rate: i64, t_rate: i64) -> Self {
        Self { h, v, t, h_rate, v_rate, t_rate }
    }

    pub fn tick(&mut self) {
        self.tick_with_mass(1)
    }

    pub fn tick_with_mass(&mut self, mass: u64) {
        // The arc curvature on each circle
        let h_curv = remaining_curv(fold_to_quarter(self.h));
        let v_curv = remaining_curv(fold_to_quarter(self.v));
        let t_curv = remaining_curv(fold_to_quarter(self.t));

        // Direction toward center on each of the arc circles
        let h_sign: i64 = if self.h == 0 { 0 } else if self.h <= HALF { -1 } else { 1 };
        let v_sign: i64 = if self.v == 0 { 0 } else if self.v <= HALF { -1 } else { 1 };
        let t_sign: i64 = if self.t == 0 { 0 } else if self.t <= HALF { -1 } else { 1 };

        // scale the curvature to acceleration
        let scale = HORIZON_K_SHIFT - 13;
        let h_curv_s = ((h_curv >> scale) as u64).saturating_mul(mass);
        let v_curv_s = ((v_curv >> scale) as u64).saturating_mul(mass);
        let t_curv_s = ((t_curv >> scale) as u64).saturating_mul(mass);

        // Each of the rates bent by other two curvatures
        let h_accel = (v_curv_s.saturating_add(t_curv_s)) as i64;
        let v_accel = (h_curv_s.saturating_add(t_curv_s)) as i64;
        let t_accel = (h_curv_s.saturating_add(v_curv_s)) as i64;

        // Apply the acceleration toward center
        self.h_rate = self.h_rate.saturating_add(h_sign.saturating_mul(h_accel));
        self.v_rate = self.v_rate.saturating_add(v_sign.saturating_mul(v_accel));
        self.t_rate = self.t_rate.saturating_add(t_sign.saturating_mul(t_accel));

        // just move
        self.h = self.h.wrapping_add(self.h_rate as u64);
        self.v = self.v.wrapping_add(self.v_rate as u64);
        self.t = self.t.wrapping_add(self.t_rate as u64);
    }

    /// geomenty, rate saturation = trapped = black hole
    pub fn is_trapped(&self) -> bool {
        self.h_rate == i64::MAX || self.h_rate == i64::MIN
            || self.v_rate == i64::MAX || self.v_rate == i64::MIN
            || self.t_rate == i64::MAX || self.t_rate == i64::MIN
    }

    pub fn rate_squared(&self) -> u128 {
        let h2 = (self.h_rate as i128) * (self.h_rate as i128);
        let v2 = (self.v_rate as i128) * (self.v_rate as i128);
        let t2 = (self.t_rate as i128) * (self.t_rate as i128);
        (h2 + v2 + t2) as u128
    }

    pub fn distance_from_center(&self) -> u128 {
        fold_to_quarter(self.h) as u128
            + fold_to_quarter(self.v) as u128
            + fold_to_quarter(self.t) as u128
    }
}

// =============================================================================
// Black Hole should emerge from geometry
// =============================================================================

/// M × curv(0) ≥ 2^64 → M ≥ 8
pub const GEOMETRIC_CRITICAL_MASS: u64 = 8;

/// Is position d inside the horizon for mass M?
/// Checks: M × curv(d) ≥ 2^64
#[inline]
pub fn is_inside_horizon(d: u64, mass: u64) -> bool {
    if mass == 0 { return false; }
    let curv = remaining_curv(d);
    curv > (u64::MAX as u128) / (mass as u128)
}

//  this is a somewhat forced horizon, 
/// Find horizon by walking outward. Returns first position outside.
pub fn find_horizon(mass: u64) -> Option<u64> {
    if !is_inside_horizon(0, mass) { return None; }

    let mut lo: u64 = 0;
    let mut hi: u64 = QUARTER;

    while lo < hi {
        let mid = lo + ((hi - lo) >> 1);
        if is_inside_horizon(mid, mass) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    Some(lo)
}

// =============================================================================
// Utility
// =============================================================================

#[inline]
pub fn dp_to_meters(dp: u64) -> u64 {
    dp >> PLANET_METER_SHIFT
}

#[inline]
pub fn meters_to_dp(meters: u64) -> u64 {
    meters << PLANET_METER_SHIFT
}
