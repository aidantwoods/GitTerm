install: $(shell cargo install --path .)
	$(info )
	$(info Add the following line to ~/.zshrc to enable:)
	$(info )
	$(info $(SPACE) precmd() { eval $$(git-term) }';)
	$(info )

.PHONY: build
build: $(shell cargo build --release)
	$(info )
	$(info Add the following line to ~/.zshrc to enable:)
	$(info )
	$(info $(SPACE) precmd() {eval $$($(shell pwd)/target/release/git-term) };)
	$(info )