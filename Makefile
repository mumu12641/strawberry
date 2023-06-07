
GCC_EXIST := $(shell gcc --version 2>/dev/null)
CARGO_EXIST := $(shell cargo --version 2>/dev/null)

install:
	ifdef GCC_EXIST
		ifdef CARGO_EXIST
			$(shell cargo install --path .) 
		else
			@echo "No cargo in your PC"
		endif
	else
		@echo "No gcc in your PC, consider doing sudo apt install build-essential"
	endif
