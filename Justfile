crate_name := "typed_grid"
readme_src := "README.md"

readme:
    cp {{readme_src}} {{crate_name}}/README.md
    @echo "✅ Copied README.md to {{crate_name}}/README.md"

build: readme
    cargo build --workspace

test: readme
    cargo test --workspace

release level: readme
    cd {{crate_name}} && cargo release {{level}} --execute

clean:
    cargo clean
    rm {{crate_name}}/README.md
