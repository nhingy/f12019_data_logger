use std::net::UdpSocket;
use byte::*;
use std::collections::VecDeque;

/*
@TODO convert name array to utf-8
*/

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

const MAX_PACKET_SIZE: usize = 1347;					//Max packet size to spec
const DEFAULT_SOCKET_BINDING: &str = "0.0.0.0:20777";	//20777 default on ps4
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
const PLAYER_CAR_INDEX_OFFESET: usize = 22;

//Packset Sizes for v basic check vs packet type byte
const HEADER_SIZE: usize = 23;
const MOTION_SIZE: usize = 1343;
const SESSION_SIZE: usize = 149;
const LAP_SIZE: usize = 843;
const EVENT_SIZE: usize = 43; // Might not be able to use this as event packet changes with event type
const PARTICIPANTS_SIZE: usize = 1104;
const CARSETUPS_SIZE: usize = 843;
const TELEMETY_SIZE: usize = MAX_PACKET_SIZE;
const STATUS_SIZE: usize = 1143;

//Sub type sizes
const MARSHAL_ZONE_SIZE: usize = 5;
const CAR_MOTION_SIZE: usize = 60;
const CAR_LAP_SIZE: usize = 41;
const PARTICIPANT_SIZE: usize = 54;
const CAR_SETUP_SIZE: usize = 41;
const CAR_TELEMETRY_SIZE: usize = 66;
const CAR_STATUS_SIZE: usize = 56;

//Nums of elems
const NUM_MARSHAL_ZONES: usize = 21;
const NUM_CARS: usize = 20;
const NUM_WHEELS: usize = 4;

fn main() {
    let mut motion_data: 		VecDeque<MotionData> = VecDeque::with_capacity(200);
    let mut session_data: 		VecDeque<SessionData> = VecDeque::with_capacity(200);
    let mut lap_data: 			VecDeque<Lap> = VecDeque::with_capacity(200);
    let mut event_data:			VecDeque<Event> = VecDeque::with_capacity(200);
    let mut setup_data: 		VecDeque<CarSetups> = VecDeque::with_capacity(200);
    let mut car_status_data: 	VecDeque<CarStatusData> = VecDeque::with_capacity(200);
    let mut participant_data: 	VecDeque<Participants> = VecDeque::with_capacity(200);
    let mut telemetry_data: 	VecDeque<Telemetry> = VecDeque::with_capacity(200);	
	
	let mut header: 			PacketHeader = Default::default(); 
	let mut buf = [0u8; MAX_PACKET_SIZE]; 
    
    let socket = UdpSocket::bind(DEFAULT_SOCKET_BINDING).expect("failed to bind to socket");
    
    loop {
    	let num_bytes = match socket.recv(&mut buf) {
    		Ok(num_bytes) => num_bytes,
    		Err(_) => continue,
    	};
		match parse_header(&buf, num_bytes) {
			Some(header) => match header.get_type() {
						PacketType::Motion => match parse_motion_data(&buf, header, num_bytes) {
												Some(p) => motion_data.push_back(p),
												None => {},
											},
						PacketType::Session => match parse_session_data(&buf, header, num_bytes)  {
			    								Some(p) => session_data.push_back(p),
			    								None => {},
			    							},
						PacketType::Lap => match parse_lap_data(&buf, header, num_bytes) {
			    								Some(p) => lap_data.push_back(p),
			    								None => {},
			    							},
						PacketType::Event => match parse_event_data(&buf, header) {
			    								Some(p) => event_data.push_back(p),
			    								None => {},
			    							},
			    		PacketType::Setup => match parse_setups(&buf, header, num_bytes) {
			    								Some(p) => setup_data.push_back(p),
			    								None => {},
			    							},
			    		PacketType::CarStatus => match parse_car_status_data(&buf, header, num_bytes)  {
			    								Some(p) => car_status_data.push_back(p),
			    								None => {},
			    							},
						PacketType::Participant => match parse_participant_data(&buf, header, num_bytes) {
			    								Some(p) => participant_data.push_back(p),
			    								None => {},
			    							},
						PacketType::Telemetry => match parse_telemetry_data(&buf, header, num_bytes) {
			    								Some(p) => telemetry_data.push_back(p),
			    								None => {},
			    							},
					 	PacketType::InvalidPacket => {},
					},
			None => continue,
		}
    }
}

