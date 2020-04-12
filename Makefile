publish:
	@aws s3 sync ./wasm-demo/dist s3://aiku.wtf --acl public-read --follow-symlinks --delete
.PHONY: publish