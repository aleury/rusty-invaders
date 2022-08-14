clean:
	cargo clean
	rm -rf builds

build-macos:
	cargo build --release 

build-windows:
	docker build . -t invaders/windows -f Dockerfile.windows
	docker run --rm -ti -v `pwd`:/app invaders/windows	

pkg-windows:
	mkdir -p ./builds/invaders-windows
	cp -r ./sounds ./builds/invaders-windows
	cp ./target/x86_64-pc-windows-gnu/release/invaders.exe ./builds/invaders-windows
	cd ./builds && zip -vr ./invaders-windows.zip ./invaders-windows
	cd ..

pkg-macos:
	mkdir -p ./builds/invaders-macos
	cp -r ./sounds ./builds/invaders-macos
	cp ./target/release/invaders ./builds/invaders-macos
	cd ./builds && zip -vr ./invaders-macos.zip ./invaders-macos
	cd ..