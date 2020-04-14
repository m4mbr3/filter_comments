FROM rustembedded/cross:x86_64-pc-windows-gnu

RUN apt install -y qtbase5-dev
RUN apt install -y qt5-qmake
RUN apt install -y qt5-default
RUN apt install -y qtbase5-dev
RUN apt install -y pacapt

# Add martchus.no-ip.biz repo for mingw binaries
RUN    echo "[ownstuff]" >> /etc/pacapt.conf \
    && echo "SigLevel = Optional TrustAll" >> /etc/pacapt.conf \
    && echo "Server = https://martchus.no-ip.biz/repo/arch/\$repo/os/\$arch " >> /etc/pacapt.conf \
    && pacapt -Sy

# Add mingw-repo
RUN    echo "[mingw-w64]" >> /etc/pacapt.conf \
    && echo "SigLevel = Optional TrustAll" >> /etc/pacapt.conf \
    && echo "Server = http://downloads.sourceforge.net/project/mingw-w64-archlinux/\$arch" >> /etc/pacapt.conf \
    && pacapt -Sy

RUN pacapt -S  mingw-w64-qt5-base \
        mingw-w64-qt5-base-static \
        mingw-w64-qt5-connectivity \
        mingw-w64-qt5-charts \
        mingw-w64-qt5-datavis3d \
        mingw-w64-qt5-declarative \
        mingw-w64-qt5-gamepad \
        mingw-w64-qt5-graphicaleffects \
        mingw-w64-qt5-imageformats \
        mingw-w64-qt5-location \
        mingw-w64-qt5-multimedia \
        mingw-w64-qt5-networkauth \
        mingw-w64-qt5-quickcontrols \
        mingw-w64-qt5-quickcontrols2 \
        mingw-w64-qt5-script \
        mingw-w64-qt5-scxml \
        mingw-w64-qt5-sensors \
        mingw-w64-qt5-serialport \
        mingw-w64-qt5-svg \
        mingw-w64-qt5-virtualkeyboard \
        mingw-w64-qt5-webkit \
        mingw-w64-qt5-webchannel \
        mingw-w64-qt5-webglplugin \
        mingw-w64-qt5-websockets \
        mingw-w64-qt5-winextras \
        mingw-w64-qt5-xmlpatterns

