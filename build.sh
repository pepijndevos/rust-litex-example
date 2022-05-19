pushd ..
source env/bin/activate
export PATH=$PATH:$(echo $PWD/riscv64-*/bin/):~/.cargo/bin
popd

pushd stepper
make init
make synth
popd

python colorlight_5a_75x.py --build --revision 8.0 --with-ethernet --cpu-type vexriscv --cpu-variant imac --csr-svd colorlight.svd --csr-csv colorlight.csv

openFPGALoader -c ft2232 build/colorlight_5a_75b/gateware/colorlight_5a_75b.bit

pushd ../colorlight-pac/src
svd2rust -i ../../rust-colorlight/colorlight.svd --target riscv
popd

export BUILD_DIR=$PWD/build/colorlight_5a_75b
cargo build --release

# litex_server --udp
# cargo run --release