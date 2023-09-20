FROM messense/rust-musl-cross:x86_64-musl as builder

WORKDIR /build
# Copy the source code
COPY . .
# Build the app
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a minimal runner
FROM scratch
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/guessing_game /gg
ENTRYPOINT [ "/gg" ]