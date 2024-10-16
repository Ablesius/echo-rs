#!/usr/bin/env bashf

CARGO_OPTS="-q"

# Test the end-to-end functionality of the program.
echo "DEBUG MODE"
echo "this should print 'Hello world!':"
cargo run $CARGO_OPTS -- Hello world!

echo ""

echo "RELEASE MODE"
echo "this should print 'Hello world!':"
cargo run $CARGO_OPTS --release -- Hello world!
