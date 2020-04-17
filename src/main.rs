
use std::net::UdpSocket;
use byte::*;

const BUFFER_SIZE: usize = 1347;						//Max packet size to spec
const DEFAULT_SOCKET_BINDING: &str = "0.0.0.0:20777";	//20777 default on ps4

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

#[derive(Debug)]
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

enum EventType { 
	SSTA,	// Session Started
	SEND,	// Session Ended
	FTLP,	// Fastest Lap
	RTMT,	// Retirement	
	DRSE,	// DRS Enabled	
	DRSD, 	// DRS Disabled
	TMPT,	// Team mate in the pits
	CHQF,	// The chequered flag has been waved
	RCWN,	// The race winner is announced 
}

struct Event {
	event_type: EventType,
	
}

#[derive(Debug)]
struct PacketHeader {   
    packet_format: 		u16,		// 2019
    maj_version: 		u8,			// Game major version - "X.00"
    min_version: 		u8,			// Game minor version - "1.XX"
    packet_version: 	u8,			// Version of this packet type, all start from 1
    packet_type:		PacketType,	// JP enum spec is u8
    session_id: 		u64,		// Unique identifier for the session
    session_time: 		f32,		// Session timestamp
    frame_id:			u16,  		//Identifier for the frame the data was retrieved on. JP 2020 this spec had typo 'uint' guessing 16 bit as 8 bit not enough to contain num frames in game - could be u64 but seems excessive.
    player_car_index: 	u8			// Index of player's car in the array
}

#[derive(Debug)]
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

#[derive(Debug)]
struct MarshalZone{
	zone_start:	f32,	// Fraction (0..1) of way through the lap the marshal zone starts
	flag:		i8,		// -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red   
}

#[derive(Debug)]
struct SessionData {
	header:			PacketHeader,
	weather:		u8,					// Weather - 0 = clear, 1 = light cloud, 2 = overcast, 3 = light rain, 4 = heavy rain, 5 = storm
	track_temp:		i8,					// Track temp. in degrees celsius
	air_temp:		i8,					// Air temp. in degrees celsius
	total_laps:		u8,					// Total number of laps in this race
	track_len:		u16,				// Track length in metres
	sesion_type:	u8,					// 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short, P5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ, 10 = R, 11 = R2, 12 = Time Trial
	track_id:		i8,					// -1 for unknown, 0-21 for tracks, see appendix
	formual:		u8,					// Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2, 3 = F1 Generic
	session_ttl:	u16,				// Time left in session in seconds
	session_len:	u16,				// Session duration in seconds
	pit_spd_lim:	u8,					// Pit speed limit in kilometres per hour
	is_paused:		u8,					// Whether the game is paused
	is_spectating:	u8,					// Whether the player is spectating
	spectator_car:	u8,					// Index of the car being spectated
	sli_native:		u8,					// SLI Pro support, 0 = inactive, 1 = active
	num_zones:		u8,					// Number of marshal zones to follow
	zones:			[MarshalZone; 21], 	// List of marshal zones – max 21
	safety_car:		u8,					// 0 = no safety car, 1 = full safety car, 2 = virtual safety car
	is_network_game:u8,					// 0 = offline, 1 = online
}

#[derive(Debug)]
struct LapData {          
	last_lap: f32,			// Last lap time in seconds
	current_lap: f32,		// Current time around the lap in seconds
	best_lap: f32,			// Best lap time of the session in seconds
	best_sec_1: f32,		// Sector 1 time in seconds
	best_sec_2: f32,		// Sector 2 time in seconds
	lap_distance: f32,		// Distance vehicle is around current lap in metres – could be negative if line hasn’t been crossed yet
	total_distance: f32,	// Total distance travelled in session in metres – could be negative if line hasn’t been crossed yet
	safety_car_delta: f32,	// Delta in seconds for safety car
	position: u8,			// Car race position
	lap_num: u8,			// Current lap number
	pit_status: u8,			// 0 = none, 1 = pitting, 2 = in pit area
	sector: u8,				// 0 = sector1, 1 = sector2, 2 = sector3
	is_lap_valid: u8,		// Current lap invalid - 0 = valid, 1 = invalid
	penalties: u8,			// Accumulated time penalties in seconds to be added
	grid_position: u8,		// Grid position the vehicle started the race in
	driver_status: u8,		// Status of driver - 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
	result_status: u8,		// Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = disqualified, 5 = not classified, 6 = retired
}

#[derive(Debug)]
struct Lap	{
    header:	PacketHeader,     // Header
    lap_data: [LapData; 20]  // Lap data for all cars on track
}

union EventDataDetails
{
    struct
    {
        uint8	vehicleIdx; // Vehicle index of car achieving fastest lap
        float	lapTime;    // Lap time is in seconds
    } FastestLap;

    struct
    {
        uint8   vehicleIdx; // Vehicle index of car retiring
    } Retirement;

    struct
    {
        uint8   vehicleIdx; // Vehicle index of team mate
    } TeamMateInPits;

    struct
    {
        uint8   vehicleIdx; // Vehicle index of the race winner
    } RaceWinner;
};

struct EventData {
   	header: PacketHeader,               // Header
	uint8           	m_eventStringCode[4];   // Event string code, see below
EventDataDetails	m_eventDetails;         // Event details - should be interpreted differently
// for each type
};

fn main() {
	let mut buf = [0u8; BUFFER_SIZE]; 	
    let socket = UdpSocket::bind(DEFAULT_SOCKET_BINDING).expect("failed to bind to socket");
    let mut bytes_recevied: usize = 0;
    loop {
    	let bytes_recevied = match socket.recv(&mut buf) {
    		Ok(bytes) => bytes,
    		Err(_e) => 0
    	};
    	if bytes_recevied == 0  {
    		continue;
    	}
    	let header: PacketHeader = parse_header(&buf, bytes_recevied).unwrap(); 
    	match header.packet_type {

    	}
    }
}

fn parse_header(buf: &[u8; BUFFER_SIZE], len: usize) -> Option<PacketHeader> {
	let packet_type: Option<PacketType> = get_packet_type(buf.read_with::<u8>(&mut PACKET_TYPE_OFFSET, LE).unwrap());
	println!("Got packet type {:?}", packet_type);
	match packet_type {
		Some(p) => build_header(&buf, p),
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
		7 => Some(PacketType::CARSTATUS),
		_ => None
	}
}

fn build_header(buf: &[u8; BUFFER_SIZE], packet_type: PacketType) -> Option<PacketHeader> {
	Some(PacketHeader {
		packet_format : buf.read_with(&mut PACKET_FORMAT_OFFSET, LE).unwrap(),
    	maj_version : buf.read_with(&mut MAJ_VERSION_OFFSET, LE).unwrap(),
    	min_version : buf.read_with(&mut MIN_VERSION_OFFSET, LE).unwrap(),
    	packet_version: buf.read_with(&mut PACKET_VERSION_OFFSET, LE).unwrap(),
    	packet_type,
    	session_id 	: buf.read_with(&mut SESSION_ID_OFFSET, LE).unwrap(),
    	session_time : buf.read_with(&mut SESSION_TIME_OFFSET, LE).unwrap(),
    	frame_id : buf.read_with(&mut FRAME_ID_OFFSET, LE).unwrap(),
    	player_car_index : buf.read_with(&mut PLAYER_CAR_INDEX_OFFESET, LE).unwrap(),
	} )
}
