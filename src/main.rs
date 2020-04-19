use std::net::UdpSocket;
use byte::*;

mod f1_2019_net;

use f1_2019_net::PacketType;
use f1_2019_net::PacketHeader;
use f1_2019_net::CarMotion;
use f1_2019_net::MotionData;
use f1_2019_net::SessionData;
use f1_2019_net::LapData;
use f1_2019_net::Lap;
use f1_2019_net::CarSetupData;
use f1_2019_net::CarSetups;
use f1_2019_net::CarStatusData;
use f1_2019_net::CarStatus;
use f1_2019_net::ParticipantData;
use f1_2019_net::Participants;
use f1_2019_net::Telemetry;
use f1_2019_net::CarTelemetry;
use f1_2019_net::EventType;
use f1_2019_net::Event;
use f1_2019_net::MarshalZone;


//If we have a 16k buffer we can temporarily store up to a min of 16384 / 1347 = 12 packets 
const BUFFER_SIZE: usize = 16384;
const MAX_PACKET_SIZE: usize = 1347;					//Max packet size to spec
const DEFAULT_SOCKET_BINDING: &str = "0.0.0.0:20777";	//20777 default on ps4
const HEADER_SIZE: usize = 21;
const MIN_PAYLOAD_SIZE: usize = 32;
const MIN_PACKET_SIZE: usize = HEADER_SIZE + MIN_PAYLOAD_SIZE;

//Header part offsets
const PACKET_FORMAT_OFFSET: usize = 0;
const MAJ_VERSION_OFFSET: usize = 2;
const MIN_VERSION_OFFSET: usize = 3;
const PACKET_VERSION_OFFSET: usize = 4;
const PACKET_TYPE_OFFSET: usize = 5;
const SESSION_ID_OFFSET: usize = 6;
const SESSION_TIME_OFFSET: usize = 14;
const FRAME_ID_OFFSET: usize = 18;
const PLAYER_CAR_INDEX_OFFESET: usize = 20;


fn main() {
    let mut motion_data: 		Vec<MotionData> = Vec::new();
    let mut session_data: 		Vec<SessionData> = Vec::new();
    let mut lap_data: 			Vec<Lap> = Vec::new();
    let mut event_data:			Vec<Event> = Vec::new();
    let mut setup_data: 		Vec<CarSetups> = Vec::new();
    let mut car_status_data: 	Vec<CarStatus> = Vec::new();
    let mut participant_data: 	Vec<Participants> = Vec::new();
    let mut telemetry_data: 	Vec<Telemetry> = Vec::new();	
	
	let mut header: 			PacketHeader = Default::default(); 
	let mut buf = [0u8; MAX_PACKET_SIZE]; 
    
    let socket = UdpSocket::bind(DEFAULT_SOCKET_BINDING).expect("failed to bind to socket");
    
    loop {
    	match socket.recv(&mut buf) {
    		Ok(num_bytes) => {
    			header = match parse_header(&buf, num_bytes) {
    				Some(header) => header,
    				None => continue,
    			}
    		}
    		Err(_) => continue,
    	};
    	match header.get_type() {
    		PacketType::Motion => match build_motion_data(&buf) {
    								Some(p) => motion_data.push(p),
    								None => {},
    							},
			PacketType::Session => match build_session_data(&buf)  {
    								Some(p) => session_data.push(p),
    								None => {},
    							},
			PacketType::Lap => match build_lap_data(&buf) {
    								Some(p) => lap_data.push(p),
    								None => {},
    							},
			PacketType::Event => match build_event_data(&buf) {
    								Some(p) => event_data.push(p),
    								None => {},
    							},
    		PacketType::Setup => match build_setup_data(&buf) {
    								Some(p) => setup_data.push(p),
    								None => {},
    							},
    		PacketType::CarStatus => match build_car_status_data(&buf)  {
    								Some(p) => car_status_data.push(p),
    								None => {},
    							},
			PacketType::Participant => match build_participant_data(&buf) {
    								Some(p) => participant_data.push(p),
    								None => {},
    							},
			PacketType::Telemetry => match build_telemetry_data(&buf) {
    								Some(p) => telemetry_data.push(p),
    								None => {},
    							},
		 	PacketType::InvalidPacket => {},
    	}
    }
}

fn get_packet_type(packet_type_byte: u8) -> PacketType {
	match packet_type_byte {
		0 => PacketType::Motion,
		1 => PacketType::Session,
		2 => PacketType::Lap,
		3 => PacketType::Event,
		4 => PacketType::Participant,
		5 => PacketType::Setup,
		6 => PacketType::Telemetry,
		7 => PacketType::CarStatus,
		_ => PacketType::InvalidPacket,
	}
}

fn parse_header(buf: &[u8; MAX_PACKET_SIZE], num_bytes: usize) -> Option<PacketHeader> {
	if num_bytes >= MIN_PACKET_SIZE {
		let header = PacketHeader::new( 
			buf.read_with::<u16>(&mut PACKET_FORMAT_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<u8>(&mut MAJ_VERSION_OFFSET, LE).expect("error reading from buffer!"),
	    	buf.read_with::<u8>(&mut MIN_VERSION_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<u8>(&mut PACKET_VERSION_OFFSET, LE).expect("error reading from buffer!"),
    		get_packet_type(buf.read_with::<u8>(&mut PACKET_TYPE_OFFSET, LE).expect("error reading from buffer!")),
    		buf.read_with::<u64>(&mut SESSION_ID_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<f32>(&mut SESSION_TIME_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<u16>(&mut FRAME_ID_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<u8>(&mut PLAYER_CAR_INDEX_OFFESET, LE).expect("error reading from buffer!"));
    	return Some(header)
	} else {
		return None;
	}
}	 

fn build_motion_data(buf: &[u8; MAX_PACKET_SIZE]) -> Option<MotionData> {
	None
}

fn build_session_data(buf: &[u8; MAX_PACKET_SIZE]) -> Option<SessionData> {
	None
}

fn build_lap_data(buf: &[u8; MAX_PACKET_SIZE]) -> Option<Lap> {
	None
}

fn build_event_data(buf: &[u8; MAX_PACKET_SIZE]) -> Option<Event> {
	None
}

fn build_participant_data(buf: &[u8; MAX_PACKET_SIZE]) -> Option<Participants> {
	None
}

fn build_setup_data(buf: &[u8; MAX_PACKET_SIZE]) -> Option<CarSetups> {
	None
}

fn build_telemetry_data(buf: &[u8; MAX_PACKET_SIZE]) -> Option<Telemetry> {
	None
}

fn build_car_status_data(buf: &[u8; MAX_PACKET_SIZE]) -> Option<CarStatus> {
	None
}
	

