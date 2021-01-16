


use hyper::Server;
use hyper::server::{Request, Response};
use hyper::Client;
use url;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::status::StatusCode;
use url::form_urlencoded::parse as parse_urlencoded;
use std::io::Read;
use std::io;
use hyper::client::Response as ClientResponse;
use hyper::error::Error as HyperError;
use escape_json::JsonEscaper;
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowCredentials};
use hyper::uri::RequestUri;
use url::SchemeData::Relative;
use url::Host::{Domain, Ipv4, Ipv6};
use std::net::{Ipv4Addr, Ipv6Addr};


static USAGE: &'static str = "<html>
	<head>
	<body>
		<h1>Anyorigin</h1>
		<p>A very humble clone of <a href='http://anyorigin.com/'>anyorigin.com</a> 
				or <a href='http://whateverorigin.org/'>whateverorigin.org</a></p>
		<p>Usage: </p>
		<p>
			<pre>/jsonp?url=http://google.com/&callback=func</pre>
			or <pre>/raw?url=http://google.com/</pre>
			or <pre>/get?url=http://google.com/&callback=func</pre>
		</p>
		<p>See <a href='http://bitbucket.org/legeyda/anyorigin'>project page</a> for details</p>
	<body>
</html>
";



header ! { (Origin, "Origin") => [String] }

pub fn request_url(url: &str) -> Result<Box<ClientResponse>, HyperError> {
	let client = Client::new();
	match client.get(url).send() {
		Ok(res) => {
			info!("request to {} ok", url);
			Ok(Box::new(res))
		},
		Err(e)  => {
			error!("error making request to {}: {}", url, e);
			Err(e)
		}
	}
}

pub fn respond_jsonp(request: &Request, response: Response, url: &str, callback: &str) {
	match request_url(url) {
		Ok(res) => {
			let status = res.status;
			let callback_string = callback.to_string() + "(\"";
			let reader0 = callback_string.as_bytes();
			let reader1 = reader0.chain(JsonEscaper::new(Box::new(res)));
			let mut reader2 = reader1.chain("\")".as_bytes());
			respond(request, response, status, &mut reader2);
		},
		Err(e) => {
			error!("error making request to {}: {}", url, e)
		}
	}
}



pub fn respond_raw(request: &Request, response: Response, url: &str) {
	match request_url(url) {
		Ok(res) => {
			let mut res = res;
			respond(request, response, res.status, &mut res);
		},
		Err(_) => {}
	}
}







pub fn respond_str(request: &Request, response: Response, status_code: StatusCode, body: &str) {
	let s = body.to_string();
	respond(request, response, status_code, &mut s.as_bytes());
}


pub fn respond(request: &Request, mut response: Response, status_code: StatusCode, body: &mut Read) {
	
	
	*response.status_mut() = status_code;
	match request.headers.get::<Origin>() {
		Some(header) => {
			let header_value: String = header.0.clone();
			response.headers_mut().set(AccessControlAllowOrigin::Value(header_value))
		},
		None => response.headers_mut().set(AccessControlAllowOrigin::Any)
	}
	response.headers_mut().set(AccessControlAllowCredentials);
	
	match response.start() {
        Ok(mut started_response) => {
        	match io::copy(body, &mut started_response) {
        		Ok(amount) => {
        			info!("{} bytes written to response", amount);
        		},
        		Err(e) => {
        			error!("error writing body: {}", e);
        		}	
        	}
        	match started_response.end() {
        		Ok(()) => {
        			info!("response <{}> sent", status_code);
        		},
        		Err(e) => {
        			error!("error sending response: {}", e);
        		}
        	}
        }
        Err(e) => {
            error!("error starting request: {}", e);
        }
    }
}

pub fn is_url_allowed(url_str: &str) -> bool {
	match url::Url::parse(url_str) {
		Ok(url) => {
			match url.scheme_data {
				Relative(rel_scheme_data) => {
					match rel_scheme_data.host {	
						Domain(host_str) => "localhost"!=host_str,
						Ipv4(ipv4_addr) => Ipv4Addr::new(127, 0, 0, 1)!=ipv4_addr,
						Ipv6(ipv6_addr) => Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)!=ipv6_addr
					}
				}
				_ => {
					error!("wrong url {}, relative expected", url_str);
					false
				}
			}
		},
		Err(e) => {
			error!("error parsing url {}: {}", url_str, e);
			false	
		}	
	}
}


pub fn handle(request: Request, response: Response) {
	let uri: RequestUri = request.uri.clone();
	match uri {
		AbsolutePath(s) => {
			match url::parse_path(&s) {
				Ok(arr) => {
					let (path, query, _) = arr;
					if path.len() == 0 || (path.len() == 1 && ("/" == path[0] || "" == path[0] )) {
						respond_str(&request, response, StatusCode::Ok, USAGE);
					} else if path.len()==1 && ("get" == path[0] || "jsonp" == path[0] || "raw" == path[0]) {
						match query {
							Some(query_string) => {
								let mut url:      Option<String> = None;
								let mut callback: Option<String> = None;
								for pair in parse_urlencoded(query_string.as_bytes()) {
									if "url" == pair.0 {
										url = Some(pair.1);
									} else if "callback" == pair.0 {
										callback = Some(pair.1);
									}
								}
								
								if url.is_some() {
									let url_str = url.clone().unwrap();	
									if !is_url_allowed(&url_str) {
										error!("wrong url {}", url_str);
										respond_str(&request, response, StatusCode::BadRequest, &*format!("url {} is forbidden", url_str));
										return;
									}
								}
								
								if "raw" == path[0] {
									if url.is_some() {
										let url_str = url.unwrap();
										respond_raw(&request, response, &url_str);
									} else {
										respond_str(&request, response, StatusCode::BadRequest, "param url must be set");	
									}
								} else {
									if url.is_some() && callback.is_some() {
										let url_str = url.unwrap();
										let callback_str = callback.unwrap();
										respond_jsonp(&request, response, &url_str, &callback_str);
									} else {
										respond_str(&request, response, StatusCode::BadRequest, "params url and callback must be set");	
									}
								}
							},
							None => {
								if "raw" == path[0] {
									respond_str(&request, response, StatusCode::BadRequest, "param url must be set");
								} else {
									respond_str(&request, response, StatusCode::BadRequest, "params url and callback must be set");
								}
							}
						}
					} else {
						respond_str(&request, response, StatusCode::NotFound, &*format!("resource {} not found", s));
					}
				},
				_ => {
					respond_str(&request, response, StatusCode::BadRequest, &*format!("unable parse path {}", s));
				}
			}
		},
		_ => {
			respond_str(&request, response, StatusCode::BadRequest, &*format!("wrong url, expected absolute path, given {}", request.uri));
		}
	}
}

pub fn start(address: &str) {
	match Server::http(address) {
		Ok(server) => {
			match server.handle(handle) {
				Ok(_) => {
					info!("started listening to {}", address);
				},
				Err(e) => {
					error!("error: {}", e)
				}
			}
		},
		Err(e) => {
			error!("error creating server on {}: {}", address, e)
		}
	}
}

