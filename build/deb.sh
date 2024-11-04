# https://www.debian.org/doc/debian-policy/ch-controlfields.html
mkdir -p release/deb/pcaps_0.0.1/usr/local/bin
mkdir -p release/deb/pcaps_0.0.1/DEBIAN/
cp target/release/pcaps release/deb/pcaps_0.0.1/usr/local/bin
echo "Package: pcaps
Version: 0.0.1
Section: base
Priority: optional
Architecture: amd64
Maintainer: sankooc <sankooc@163.com>
Description: pcap/pcapng analyzer" >> release/deb/pcaps_0.0.1/DEBIAN//control
cd release/deb
dpkg-deb --build pcaps_0.0.1