fn parse_header(buf: &[u8; MAX_PACKET_SIZE], num_bytes: usize) -> Option<PacketHeader> {
	if num_bytes >= MIN_PACKET_SIZE {
		let header = PacketHeader::new ( 
			buf.read_with::<u16>(&mut PACKET_FORMAT_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<u8>(&mut MAJ_VERSION_OFFSET, LE).expect("error reading from buffer!"),
	    	buf.read_with::<u8>(&mut MIN_VERSION_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<u8>(&mut PACKET_VERSION_OFFSET, LE).expect("error reading from buffer!"),
    		get_packet_type(buf.read_with::<u8>(&mut PACKET_TYPE_OFFSET, LE).expect("error reading from buffer!")),
    		buf.read_with::<u64>(&mut SESSION_ID_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<f32>(&mut SESSION_TIME_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<u32>(&mut FRAME_ID_OFFSET, LE).expect("error reading from buffer!"),
    		buf.read_with::<u8>(&mut PLAYER_CAR_INDEX_OFFESET, LE).expect("error reading from buffer!")
    		);
    	return Some(header)
	} else {
		return None;
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

fn parse_motion_data(buf: &[u8; MAX_PACKET_SIZE], header: PacketHeader, num_bytes: usize) -> Option<MotionData> {
	if num_bytes != MOTION_SIZE {
		return None;
	}
	let mut car_data = [CarMotion::default(); NUM_CARS];
	parse_car_motion(&buf, &mut car_data, NUM_CARS);
	let mut sus_pos 	= [0.0f32; NUM_WHEELS];
	let mut sus_vel 	= [0.0f32; NUM_WHEELS];
	let mut sus_acc 	= [0.0f32; NUM_WHEELS];
	let mut wheel_spd 	= [0.0f32; NUM_WHEELS];
	let mut wheel_slp 	= [0.0f32; NUM_WHEELS];
	parse_wheel_array_f32(&mut sus_pos, &buf[HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)..HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+16], NUM_WHEELS);
	parse_wheel_array_f32(&mut sus_vel, &buf[HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+16..HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+32], NUM_WHEELS);
	parse_wheel_array_f32(&mut sus_acc, &buf[HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+48..HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+64], NUM_WHEELS);
	parse_wheel_array_f32(&mut wheel_spd, &buf[HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+80..HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+96], NUM_WHEELS);
	parse_wheel_array_f32(&mut wheel_slp, &buf[HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+112..HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+128], NUM_WHEELS);
	let mut offset = HEADER_SIZE+(CAR_MOTION_SIZE*NUM_CARS)+128;
	return Some ( MotionData {
				header,
				car_motion_data: 	car_data,
				suspension_pos: 	sus_pos,
    			suspension_vel: 	sus_vel,
    			suspension_acc:		sus_acc,
    			wheel_speed: 		wheel_spd,
    			wheel_slip:			wheel_slp,
    			local_vel_x:		buf.read_with::<f32>(&mut (offset), LE).unwrap(),			//Local space
    			local_vel_y:		buf.read_with::<f32>(&mut (offset+4), LE).unwrap(),
    			local_vel_z:		buf.read_with::<f32>(&mut (offset+8), LE).unwrap(),
    			angular_vel_x:		buf.read_with::<f32>(&mut (offset+12), LE).unwrap(),
    			angular_vel_y:		buf.read_with::<f32>(&mut (offset+16), LE).unwrap(),
    			angular_vel_z:		buf.read_with::<f32>(&mut (offset+22), LE).unwrap(),
    			angular_acc_x:		buf.read_with::<f32>(&mut (offset+26), LE).unwrap(),
    			angular_acc_y:		buf.read_with::<f32>(&mut (offset+30), LE).unwrap(),
    			angular_acc_z:		buf.read_with::<f32>(&mut (offset+34), LE).unwrap(),
    			front_wheels_angle: buf.read_with::<f32>(&mut (offset+38), LE).unwrap(),
		})
}

fn parse_car_motion(buf: &[u8; MAX_PACKET_SIZE], car_data: &mut [CarMotion; 20], count: usize) {
	let index: usize = NUM_CARS - count;
	let mut offset = index * CAR_MOTION_SIZE;
	let car_data = CarMotion {
		world_pos_x: buf.read_with::<f32>(&mut (HEADER_SIZE+offset), LE).unwrap(),			
    	world_pos_y: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+4), LE).unwrap(),
    	world_pos_z: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+8), LE).unwrap(),
    	world_vel_x: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+12), LE).unwrap(),			
    	world_vel_y: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+16), LE).unwrap(),
    	world_vel_z: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+20), LE).unwrap(),
    	world_fwd_dir_x: buf.read_with::<i16>(&mut (HEADER_SIZE+offset+24), LE).unwrap(),	 	
    	world_fwd_dir_y: buf.read_with::<i16>(&mut (HEADER_SIZE+offset+26), LE).unwrap(),		
    	world_fwd_dir_z: buf.read_with::<i16>(&mut (HEADER_SIZE+offset+28), LE).unwrap(),		
    	world_right_dir_x: buf.read_with::<i16>(&mut (HEADER_SIZE+offset+30), LE).unwrap(),		
    	world_right_dir_y: buf.read_with::<i16>(&mut (HEADER_SIZE+offset+32), LE).unwrap(),		
    	world_right_dir_z: buf.read_with::<i16>(&mut (HEADER_SIZE+offset+34), LE).unwrap(),		
    	lateral_g: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+36), LE).unwrap(),
    	longitudinal_g: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+40), LE).unwrap(),
    	vertical_g: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+44), LE).unwrap(),
    	pitch: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+48), LE).unwrap(),					
    	yaw: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+52), LE).unwrap(),					
    	roll: buf.read_with::<f32>(&mut (HEADER_SIZE+offset+56), LE).unwrap(),	
	};

}

