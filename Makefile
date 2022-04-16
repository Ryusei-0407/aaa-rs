run:
	mold -run cargo +nightly run -q

test:
	mold -run cargo +nightly nextest run

lint:
	mold -run cargo +nightly clippy

debug:
	RUST_BACKTRACE=1 mold -run cargo +nightly nextest run
