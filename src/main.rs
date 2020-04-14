enum PacketType {
	Motion,
	Session,
	Lap,
	Event,
	Participant,
	Setup,
	Telemetry,
	CarStatus
}

fn main() {
	let mut buf = [0u8; 1347]; 	//Max packet size
    loop {
    	//attempt_packet_read;
    	//write unprocessed packet to buffer;
    }
}
