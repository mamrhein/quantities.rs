Unit-safe computations with quantities.

### What is a Quantity?

"The value of a quantity is generally expressed as the product of a number
and a unit. The unit is simply a particular example of the quantity concerned
which is used as a reference, and the number is the ratio of the value of the
quantity to the unit." (Bureau International des Poids et Mesures: The
International System of Units, 8th edition, 2006)

**Basic** types of quantities are defined "by convention", they do not depend on
other types of quantities, for example Length, Mass or Duration.

**Derived** types of quantities, on the opposite, are defined as products of
other types of quantities raised by some exponent.

Examples:

* Volume = Length ^ 3

* Velocity = Length ^ 1 * Duration ^ -1

* Acceleration = Length ^ 1 * Duration ^ -2

* Force = Mass ^ 1 * Acceleration ^ 1

Each type of quantity may have one special unit which is used as a reference
for the definition of all other units, for example Meter, Kilogram and
Second. The other units are then defined by their relation to the reference
unit.

If a type of quantity is derived from types of quantities that all have a
reference unit, then the reference unit of that type is defined by a formula
that follows the formula defining the type of quantity.

Examples:

* Velocity -> Meter per Second = Meter ^ 1 * Second ^ -1

* Acceleration -> Meter per Second squared = Meter ^ 1 * Second ^ -2

* Force -> Newton = Kilogram ^ 1 * Meter ^ 1 * Second ^ -2


### "Systems of Measure"

There may be different systems which define quantities, their units and the
relations between these units in a different way.

This is not directly supported by this package. For each type of quantity there
can be only no or exactly one reference unit. But, if you have units from
different systems for the same type of quantity, you can define these units
and provide mechanisms to convert between them.

### The Basics: Quantity and Unit

The essential functionality of the package is provided by the two traits 
`Quantity` and `Unit` as well as the generic struct `Qty<U: Unit>`.

A **basic** type of quantity can easily be defined using the proc-macro
attribute `quantity`.

Example:

```rust
# use quantities::prelude::*;
#[quantity]
#[ref_unit(Kilogram, "kg", KILO)]
#[unit(Milligram, "mg", MILLI, 0.000001)]
#[unit(Carat, "ct", 0.0002)]
#[unit(Gram, "g", NONE, 0.001)]
#[unit(Ounce, "oz", 0.028349523125)]
#[unit(Pound, "lb", 0.45359237)]
#[unit(Stone, "st", 6.35029318)]
#[unit(Tonne, "t", MEGA, 1000.)]
/// The quantity of matter in a physical body.
struct Mass {}
```

In a future version, the macro will also allow to create a **derived** type of
quantity based on more basic types of quantities.

### Commonly Used Quantities

The package will provide definitions of many commonly used quantities; each
can be activated by a feature with a corresponding name.

