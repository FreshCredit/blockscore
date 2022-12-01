FROM rust:1.65

COPY . /blockscore
WORKDIR /blockscore

# Install WebAssembly tools
RUN rustup target add wasm32-unknown-unknown
RUN rustup override set nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

#Install dependenties
RUN apt-get update
RUN apt install cmake -y
RUN apt install -y protobuf-compiler
RUN apt-get -y install clang

#Run build
RUN cargo build --release
RUN ./target/release/parachain-blockscore-node build-spec --disable-default-bootnode > plain-parachain-chainspec.json
RUN ./target/release/parachain-blockscore-node build-spec --chain plain-parachain-chainspec.json --disable-default-bootnode --raw > raw-parachain-chainspec.json
RUN ./target/release/parachain-blockscore-node export-genesis-state --chain raw-parachain-chainspec.json para-2001-genesis-stat
RUN ./target/release/parachain-blockscore-node export-genesis-wasm --chain raw-parachain-chainspec.json para-2001-wasm

#Start the app
CMD ./target/release/parachain-blockscore-node --bob --collator --force-authoring --chain raw-parachain-chainspec.json --base-path /tmp/parachain/bob --port 40334 --ws-port 8845 -- --execution wasm --chain rococo-custom-3-raw.json --port 30344 --ws-port 9978

#Expose ports
EXPOSE 40334
EXPOSE 8845
EXPOSE 30344
EXPOSE 9978
