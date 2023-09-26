#grapesjs directory
GRAPESJS_DIR = grapesjs-tailwind/

all: grapesjs install

grapesjs:
	@echo "Setting env variable..."
	export NODE_OPTIONS=--openssl-legacy-provider; \
	echo "Building GrapesJS..."; \
	cd $(GRAPESJS_DIR) && npm install && npm run build
	@echo "Done"

rustyroad:
	@echo "Building RustyRoad..."
	cargo build --release
	@echo "Done"
install:
	@echo "Installing RustyRoad..."
	cargo install --path .
	@echo "Done"

run_grapes:
	@echo "Setting env variable..."
	export NODE_OPTIONS=--openssl-legacy-provider; \
	echo "Running GrapesJS..."; \
	(cd $(GRAPESJS_DIR) && npm install && npm run start)
	@echo "Done"
