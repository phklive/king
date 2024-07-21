start-frontend:
	cd frontend
	npm run dev

start-backend:
	./backend/ RUST_LOG=info cargo run --release
	
