pushd ..
source env/bin/activate
export PATH=$PATH:$(echo $PWD/riscv64-*/bin/):~/.cargo/bin
popd

pushd stepper
make init
make synth
popd

# instead or regular UART, use UARBone + Crossover UART
# so build with --uart-name=crossover+uartbone
# then open litex_server with --uart --uart-port=/dev/ttyUSBX
# then litex_term crossover
# this should allow you to load/execute your firmware as you were doing over etherbone
# and you can also open litex_cli --gui in parallel
# which will allow you to write/read the register of the SoC
# registers
# and should also have a reboot button

python colorlight_5a_75x.py --build --revision 8.0 --csr-address-width 15 --with-ethernet --uart-name=crossover+uartbone --cpu-type vexriscv --cpu-variant imac --csr-svd colorlight.svd --csr-csv colorlight.csv

openFPGALoader -c ft2232 build/colorlight_5a_75b/gateware/colorlight_5a_75b.bit

pushd ../colorlight-pac/src
svd2rust -i ../../rust-colorlight/colorlight.svd --target riscv
popd

export BUILD_DIR=$PWD/build/colorlight_5a_75b
cargo build --release

# litex_server --udp
# cargo run --release