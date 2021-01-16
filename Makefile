VERSION=$(shell cat VERSION)
OS:=$(shell uname | tr '[:upper:]' '[:lower:]')
ARCH:=$(shell uname -m | sed -e 's/^\(x64\|x86_64\)$$/amd64/g' | tr '[:upper:]' '[:lower:]')
EXT:=aci


TARGET=target
prefix?=/usr/local
bindir?=${prefix}/bin
libdir?=${prefix}/lib

SOURCES=$(shell find src -name '*.rs')
EXECUTABLE=${TARGET}/release/anyorigin
IMAGE=${TARGET}/anyorigin-${VERSION}-${OS}-${ARCH}.${EXT}
ROOTFS=${TARGET}/.acbuild/currentaci/rootfs
AC_DISCOVERY=target/ac-discovery
ROOTFS=${TARGET}/.acbuild/currentaci/rootfs
EXECUTABLE=${TARGET}/release/anyorigin
ACI=${TARGET}/anyorigin-${VERSION}.aci

# port anyorigin will be listening
PORT?=49049

# host anyorigin will be listening
HOST?=0.0.0.0

# log level
LOG_LEVEL?=WARN

# port any origin will be accessible through nginx
NGINX_PORT?=80

# host any origin will be accessible through nginx
NGINX_HOST?=anyorigin


SED_REPLACE_COMMAND= s={{VERSION}}=${VERSION}=; s={{PREFIX}}=${prefix}=; s={{PORT}}=${PORT}=; s={{HOST}}=${HOST}=;s={{NGINX_HOST}}=${NGINX_HOST}=; s={{NGINX_PORT}}=${NGINX_PORT}=;

ACBUILD=acbuild --work-path ${TARGET}
ACBUILD_RUN=${ACBUILD} run --insecure=true
RKT=rkt --insecure-options=image


.PHONY: all
all: native-build


.PHONY: clean
clean:
	rm -rf ${TARGET} ${CARGO}


Cargo.toml: Cargo.toml.in
	cat $< | sed -e 's|{{VERSION}}|${VERSION}|g' > $@


.PHONY: native-build
native-build: Cargo.toml ${SOURCES}
	cargo build --release


.PHONY: container-build
container-build: Cargo.toml ${SOURCES}
	cat build.rkt-args | sed -e "s|source\s*=\s*.|source=$(realpath .)|g" | sudo xargs --delimiter='\n' --max-lines=999 rkt


${EXECUTABLE}:
	$(MAKE) native-build


.PHONY: install
install: ${EXECUTABLE}
	install -D $< ${DESTDIR}${bindir}
	cat etc/anyorigin.service.in | sed -e "${SED_REPLACE_COMMAND}" > ${TARGET}/anyorigin.service
	install -D ${TARGET}/anyorigin.service ${DESTDIR}/etc/systemd/system
	@echo 'systemd service installed. run "systemctl enable anyorigin.service" to enable it'


.PHONY: install-nginx-config
install-nginx-config: 
	cat etc/anyorigin.conf.in | sed -e "${SED_REPLACE_COMMAND}" > ${TARGET}/anyorigin.conf
	install -D ${TARGET}/anyorigin.conf ${DESTDIR}${prefix}/etc/nginx/conf.d
	@echo 'nginx config installed. restart nginx to activate it'


.PHONY: uninstall
uninstall:
	rm -f "${DESTDIR}${bindir}/anyorigin" "${DESTDIR}/etc/systemd/system/anyorigin.service" "${DESTDIR}${prefix}/etc/nginx/conf.d/anyorigin.conf"


.PHONY: build-image
build-image: ${IMAGE}


${IMAGE}: ${EXECUTABLE}
	mkdir -p ${TARGET}
	cat acbuild-script | sed -e 's|{{VERSION}}|${VERSION}|g; s|{{OS}}|${OS}|g; s|{{ARCH}}|${ARCH}|g; s|{{OUTPUT}}|$@|g;' > ${TARGET}/acbuild-script
	acbuild --work-path ${TARGET} script ${TARGET}/acbuild-script


.PHONY: install-image
install-image: ${ACI}
	${RKT} fetch 'file://$(realpath $<)'


.PHONY: uninstall-image
uninstall-image:
	while ${RKT} image rm legeyda.com/anyorigin:${VERSION} | grep 'successfully removed'; do echo image deleted; done

