MEHR2 = {
    packages = {
        pacman = {
            "git",
            "picom",
            "fish",
            "imagemagick",
            "firefox",
            "flameshot",
            "pipewire",
            "dunst",
            "rofi",
            "i3",
            "acpi",
            "zathura",
            "curl",
            "base-devel",
            "pamixer",
            "hugo",
            "go",
            "ghostty",
            "rustup"
        },
        cargo = { "exa", "bat", "ripgrep", "yazi" },
        scratch = {
            {
                identifier = "rustup-tooling",
                needs = { "rustup" },
                script = [[
                    rustup component add rust-docs
                    rustup component add cargo
                    rustup component add clippy
                    rustup component add rustfmt
                ]]
            },
            {
                -- see: https://github.com/neovim/neovim/blob/master/BUILD.md
                identifier = "nvim",
                git = "github.com/neovim/neovim",
                needs = { "make", "cmake", "gcc" },
                branch = "nightly",
                script = [[
                    make CMAKE_BUILD_TYPE=Release
                    make install
                ]]
            },
        },
    },
}
