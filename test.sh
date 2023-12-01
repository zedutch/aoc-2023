#! /bin/bash

cargo watch -q -c -x  "test $1 -- --nocapture"
