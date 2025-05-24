crate_name := "typed_grid"
readme_src := "README.md"

readme:
    cp {{readme_src}} {{crate_name}}/README.md
    @echo "✅ Copied README.md to {{crate_name}}/README.md"

build: readme
    cargo build --workspace

test: readme
    # Doc tests runs separately, not yet supported in Nextest - https://nexte.st/#quick-start
    cargo nextest run --workspace --all-features
    cargo test --workspace --all-features --doc

clippy: readme
    cargo clippy --workspace --all-features --tests --bins --benches

clippy-no-color: readme
    cargo clippy --workspace --all-features --tests --bins --benches --color never

doc: readme
    cargo doc --workspace --bins --no-deps --document-private-items

doc-no-color: readme
    cargo doc --workspace --bins --no-deps --document-private-items --color never

release level: readme
    cargo release {{level}} --execute

clean:
    cargo clean
    rm {{crate_name}}/README.md
