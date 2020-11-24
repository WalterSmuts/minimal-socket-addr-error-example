use mio::net::UnixDatagram;
use std::path::Path;

fn main() {
    let mut buff = [0u8; 1024];
    let path = Path::new("/tmp/arbitrary.sock");
    let socket_a = UnixDatagram::bind(path).unwrap();
    let socket_b = UnixDatagram::unbound().unwrap();

    socket_b.connect(path).unwrap();
    socket_b.send_to(&buff, path).unwrap();

    let (_size, addr) = socket_a.recv_from(&mut buff).unwrap();
    let _ = addr.as_pathname();
}
