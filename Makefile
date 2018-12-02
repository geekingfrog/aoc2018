
pb1:
	cargo run --release 1 data/1.txt

pb2:
	cargo run --release 2 data/2.txt

test:
	cargo test --release

clean:
	cargo clean
