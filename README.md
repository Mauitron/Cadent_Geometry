# Cadent Geometry

**A discrete arc-space geometry where physics emerges from a single irreducable formula**

---

## One Formula. Everything Else Emerges.

```rust
d²/K
```

Where `K = 2^63`, and `d` is arc distance.

Cadent Geometry is the geometric framework I built on for my own rendering engine.
It is defined by three disconnected circles (h, v, t) with 2^64 discrete points each,
where curvature bends rates and rates move position. Gravity, acceleration,
and even black hole horizons can emerge naturally from the geometry itself.

### Information / disclaimers
I am working on a more thorough, in-depth way to show and explain the 
many different interesting oddities cadent geometry has to offer. This is for sharing
it with others, but also making it public, for attribution purposes. 

The images below, the one showing distance / depth and the one showing the
internals of the cadent black hole are taken from different versions of one of my demos. 
Everything is running on 1 core of a 7700x3D, running at 1080p and they are using 
~10.6 mb of ram.

The fps on the black hole one is clamped, so to be clear, it is about 500 fps
slower than the first one. So even if the behavior of is emergent, it IS much more
computationally expensive. 

It is also worth noting that, even though the geometry has many cool things
going for it. There are drawbacks. One of the major ones is the difficulty
to represent some basic geometries such as spheres accurately. 
At least, I find it horribly difficult at the moment, as it needs to be done as an
'observer sphere'. But I am crossing my fingers that smarter people than I can
help figure out ways to do it well before I do (looking at you who is reading this!)

Cadent allows you to cheat though. You can place euclidean objects in cadent space,
allowing you to at least make world distance be emergent, but the euclidean object
itself, in the world, still needs its local distance primitive, otherwise it cannot
express itself.

If you want to try cadent geometry out in graphics rendering. You should understand that
cadent does not work as one would expect. You can't treat things as being rendered from 
the sceen in towards the camera, it works by doing the drop from the camera out towards
the world. A good mental model for working with cadent is to think about the camera as being
two things:

1. the reference for defining the cadent arc meter
2. as an observer on a world that would exist without it. 

## Why Cadent Geometry?

Originally created to **avoid vector normalization** in graphics by working
in arc space instead of Euclidean space. by not using distance and scale and instead
only operate on angles and curvature.

When designing and exploring the geometry, I realized something i did not expect:
You can make physics-like behavior emerged naturally from the geometric primitives.
Particles accelerate toward regions of high curvature, rates saturate at boundaries,
and black hole-like horizons appear when curvature becomes extreme.

## Core Concepts

### Arc Space, Not Euclidean

Traditional graphics and physics work in Euclidean space with vectors that need constant normalization.
Cadent works in **arc space**, circular 'coordinates' where:
- All positions wrap (full u64 range)
- No square roots needed
- No normalization required
- Scale-independent by design

The fundamental differences between their observed geometries is, L1 or L2 is not conserved in Cadent.
Cadents sphere diagonal in euclidean mesurment is exactly 3/4, or .75, while a euclidean diagonal is
.707.

### The Drop Formula
<img width="2391" height="1353" alt="image" src="https://github.com/user-attachments/assets/e8282512-e5b5-4dd2-9365-fb2e4eb43260" />

The foundational irriducable operation in Cadent:

```rust
pub fn drop_at(d: u64) -> u128 {
    (d as u128) * (d as u128) >> HORIZON_K_SHIFT
}
```

This tells you how far a surface curves away at arc distance `d`. Everything else derives from this.
This is the formula that allows distance to emerge as a result, not a primitive

