dev:
	cd frontend && trunk serve --open || true && cd -

lint:
	cd frontend && cargo clippy || true && cd -
