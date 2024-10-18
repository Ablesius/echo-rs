#!/usr/bin/env bashf

CARGO_OPTS="-q"

# Test the end-to-end functionality of the program.
echo "DEBUG MODE"
echo "this should print 'Hello world!':"
cargo run $CARGO_OPTS -- Hello world!
echo "this should print 'Helloworld!':"
cargo run $CARGO_OPTS -- -s Hello world!

echo ""

echo "RELEASE MODE"
echo "this should print 'Hello world!':"
cargo run $CARGO_OPTS --release -- Hello world!
echo "this should print 'Helloworld!':"
cargo run $CARGO_OPTS --release -- -s Hello world!
