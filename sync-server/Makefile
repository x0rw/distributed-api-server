kill:
	-@kill $(shell lsof -t -i:1111) 2>/dev/null || true 
docker-build:
	sudo docker build -t rustapp .
docker-run:
	sudo docker run -p 1111:1111 rustapp
docker-restart:
	sudo systemctl restart docker

kill-port:
	@sudo docker exec -it rustapp sh -c "kill -9 \$$(lsof -t -i:1111)" || true
run:
	RUSTFLAGS='-A dead_code -A unused_variables' cargo run 

