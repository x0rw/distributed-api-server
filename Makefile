all:
	cargo check
node1:
	cargo run --bin sync_main node-1 5454 4441 8888 /echo

node2:
	cargo run --bin sync_main node-2 5353 4442 8888 /posts

node3:
	cargo run --bin sync_main node-3 5555 4443 8888 /help

node4:
	cargo run --bin sync_main node-4 5656 4444 8888 /ping

gateway:
	cargo run --bin gateway
	
