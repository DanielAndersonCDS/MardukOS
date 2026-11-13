1. Unzip over the repo root (overwrites kernel/Cargo.toml, kernel/build.rs, kernel/src/main.rs; adds kernel/src/arch/).
2. In Makefile, change the `kernel:` recipe to:
       cargo build -p marduk-kernel $(if $(FEATURES),--features "$(FEATURES)",)
3. make run
   make run FEATURES=test-breakpoint | test-divide | test-pagefault | test-doublefault
