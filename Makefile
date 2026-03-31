APP=slopfetch

all: build

build:
	cargo build --release

run:
	cargo run

install:
	cargo build --release
	cp target/release/$(APP) /usr/local/bin/$(APP)

clean:
	cargo clean
