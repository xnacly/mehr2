MEHR2 = {
    packages = {
        default = {
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
        },
        pacman = { "base-devel", "pamixer", "hugo", "go", "ghostty" },
        cargo = { "exa", "bat", "ripgrep", "yazi" },
        scratch = {
            {
                identifier = "rustup",
                needs = { "curl" },
                update = "rustup update",
                script = [[
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
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
