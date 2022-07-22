mod network;

fn main() {
    let mut nc: network::Stream = network::Stream::new();
    nc.pt = network::PType::Ping;
    nc.write();
}
