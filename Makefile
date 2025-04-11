all:
	cargo check
node1:
	cargo run --bin sync_main node-1 5454 4441 8888 /echo
