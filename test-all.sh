#!/bin/bash
TRYBUILD=overwrite cargo hack test --feature-powerset --optional-deps \
  --group-features mass,length,duration,area,volume,speed,acceleration,force,energy,power,frequency,datavolume,datathroughput,temperature \
  --exclude-features doc,f32,f64
