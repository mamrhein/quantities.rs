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

* Volume = Length ³

* Velocity = Length ¹ · Duration ⁻¹

* Acceleration = Length ¹ · Duration ⁻²

* Force = Mass ¹ · Acceleration ¹

Each type of quantity may have one special unit which is used as a reference
for the definition of all other units, for example Meter, Kilogram and
Second. The other units are then defined by their relation to the reference
unit.

If a type of quantity is derived from types of quantities that all have a
reference unit, then the reference unit of that type is defined by a formula
that follows the formula defining the type of quantity.

Examples:

* Velocity -> Meter per Second = Meter ¹ · Second ⁻¹

* Acceleration -> Meter per Second squared = Meter ¹ · Second ⁻²

* Force -> Newton = Kilogram ¹ · Meter ¹ · Second ⁻²


### "Systems of Measure"

There may be different systems which define quantities, their units and the
relations between these units in a different way.

This is not directly supported by this package. For each type of quantity
there can be only no or exactly one reference unit. But, if you have units
from different systems for the same type of quantity, you can define these
units and provide mechanisms to convert between them.

### The Basics: Quantity and Unit

The essential functionality of the package is provided by the two traits 
`Quantity` and `Unit` as well as the generic struct `Qty<U: Unit>`.

A **basic** type of quantity can easily be defined using the proc-macro
attribute `quantity`, optionally followed by an attribute `refunit` and
followed by at least one attribute `unit`.

The macro generates an enum with the given units (incl. the refunit, if given)
as variants, an implemention of trait `Unit` for this enum and a type alias of
`Qty` with the enum as parameter and named after the given struct.

In addition, it creates a constant for each enum variant, thus providing a
constant for each unit. This implies that the identifiers of all units over
all defined quantitities have to be unique!

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

assert_eq!(MILLIGRAM.name(), "Milligram");
assert_eq!(POUND.symbol(), "lb");
assert_eq!(TONNE.si_prefix(), Some(SIPrefix::MEGA));
assert_eq!(CARAT.scale(), Some(Amnt!(0.0002)));
```

In a future version, the macro will also allow to create a **derived** type of
quantity based on more basic types of quantities.

### Instantiating quantities

An instance of a quantity type can be created by calling the function `new`,
giving an amount and a unit. Alternatively, a unit can be multiplied by an
amount.

Example:

```rust
# use quantities::prelude::*;
# #[quantity]
# #[ref_unit(Kilogram, "kg", KILO)]
# #[unit(Gram, "g", NONE, 0.001)]
# struct Mass {}
let m = Mass::new(Amnt!(17.4), GRAM);
assert_eq!(m.to_string(), "17.4 g");
let m = Amnt!(17.4) * GRAM;
assert_eq!(m.to_string(), "17.4 g");
```

### Unit-safe computations

If the quantity type has a refernce unit, a quantity instance can be converted
to a quantity instance with a different unit of the same type by calling the
method `convert`.

Example:

```rust
# use quantities::prelude::*;
# #[quantity]
# #[ref_unit(Kilogram, "kg", KILO)]
# #[unit(Carat, "ct", 0.0002)]
# #[unit(Gram, "g", NONE, 0.001)]
# struct Mass {}
let x = Mass::new(Amnt!(13.5), GRAM);
let y = x.convert(CARAT).unwrap();
assert_eq!(y.to_string(), "67.5 ct");
```

### Commonly Used Quantities

The package will provide definitions of many commonly used quantities; each
can be activated by a feature with a corresponding name.

