.PHONY: build install clean release aur

PREFIX ?= $(HOME)/.local
AUR_DIR ?= $(HOME)/aur/rofi-linear-git

build:
	cargo build --release

install: build
	install -Dm755 target/release/rofi-linear $(PREFIX)/bin/rofi-linear

uninstall:
	rm -f $(PREFIX)/bin/rofi-linear

clean:
	cargo clean

release:
	cargo build --release
	strip target/release/rofi-linear

aur:
	@echo "==> Preparing AUR package..."
	@mkdir -p $(AUR_DIR)
	@if [ ! -d "$(AUR_DIR)/.git" ]; then \
		git clone ssh://aur@aur.archlinux.org/rofi-linear-git.git $(AUR_DIR); \
	fi
	@cp PKGBUILD $(AUR_DIR)/
	@cd $(AUR_DIR) && makepkg --printsrcinfo > .SRCINFO
	@cd $(AUR_DIR) && git add PKGBUILD .SRCINFO
	@cd $(AUR_DIR) && git commit -m "Update to $(shell grep pkgver= PKGBUILD | head -1 | cut -d= -f2)"
	@cd $(AUR_DIR) && git push
	@echo "==> Published to AUR!"
