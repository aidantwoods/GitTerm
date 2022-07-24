install: $(shell cargo install --path .)
	$(info )
	$(info Add the following line to ~/.bash_profile to enable:)
	$(info )
	$(info $(SPACE) PROMPT_COMMAND='PS1=$$(git-term)';)
	$(info )

.PHONY: build
build: $(shell cargo build --release)
	$(info )
	$(info Add the following line to ~/.bash_profile to enable:)
	$(info )
	$(info $(SPACE) PROMPT_COMMAND='PS1=$$($(shell pwd)/target/release/git-term)';)
	$(info )