fn parse_wheel_array_f32(array_to_fill: &mut [f32; NUM_WHEELS], bytes: &[u8], count: usize) {
	let mut index = NUM_WHEELS - count;
	array_to_fill[index] = bytes.read_with::<f32>(&mut (index * 4), LE).unwrap(); // *4 4 bytes per f32
	if count <= 0 {
		return;
	} else {
		parse_wheel_array_f32(array_to_fill, bytes, count-1);
	}
}

fn parse_wheel_array_u16(array_to_fill: &mut [u16; NUM_WHEELS], bytes: &[u8], count: usize) {
	let mut index = NUM_WHEELS - count;
	array_to_fill[index] = bytes.read_with::<u16>(&mut (index * 2), LE).unwrap(); // *2 2 bytes per f32
	if count <= 0 {
		return;
	} else {
		parse_wheel_array_u16(array_to_fill, bytes, count-1);
	}
}

fn parse_wheel_array_u8(array_to_fill: &mut [u8; NUM_WHEELS], bytes: &[u8], count: usize) {
	let mut index = NUM_WHEELS - count;
	array_to_fill[index] = bytes.read_with::<u8>(&mut index, LE).unwrap();
	if count <= 0 {
		return;
	} else {
		parse_wheel_array_u8(array_to_fill, bytes, count-1);
	}
}

