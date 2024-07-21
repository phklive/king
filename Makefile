BACKEND_DIR = ./backend
FRONTEND_DIR = ./frontend

start-frontend:
	@echo "Starting frontend..."
	@cd $(FRONTEND_DIR) && npm run build && npm run start

start-backend:
	@echo "Starting backend..."
	@cd $(BACKEND_DIR) && RUST_LOG=info cargo run --release

help:
	@echo "Available commands:"
	@echo "  make start-frontend - Start the frontend (npm run dev)"
	@echo "  make start-backend  - Start the backend (cargo run --release with RUST_LOG=info)"

.PHONY: start-frontend start-backend start help
