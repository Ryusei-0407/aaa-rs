run:
	mold -run cargo run -q

test:
	mold -run cargo nextest run

lint:
	mold -run cargo clippy

debug:
	RUST_BACKTRACE=1 mold -run cargo nextest run