fn parse_session_data(buf: &[u8; MAX_PACKET_SIZE], header: PacketHeader, num_bytes: usize) -> Option<SessionData> {
	//crappy check 
	if num_bytes != SESSION_SIZE {
		return None;
	}
	let mut zones = [MarshalZone{zone_start: 0.0, flag: 0}; NUM_MARSHAL_ZONES];
	parse_marshal_zones(&mut zones, &buf[HEADER_SIZE+19..], NUM_MARSHAL_ZONES);
	return Some( SessionData {
		header,			
		weather:		buf.read_with::<u8>(&mut HEADER_SIZE, LE).unwrap(),		
		track_temp:		buf.read_with::<i8>(&mut (HEADER_SIZE+1), LE).unwrap(),	
		air_temp:		buf.read_with::<i8>(&mut (HEADER_SIZE+2), LE).unwrap(),	
		total_laps:		buf.read_with::<u8>(&mut (HEADER_SIZE+3), LE).unwrap(),	
		track_len:		buf.read_with::<u16>(&mut (HEADER_SIZE+4), LE).unwrap(),	
		session_type:	buf.read_with::<u8>(&mut (HEADER_SIZE+6), LE).unwrap(),	
		track_id:		buf.read_with::<i8>(&mut (HEADER_SIZE+7), LE).unwrap(),
		formual:		buf.read_with::<u8>(&mut (HEADER_SIZE+8), LE).unwrap(),
		session_ttl:	buf.read_with::<u16>(&mut (HEADER_SIZE+9), LE).unwrap(),
		session_len:	buf.read_with::<u16>(&mut (HEADER_SIZE+11), LE).unwrap(),
		pit_spd_lim:	buf.read_with::<u8>(&mut (HEADER_SIZE+13), LE).unwrap(),
		is_paused:		buf.read_with::<u8>(&mut (HEADER_SIZE+14), LE).unwrap(),
		is_spectating:	buf.read_with::<u8>(&mut (HEADER_SIZE+15), LE).unwrap(),
		spectator_car:	buf.read_with::<u8>(&mut (HEADER_SIZE+16), LE).unwrap(),
		sli_native:		buf.read_with::<u8>(&mut (HEADER_SIZE+17), LE).unwrap(),
		num_zones:		buf.read_with::<u8>(&mut (HEADER_SIZE+18), LE).unwrap(),
		zones:			zones,
		safety_car:		buf.read_with::<u8>(&mut (HEADER_SIZE+124), LE).unwrap(), // HEADER_SIZE + 19 (previous offset) + 21*5 (Marshal Zone is 1xi8 & 1xf32)
		is_network_game:buf.read_with::<u8>(&mut (HEADER_SIZE+125), LE).unwrap(), // HEADER_SIZE + 125 = 148 giving tot packet size 149
	})	
}

fn parse_marshal_zones(zones: &mut [MarshalZone; 21], bytes: &[u8], count: usize) {
	let mut index = NUM_MARSHAL_ZONES - count;
	let zone = MarshalZone {
		zone_start: bytes.read_with::<f32>(&mut (0+(5*index)), LE).unwrap(),	
		flag:		bytes.read_with::<i8>(&mut (4+(5*index)), LE).unwrap(), 
	};	
	zones[index] = zone;
	if count <= 0 {
		return;
	} else {
		parse_marshal_zones(zones, bytes, count-1);
	}
}

fn parse_lap_data(buf: &[u8; MAX_PACKET_SIZE], header: PacketHeader, num_bytes: usize) -> Option<Lap> {
	if num_bytes != LAP_SIZE {
		return None;
	}
	let mut car_laps = [LapData::default(); 20];
	parse_car_laps(&mut car_laps, &buf[HEADER_SIZE..], NUM_CARS);
	return Some (Lap {
		header,
		lap_data: car_laps,
	})
}

fn parse_car_laps(car_laps: &mut [LapData; 20], bytes: &[u8], count: usize) { 
	let mut index = NUM_CARS - count;
	let mut offset: usize = index * CAR_LAP_SIZE;
	let car_lap_data = LapData {
		last_lap: 		bytes.read_with::<f32>(&mut offset, LE).unwrap(),			
		current_lap: 	bytes.read_with::<f32>(&mut (offset+4), LE).unwrap(),		
		best_lap: 		bytes.read_with::<f32>(&mut (offset+8), LE).unwrap(),			
		best_sec_1: 	bytes.read_with::<f32>(&mut (offset+12), LE).unwrap(),		
		best_sec_2: 	bytes.read_with::<f32>(&mut (offset+16), LE).unwrap(),		
		lap_distance: 	bytes.read_with::<f32>(&mut (offset+20), LE).unwrap(),		
		total_distance: bytes.read_with::<f32>(&mut (offset+24), LE).unwrap(),	
		safety_car_delta: bytes.read_with::<f32>(&mut (offset+28), LE).unwrap(),	
		position: 		bytes.read_with::<u8>(&mut (offset+32), LE).unwrap(),			
		lap_num: 		bytes.read_with::<u8>(&mut (offset+33), LE).unwrap(),			
		pit_status: 	bytes.read_with::<u8>(&mut (offset+34), LE).unwrap(),			
		sector: 		bytes.read_with::<u8>(&mut (offset+35), LE).unwrap(),				
		is_lap_valid: 	bytes.read_with::<u8>(&mut (offset+36), LE).unwrap(),		
		penalties: 		bytes.read_with::<u8>(&mut (offset+37), LE).unwrap(),			
		grid_position: 	bytes.read_with::<u8>(&mut (offset+38), LE).unwrap(),		
		driver_status: 	bytes.read_with::<u8>(&mut (offset+39), LE).unwrap(),		
		result_status: 	bytes.read_with::<u8>(&mut (offset+40), LE).unwrap(),		
	};
	car_laps[index] = car_lap_data;
	if count <= 0 {
		return;
	} else {
		parse_car_laps(car_laps, bytes, count-1);
	}

}

