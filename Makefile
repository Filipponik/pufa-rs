version=1.1.0
image=filipponik/pufa-rs

# Build docker images
build:
	docker build . -t $(image_name):$(version) -t $(image_name):latest

# Run tests
test:
	cargo test