# mpk -- makefile
SRC=lib bin crates ffi lisp tests
DOCS=README STYLE NOTES TODO
ORG=org/readme.org org/style.org org/todo.org org/notes.org
ROOT=$(dir $(abspath $(firstword $(MAKEFILE_LIST))))
STASH=~/stash


TARGET:=target/release
RF:=--release

UC=$(shell echo '$1' | tr '[:lower:]' '[:upper:]')
UN=$(shell uname)
FE=$(STASH)/fast-export/hg-fast-export.sh

# TODO
STATIC?=0
DEBUG?=0
PY_FFI?=1
CARGO?=
FFI?=0
_FFI?=libmpk_ffi
FFI_H?=mpk_ffi.h

ifeq ($(STATIC), 1)
  override FFI:=${_FFI:=.a}
else
  ifeq ($(UN), Darwin)
    override FFI:=${_FFI:=.dylib}
  else
    ifeq ($(UN), Linux)
      override FFI:=${_FFI:=.so}
    endif
  endif
endif

ifeq ($(DEBUG), 1)
  TARGET=target/debug
  RF=--debug
endif
  
.PHONY:install ffi
all:install ffi;
fmt:;cargo fmt $(CARGO)
clean:;cargo clean $(CARGO);rm -rf out build Cargo.lock
check:;cargo clippy $(CARGO)
test:;cargo test $(RF) $(CARGO)
bench:;cd tests/benches && cargo bench $(CARGO)
build:$(SRC);cargo build $(RF) $(CARGO)
install:$(SRC);cargo install $(CARGO)
out:;mkdir -p $@;
ffi:build;@cp $(TARGET)/$(_FFI) out/$(_FFI)
	ifeq ($(PY_FFI), 1) $(shell cp ffi/build.py out/build.py && cd out && python3 build.py) endif

ox:$(DOCS) $(ORG);$(foreach d,$(ORG),cp $(d) $(call UC,$(patsubst org/%.org,%,$(d));))
	emacs --eval '(let ((files (list "README" "STYLE" "TODO" "NOTES"))) \
	(dolist (f files) (find-file f) (mark-whole-buffer) (org-ascii-convert-region-to-utf8)) \
	(save-buffers-kill-terminal))'
mirror:$(FE) $(ROOT) out
	mkdir -p out/$@;
	git init out/$@;
	cd out/$@ && \
	git config core.ignoreCase false && git config push.followTags true && \
	$(FE) -r $(ROOT) -M default && git checkout HEAD && \
	git remote add gh git@github.com:richardwesthaver/mpk.git && \
	git push gh --all --force;
	@rm -rf out/$@;
