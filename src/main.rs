
use std::net::UdpSocket;
use byte::*;

const BUFFER_SIZE: usize = 1347;						//Max packet size to spec
const DEFAULT_SOCKET_BINDING: &str = "0.0.0.0:34620";
const HEADER_OFFSET: usize = 5;

enum PacketType {
	MOTION,
	SESSION,
	LAP,
	EVENT,
	PARTICIPANT,
	SETUP,
	TELEMETRY,
	CARSTATUS
}

struct PacketHeader {   
    packet_format: 		u16,		// 2019
    maj_version: 		u8,			// Game major version - "X.00"
    min_version: 		u8,			// Game minor version - "1.XX"
    packet_version: 	u8,			// Version of this packet type, all start from 1
    packet_id: 			u8,			// Identifier for the packet type, see below
    packet_type:		PacketType,	// Might use enum or not.... this would replace the above packet_id
    session_id: 		u64,		// Unique identifier for the session
    session_time: 		f32,		// Session timestamp
    frame_id:			u16,  		//Identifier for the frame the data was retrieved on. JP 2020 this spec had typo 'uint' guessing 16 bit as 8 bit not enough to contain num frames in game - could be u64 but seems excessive.
    player_car_index: 	u8			// Index of player's car in the array
}

struct CarTelemetryData {				//65 bytes?
	header:				PacketHeader,
    car_speed: 			u16,                    // Speed of car in kilometres per hour
    throttle_pos: 		f32,                    // Amount of throttle applied (0.0 to 1.0)
    steering_pos:		f32,                    // Steering (-1.0 (full lock left) to 1.0 (full lock right))
    brake_pos: 			f32,                    // Amount of brake applied (0.0 to 1.0)
    clutch_pos:			u8,						// Amount of clutch applied (0 to 100)         
    gear: 				i8,						// Gear selected (1-8, N=0, R=-1)
    engine_rpm:			u16,					// Engine RPM
    drs_active: 		u8,						// 0 = off, 1 = on
    change_light_perc: 	u8,						// Rev lights indicator (percentage)
    brake_temp: 		[u16; 4],				// Brakes temperature (celsius)
    tyre_surface_temps: [u16; 4],				// Tyres surface temperature (celsius)
    tyre_inner_temp: 	[u16; 4],				// Tyres inner temperature (celsius)
    engine_temp: 		u16,					// Engine temperature (celsius)
    tyre_pressures: 	[u16; 4], 				// Tyres pressure (PSI)
    tyre_contact_type: 	[u8; 4] 				// Driving surface, see appendices
}

fn main() {
	let mut buf = [0u8; BUFFER_SIZE]; 	
    let socket = UdpSocket::bind(DEFAULT_SOCKET_BINDING).expect("failed to bind to socket");
    let mut bytes_recevied: usize = 0;
    loop {
    	let bytes_recevied = match socket.recv(&mut buf){
    		Ok(bytes) => bytes,
    		Err(_e) => 0
    	};
    	if let Some(h) = parse_header(&buf, bytes_recevied) {
    		//parse packet
    	} else {
    		println!("ignoring weird packet");
    	}
    }
}

fn parse_header(buf: &[u8; BUFFER_SIZE], len: usize) -> Option<PacketHeader> {
	let offset: usize = 5;
	let packet_type: u8 = match buf.read_with(&mut HEADER_OFFSET, LE).unwrap(){
		Some(p) => PacketHeader {
			
		},
		_ => None 
	}
}

fn get_packet_type(parsed_byte: u8) -> Option<PacketType> {
	match parsed_byte {
		0 => Some(PacketType::MOTION),
		1 => Some(PacketType::SESSION),
		2 => Some(PacketType::LAP),
		3 => Some(PacketType::EVENT),
		4 => Some(PacketType::PARTICIPANT),
		5 => Some(PacketType::SETUP),
		6 => Some(PacketType::TELEMETRY),
		7 => Some(PacketType::CARSETUP),
		_ => None
	}
}

fn build_header(buf: &[u8; BUFFER_SIZE]) -> Option<PacketHeader> {
	//simple strategy for verifying valid packet header check 
}