fn parse_event_data(buf: &[u8; MAX_PACKET_SIZE], header: PacketHeader) -> Option<Event> {
	//Can't naively check event packet size as this varies with event type
	let evt_slice = &buf[HEADER_SIZE..];
	let event_type = match parse_event_identifier(evt_slice) {
		Some(e) => e,
		_ => return None,
	};
	match event_type {
		EventType::SessionStarted 	=> return Some(Event {
													header, 
													event_type: EventType::SessionStarted, 
													car_idx: 255, 
													lap_time: 0.0}), 
		EventType::SessionEnded 	=> return Some(Event {
													header, 
													event_type: EventType::SessionEnded, 
													car_idx: 255, 
													lap_time: 0.0}),
		EventType::FastestLap		=> return parse_multipart_event(evt_slice, event_type, header),
		EventType::Retirement		=> return parse_multipart_event(evt_slice, event_type, header),
		EventType::DrsEnabled		=> return Some(Event {
													header, 
													event_type: EventType::DrsEnabled, 
													car_idx: 255, 
													lap_time: 0.0}),
		EventType::DrsDisabled		=> return Some(Event {
													header, 
													event_type: EventType::DrsDisabled, 
													car_idx: 255, 
													lap_time: 0.0}),
		EventType::TeamMateInPits	=> return parse_multipart_event(evt_slice, event_type, header),
		EventType::ChequeredFlag	=> return Some(Event {
													header, 
													event_type: EventType::ChequeredFlag, 
													car_idx: 255, 
													lap_time: 0.0}),
		EventType::RaceWinner		=> return parse_multipart_event(evt_slice, event_type, header),
		_							=> return None,
	}
	
}

fn parse_multipart_event(bytes: &[u8], event_type: EventType, header: PacketHeader) -> Option<Event> {
	let mut offset = 4; //skip over event type identifier bytes
	//first byte is always car index
	let car_idx = bytes.read_with::<u8>(&mut offset, LE).unwrap();
	match event_type {
		EventType::FastestLap => return 
			Some(Event{
			header, 
			event_type,
			car_idx,
			lap_time: bytes.read_with::<f32>(&mut (offset+1), LE).unwrap(), 
		}),
		_ => return 
			Some(Event{
			header, 
			event_type,
			car_idx,
			lap_time: 0.0, 
		}) 	
	}
}

fn parse_event_identifier(ascii_bytes: &[u8]) -> Option<EventType> {
	// Instead of converting bytes to ascii and then comparing strings we add up the byte vals 
	// as each 4 char combined val is unique. Could have compared first two bytes but two type share first 3 chars.
	// I'm confused by this way of id-ing event packet types, maybe this was used for readability and cost not too 
	// high as event packets are infrequent
	let mut array = [0u8; 4];
	array[0] = ascii_bytes.read_with::<u8>(&mut 0, LE).unwrap();
	array[1] = ascii_bytes.read_with::<u8>(&mut 1, LE).unwrap();
	array[2] = ascii_bytes.read_with::<u8>(&mut 2, LE).unwrap();
	array[3] = ascii_bytes.read_with::<u8>(&mut 3, LE).unwrap();
	let added_bytes: u32 = u32::from_ne_bytes(array);
	match added_bytes {
		315 => return Some(EventType::SessionStarted),
		298 => return Some(EventType::SessionEnded),
		310 => return Some(EventType::FastestLap),
		327 => return Some(EventType::Retirement),
		302 => return Some(EventType::DrsEnabled),
		301 => return Some(EventType::DrsDisabled),
		325 => return Some(EventType::TeamMateInPits),
		290 => return Some(EventType::ChequeredFlag),
		314 => return Some(EventType::RaceWinner),
		_ => return None,
	}
}

