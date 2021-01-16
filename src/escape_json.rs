use std::io::Read;
use std::iter::Iterator;
use std::io::Error;
use std::io::ErrorKind;



pub struct JsonEscaper {
	iter: Box<Iterator<Item=Result<u8, Error>>>,
	pending: Option<u8>
}

impl JsonEscaper {
	pub fn new(reader: Box<Read>) -> JsonEscaper {
		JsonEscaper{ iter: Box::new(reader.bytes()), pending: None }
	}
}

const BACKSLASH: u8 = 0x5c;

impl Read for JsonEscaper {
	
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
		if buf.len()==0 {
			return Ok(0);
		}

		match self.pending {
			Some(byte) => {
				buf[0]=byte;
				self.pending = None;
				return Ok(1)
			},
			None => {}
		};
		
		
		match self.iter.next() {
			Some(result) => {
				match result {
					Ok(byte) => {
						match byte {
							0x22 | 0x5c | 0x2f | 0x7f | 0xC | 0x0A | 0x0D | 0x09 => {
								buf[0]=BACKSLASH;
								if buf.len()>=2 {
									buf[0]=BACKSLASH;
									match byte {
										0x22 | 0x5c | 0x2f => { buf[1]=byte; },
										0x7f => { buf[1]=0x62; },
										0xC  => { buf[1]=0x66; },
										0x0A => { buf[1]=0x6e; },
										0x0D => { buf[1]=0x72; },
										0x09 => { buf[1]=0x74; },
										_ => {} // impossible
									};
									Ok(2)
								} else {
									self.pending=Some(buf[1]);
									Ok(1)
								}
							},
							_ => {
								if buf.len()>0 {
									buf[0]=byte;
									Ok(1)
								} else {
									Err(Error::new(ErrorKind::Other, "buffer too small"))
								}
							}
						}
					},
					Err(e) => Err(Error::new(ErrorKind::Other, e))
				}
			},
			None => Ok(0)
		}
	}
	
}