### Remaining Curvature Creates Gravity
![recording-20260216-172420](https://github.com/user-attachments/assets/f81da6d1-9612-4b8c-b378-ba536e29525f)

```rust
pub fn remaining_curv(d: u64) -> u128 {
    let rem = QUARTER - d;
    (rem as u128) * (rem as u128) >> HORIZON_K_SHIFT
}
```

Because scale is relative in Cadent, the curvature remaining from your position to the arc horizon,
is the gravitational field in Cadent geometry, maximum at center, zero at horizon. because you are
measuring the curvature of space

## Three-Circle Spacetime

A particle exists on three independent circles simultaneously:

```rust
pub struct Particle {
    pub h: u64,      // Horizontal circle position
    pub v: u64,      // Vertical circle position
    pub t: u64,      // Time circle position
    pub h_rate: i64, // Rate of change on h
    pub v_rate: i64, // Rate of change on v
    pub t_rate: i64, // Rate of change on t
}
```

**curvature on each circle bends rates on the other two circles.**

All geometry is derived from working with independent circles, physics appear when you couple them.
When a particle sits at position `h`, the remaining curvature at that position creates an acceleration
that bends `v_rate` and `t_rate`. This cross-coupling creates physics-like behavior.

## Black Holes Emerge from Cadent Geometry
Maybe not the most visually impressive example, as my engine is still primitive, 
but this is what the inside of a cadent black hole looks like at the moment.
Meant to show the visual representation of the behavior. 
<img width="2396" height="1351" alt="image" src="https://github.com/user-attachments/assets/9698593a-fb66-4cb9-bcd7-2571f0668f89" />


In Cadent, black holes aren't really added, they can emerge in a few different ways.
one is due to dimensioal collaps. when the curvature becomes so great that 2 circles overlap
creating an intersection where all angles point towards the region between them, or by the
following:

1. **High curvature** near circle centers creates extreme acceleration
2. **Rate saturation** when rates hit `i64::MAX` or `i64::MIN`, the particle is trapped
3. **Event horizon** the boundary where `M × curv(d) ≥ 2^64`

```rust
pub const GEOMETRIC_CRITICAL_MASS: u64 = 8;

pub fn is_inside_horizon(d: u64, mass: u64) -> bool {
    let curv = remaining_curv(d);
    curv > (u64::MAX as u128) / (mass as u128)
}
```

A particle with mass ≥ 8 creates a region where other particles cannot escape.
This falls directly out of the integer bounds and curvature formula.

## Usage

```rust
use cadent_geometry::*;

// Create a particle with initial rates
let mut particle = Particle::new(
    100_000,  // h_rate
    50_000,   // v_rate
    1_000     // t_rate
);

// Simulate with unit mass
for _ in 0..1000 {
    particle.tick();

    if particle.is_trapped() {
        println!("Particle trapped in geometric singularity!");
        break;
    }
}

// Or with custom mass
let mut heavy = Particle::new(0, 0, 1000);
for _ in 0..1000 {
    heavy.tick_with_mass(100);
}
```

### Circle Navigation

Convert facing direction to movement:

```rust
let facing: u64 = QUARTER; // 90 degrees
let step: u64 = 1000; // this is just a number for the example 

let (dh, dv) = circle_step(facing, step);
// Returns horizontal and vertical deltas
```

### Check for Horizons, even though you should not need too

```rust
let mass = 20;
if let Some(horizon_pos) = find_horizon(mass) {
    println!("Event horizon at arc position: {}", horizon_pos);
    println!("Radius in meters: {}", dp_to_meters(horizon_pos));
}
```

## Properties

### No std Required

Cadent is `#![no_std]` compatible, pure integer math, no allocations, no floating point.
works everywhere, from embedded systems, WASM, or deterministic simulations etc etc.

### Deterministic

All operations use wrapping/saturating integer arithmetic.
Same input = same output, always. No floating-point non-determinism.

### Scale Independent

Because it's arc-based, Cadent naturally handles any scale.
The `PLANET_METER_SHIFT` constant is what I used to create my planet, and it lets you map to
real-world units, but the geometry works at any scale.

## Is This Real Physics?

Cadent Geometry is not derived from general relativity or quantum mechanics.
It's an **mathematical framework** that produces similar physics-like behavior but
from the a geometry. I have found that things like snell's law has a natural, accurate,
way to be expressed in Cadent, But other things like the ACMB, not so much.
I have also gotten some good hints on EM emerging aswell. But I will share more on these things later.

Think of it as:
-  A consistent mathematical system
-  An usefull simulation framework
-  A seemingly novel approach to game physics or graphics
-  Creative exploration of discrete geometry


It's functional for graphics rendering. 

## Key Constants

```rust
K = 2^63               // The fundamental constant
QUARTER = 2^62         // Quarter circle (90°)
HALF = 2^63            // Half circle (180°)
RADIUS = 2^62          // Circle radius
PLANET_METER = 2^39    // Maps to real-world meters
```


## Possible Use Cases:

- **Unifying Game physics and geometry** All the same system for rendering
- **Procedural generation** using deterministic particle systems
- **Graphics** without vector normalization
- **Embedded simulations** on resource-constrained devices
- **Educational** exploration of alternative geometries is fun
- **Maybe Deterministic networking** (lockstep multiplayer)

## Contributing

This is exploratory work to solve real problems. If you find interesting emergent behaviors,
mathematical properties, or other novel applications, please feel free to open an issue or PR.

---

**"In arc space, distance is derivative. Curvature is primitive."**