fn parse_participant_data(buf: &[u8; MAX_PACKET_SIZE], header: PacketHeader, num_bytes: usize) -> Option<Participants> {
	if num_bytes != PARTICIPANTS_SIZE {
		return None;
	}
	let mut participants = [ParticipantData::new(); 20];
	parse_participant(&mut participants, &buf[HEADER_SIZE+1..], NUM_CARS); //+1 due to u8 num cars active
	return Some(Participants{
			header,
			num_cars_active: buf.read_with::<u8>(&mut (HEADER_SIZE+1), LE).unwrap(),
			participant_data: participants,
		});
}

fn parse_participant(participants: &mut [ParticipantData; NUM_CARS], bytes: &[u8], count: usize) {
	let index = NUM_CARS - count;
	let mut offset = index * PARTICIPANT_SIZE;
	let participant = ParticipantData {
		ai_controlled: 	bytes.read_with::<u8>(&mut offset, LE).unwrap(),
    	driver_id: 		bytes.read_with::<u8>(&mut (offset+1), LE).unwrap(),
    	team_id: 		bytes.read_with::<u8>(&mut (offset+2), LE).unwrap(),
    	race_number: 	bytes.read_with::<u8>(&mut (offset+3), LE).unwrap(),
    	nationality: 	bytes.read_with::<u8>(&mut (offset+4), LE).unwrap(),
    	name: 			get_name_bytes(&bytes[offset+5..offset+52]),
    	priv_telemetry: bytes.read_with::<u8>(&mut (offset+53), LE).unwrap(),
	};
	participants[index] = participant;
	if count <= 0 {
		return;
	} else {
		parse_participant(participants, bytes, count-1);
	}

}

fn get_name_bytes(slice: &[u8]) -> [u8; 48]{
	let mut name_array = [0u8; 48];
	for x in 0..47 {
		name_array[x] = slice[x];
	}
	name_array
}

fn parse_setups(buf: &[u8; MAX_PACKET_SIZE], header: PacketHeader, num_bytes: usize) -> Option<CarSetups> {
	if  num_bytes != CARSETUPS_SIZE {
		return None;
	}
	let mut setup_data = [CarSetupData::default(); NUM_CARS];
	parse_car_setup(&mut setup_data, &buf[HEADER_SIZE..], NUM_CARS);
	return Some(CarSetups{
		header,
		car_setups: setup_data,
	});
}

fn parse_car_setup(setup_data: &mut [CarSetupData; NUM_CARS], bytes: &[u8], count: usize) {
	let index = NUM_CARS - count;
	let mut offset = index * CAR_SETUP_SIZE;
	let setup = CarSetupData {
		 		front_wing: 			bytes.read_with::<u8>(&mut offset, LE).unwrap(), 			
    			rear_wing:				bytes.read_with::<u8>(&mut (offset+1), LE).unwrap(),			
    			on_throttle: 			bytes.read_with::<u8>(&mut (offset+2), LE).unwrap(),			
    			off_throttle: 			bytes.read_with::<u8>(&mut (offset+3), LE).unwrap(),		
    			front_camber: 			bytes.read_with::<f32>(&mut (offset+4), LE).unwrap(),		
    			rear_camber: 			bytes.read_with::<f32>(&mut (offset+8), LE).unwrap(),		
    			front_toe: 				bytes.read_with::<f32>(&mut (offset+12), LE).unwrap(),		
    			rear_toe: 				bytes.read_with::<f32>(&mut (offset+16), LE).unwrap(),	
    			front_suspension: 		bytes.read_with::<u8>(&mut (offset+20), LE).unwrap(),	
    			rear_suspension:		bytes.read_with::<u8>(&mut (offset+21), LE).unwrap(),	
    			front_anti_roll_bar: 	bytes.read_with::<u8>(&mut (offset+22), LE).unwrap(),	
    			rear_anti_roll_bar: 	bytes.read_with::<u8>(&mut (offset+23), LE).unwrap(),
    			front_suspension_height:bytes.read_with::<u8>(&mut (offset+24), LE).unwrap(),
    			rear_suspension_height: bytes.read_with::<u8>(&mut (offset+25), LE).unwrap(),
    			brake_pressure: 		bytes.read_with::<u8>(&mut (offset+26), LE).unwrap(),
    			brake_bias: 			bytes.read_with::<u8>(&mut (offset+27), LE).unwrap(),
    			front_tyre_pressure: 	bytes.read_with::<f32>(&mut (offset+28), LE).unwrap(),
    			rear_tyre_pressure: 	bytes.read_with::<f32>(&mut (offset+32), LE).unwrap(),
    			ballast: 				bytes.read_with::<u8>(&mut (offset+36), LE).unwrap(),	
    			fuel_load: 				bytes.read_with::<f32>(&mut (offset+37), LE).unwrap(),
			};
	setup_data[index] = setup;
	if count <= 0 {
		return; 
	} else {
		parse_car_setup(setup_data, bytes, count-1);
	}
}

