# Anyorigin

A very humble clone of [anyorigin.com](http://anyorigin.com/) or [whateverorigin.com](http://whateverorigin.com/)
written in [rust](http://rust-lang.org/) programming language.
Anyorigin lets overcome [Same-Origin-Policy problems](https://en.wikipedia.org/wiki/Same-origin-policy).

## Use It

Usage is similar to anyorigin (or whateverorigin). 
For example, to fetch the data from http://google.com with jQuery, use this snippet:

	$.getJSON('http://anyorigin.legeyda.com/get?url=' + encodeURIComponent('http://google.com') + '&callback=?', function(data){
		alert(data.contents);
	});



## Build

Anyorigin is written in [rust](http://rust-lang.org/) programming language, 
so you will need to have rust compiler and cargo package manager installed 
to build anyorigin.

Other tools you will need:

-	gcc

-	pgk-config

Build it with

	cd anyorigin
	make

which internally calls

	cargo build --release

to produce `target/release/anyorigin`-executable.

Sometimes (on x64 machines?) cargo complains about missing openssl headers and libraries, 
in that case make something like:

	find / -name 'hmac.h' 2> /dev/null # suppose finds $HOME/.cargo/registry/src/github.com-88ac128001ac3a9a/libressl-pnacl-sys-2.1.6/libressl/include/libssl/hmac.h
	export CPATH=$CPATH:$HOME/.cargo/registry/src/github.com-88ac128001ac3a9a/libressl-pnacl-sys-2.1.6/libressl/include 
	mkdir -p ~/.local/lib
	find / -name 'libssl*' -o -name 'libcrypto*' 2> /dev/null # suppose finds /usr/lib64/libssl.so.10 and /usr/lib64/libcrypto.so.10
	sudo cp /usr/lib64/libssl.so.10    ~/.local/lib/libssl.so
	sudo cp /usr/lib64/libcrypto.so.10 ~/.local/lib/libcrypto.so
	export LIBRARY_PATH=$LIBRARY_PATH:$HOME/.local/lib
	

## Install


sudo make install HOST=0.0.0.0 PORT=80

where `HOST` and `PORT` are host and port which
anyorigin will be listening to.

You can manage installed daemon with `systemctl`.

### Behind nginx

Anyorigin installer can configure nginx virtual server 
which will be proxying recieved requests to underliying running anyorigin daemon.
This allows to run anyorigin in shared environment.

	sudo make install-nginx-config HOST=127.0.0.1 PORT=49049 SERVER_HOST=example.com SERVER_PORT=80 

where `HOST` and `PORT` are as defined above as must match them;
`SERVER_HOST` and `SERVER_PORT` are host and port nginx will be responding to.



# note
cat build-pod.manifest | sed -e "s@\(\"source\"\s*:\s*\)\".\"@\1\"$(realpath .)\"@g"