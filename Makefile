build:
	dx build --release
	cp CNAME docs
	cp .nojekyll docs
