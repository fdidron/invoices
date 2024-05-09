# Runs the cargo watch run command for development
dev-rust:
	cargo watch -x run
# Runs tailwindcss in watch mode
dev-tailwind:
	pnpx tailwindcss -i ./src/app.css -o ./static/app.css --watch

# Runs the development server and tailwindcss in watch mode
# Note: This command must be run with the -j 2 flag to run both commands in parallel
dev: dev-tailwind dev-rust
