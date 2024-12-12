.PHONY: enable-git-hooks
enable-git-hooks:
	git config --local core.hooksPath .githooks/

# cargo install zizmor
.PHONY: zizmor
zizmor:
	zizmor .github/workflows/run-benchmarks.yml