fn parse_telemetry_data(buf: &[u8; MAX_PACKET_SIZE], header: PacketHeader, num_bytes: usize) -> Option<Telemetry> {
	if num_bytes != TELEMETY_SIZE {
		return None;
	}
	let mut car_telemetry = [CarTelemetry::default(); NUM_CARS];
	parse_car_telemetry(&mut car_telemetry, &buf[HEADER_SIZE..], NUM_CARS);
	return Some(Telemetry{
				header,
				car_telemetry_data: car_telemetry,
				button_status: buf.read_with::<u32>(&mut (HEADER_SIZE+(NUM_CARS*CAR_TELEMETRY_SIZE)), LE).unwrap(),
			});

}

fn parse_car_telemetry(car_telemetry: &mut [CarTelemetry; NUM_CARS], bytes: &[u8], count: usize) {
	let index = NUM_CARS - count;
	let mut offset = index * CAR_TELEMETRY_SIZE;
	let mut brake_temps		= [0u16; NUM_WHEELS];	
	let mut tyre_s_temps 	= [0u16; NUM_WHEELS];
	let mut tyre_ic_temps 	= [0u16; NUM_WHEELS];
	let mut tyre_pres 		= [0f32; NUM_WHEELS];
	let mut tyre_contacts 	= [0u8; NUM_WHEELS];
	parse_wheel_array_u16(&mut brake_temps, &bytes[offset+20..], NUM_WHEELS);
	parse_wheel_array_u16(&mut tyre_s_temps, &bytes[offset+28..], NUM_WHEELS);
	parse_wheel_array_u16(&mut tyre_ic_temps, &bytes[offset+36..], NUM_WHEELS);
	parse_wheel_array_f32(&mut tyre_pres, &bytes[offset+46..], NUM_WHEELS);
	parse_wheel_array_u8(&mut tyre_contacts, &bytes[offset+62..], NUM_WHEELS);
	let telemetry =			 
		CarTelemetry {
					car_speed: 			bytes.read_with::<u16>(&mut (offset), LE).unwrap(),			
				    throttle_pos: 		bytes.read_with::<f32>(&mut (offset+2), LE).unwrap(),	
				    steering_pos:		bytes.read_with::<f32>(&mut (offset+6), LE).unwrap(),	
				    brake_pos: 			bytes.read_with::<f32>(&mut (offset+10), LE).unwrap(),	
				    clutch_pos:			bytes.read_with::<u8>(&mut (offset+14), LE).unwrap(),	
				    gear: 				bytes.read_with::<i8>(&mut (offset+15), LE).unwrap(),	
				    engine_rpm:			bytes.read_with::<u16>(&mut (offset+16), LE).unwrap(),	
				    drs_active: 		bytes.read_with::<u8>(&mut (offset+18), LE).unwrap(),	
				    change_light_perc: 	bytes.read_with::<u8>(&mut (offset+19), LE).unwrap(),	
				    brake_temps: 		brake_temps,
				    tyre_surface_temps:	tyre_s_temps,
				    tyre_inner_temps: 	tyre_ic_temps,	
				    engine_temp: 		bytes.read_with::<u16>(&mut (offset+44), LE).unwrap(),	
				    tyre_pressures: 	tyre_pres,
				    tyre_contact_types: tyre_contacts,
				};
	car_telemetry[index] = telemetry;
	if count <= 0 {
		return;
	} else {
		parse_car_telemetry(car_telemetry, bytes, count-1);
	}
}

