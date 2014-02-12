ARGS="-O --lib -L ."
rm bin/*
mkdir -p src bin
cd src
rustc $ARGS libusb.rs
rustc $ARGS usb.rs
rustc $ARGS oblw.rs
rustc -O -L. outlet.rs
mv outlet ../bin
mv *so ../bin
cd ..