fn parse_car_status_data(buf: &[u8; MAX_PACKET_SIZE], header: PacketHeader, num_bytes: usize) -> Option<CarStatusData> {
	if num_bytes != STATUS_SIZE {
		return None;
	} 
	let mut status_data = [CarStatus::default(); NUM_CARS];
	parse_car_status(&mut status_data, &buf[HEADER_SIZE..], NUM_CARS);
	return Some(CarStatusData{
		header, 
		car_status_data: status_data,
	});
}

fn parse_car_status(status_data: &mut [CarStatus; NUM_CARS], bytes: &[u8], count: usize) {
	let index = NUM_CARS - count;
	let mut offset = index * CAR_STATUS_SIZE;
	let mut tyre_wear 	= [0u8; NUM_WHEELS];
	let mut tyre_damage = [0u8; NUM_WHEELS];
	parse_wheel_array_u8(&mut tyre_wear, &bytes[offset+23..], NUM_WHEELS);
	parse_wheel_array_u8(&mut tyre_damage, &bytes[offset+29..], NUM_WHEELS);
	let car_status_data = 
		CarStatus {
			    traction_control: 	bytes.read_with::<u8>(&mut (offset), LE).unwrap(),		
			    anti_lock_brakes: 	bytes.read_with::<u8>(&mut (offset+1), LE).unwrap(),		
			    fuel_mix: 			bytes.read_with::<u8>(&mut (offset+2), LE).unwrap(),	
			    front_brake_bias: 	bytes.read_with::<u8>(&mut (offset+3), LE).unwrap(),	
			    pit_limiter_status: bytes.read_with::<u8>(&mut (offset+4), LE).unwrap(),	
			    fuel_in_tank: 		bytes.read_with::<f32>(&mut (offset+5), LE).unwrap(),	
				fuel_capacity: 		bytes.read_with::<f32>(&mut (offset+9), LE).unwrap(),	
				fuel_remaining_laps:bytes.read_with::<f32>(&mut (offset+13), LE).unwrap(),	
			    max_rpm: 			bytes.read_with::<u16>(&mut (offset+17), LE).unwrap(),	
			    idle_rpm:			bytes.read_with::<u16>(&mut (offset+19), LE).unwrap(),	
			    max_gears: 			bytes.read_with::<u8>(&mut (offset+21), LE).unwrap(),	
			    drs_allowed: 		bytes.read_with::<u8>(&mut (offset+22), LE).unwrap(),	
			    tyres_wear: 			tyre_wear,	
			    actual_tyre_compound: 	bytes.read_with::<u8>(&mut (offset+27), LE).unwrap(),	
				tyre_visual_compound: 	bytes.read_with::<u8>(&mut (offset+28), LE).unwrap(),      										
			    tyres_damage: 			tyre_damage,
			    front_left_wing_damage: bytes.read_with::<u8>(&mut (offset+33), LE).unwrap(),
			    front_right_wing_damage:bytes.read_with::<u8>(&mut (offset+34), LE).unwrap(),
			    rear_wing_damage: 		bytes.read_with::<u8>(&mut (offset+35), LE).unwrap(),
			    engine_damage: 			bytes.read_with::<u8>(&mut (offset+36), LE).unwrap(),
			    gear_box_damage: 		bytes.read_with::<u8>(&mut (offset+37), LE).unwrap(),
			    vehicle_fia_flags: 		bytes.read_with::<i8>(&mut (offset+38), LE).unwrap(),                   
			    ers_store_energy: 		bytes.read_with::<f32>(&mut (offset+39), LE).unwrap(),
			    ers_deploy_mode: 		bytes.read_with::<u8>(&mut (offset+43), LE).unwrap(),			
			    ers_harvested_this_lap_mguk:	bytes.read_with::<f32>(&mut (offset+44), LE).unwrap(),
			    ers_harvested_this_lap_mguh:	bytes.read_with::<f32>(&mut (offset+48), LE).unwrap(),
			    ers_deployed_this_lap: 			bytes.read_with::<f32>(&mut (offset+52), LE).unwrap(),
		};
	status_data[index] = car_status_data;
	if count <= 0 {
		return;
	} else {
		parse_car_status(status_data, bytes, count-1);
	}
}


	

