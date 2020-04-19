#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    Motion,
    Session,
    Lap,
    Event,
    Participant,
    Setup,
    Telemetry,  
    CarStatus,
    InvalidPacket,
}

impl Default for PacketType {
    fn default() -> Self {PacketType::InvalidPacket}
}

//Design is that events are differentiated by way of a 4 byte array of ascii chars???
//We have no need to read these as chars so plan to create an instance of these enum vals with the int val of the ascii chars in question 
//Possible improvement calculate the 32 bit val of concating these bytes together as we could read the data stream as one 32 bit val rather than 4 8 bit vals
#[derive(Debug)]
pub enum EventType { 
    SSTA([u8; 4]),      // Session Started "SSTA" S=83, S=83, T=84, A=65
    SEND([u8; 4]),      // Session Ended "SEND" S=83, E=69, N=78, D=68
    FTLP([u8; 4]),      // Fastest Lap "FTLP" F=70, T=84, L=76, P=80
    RTMT([u8; 4]),      // Retirement "RTMT" R=82, T=84, M=77, T=84
    DRSE([u8; 4]),      // DRS Enabled "DRSE" D=68, R=82, S=83, E=69
    DRSD([u8; 4]),      // DRS Disabled "DRSD" D=68, R=82, S=83, D=68
    TMPT([u8; 4]),      // Team mate in the pits "TMPT" T=84, M=77, P=80, T=84
    CHQF([u8; 4]),      // The chequered flag has been waved "CHQF" C=67, H=72, Q=81, F=70
    RCWN([u8; 4]),      // The race winner is announced "RCWN" R=82, C=67, W=87, N=78
}

#[derive(Debug)]
pub struct Event {
	header: PacketHeader,
	event_type: EventType,
	car_idx: u8,		//The car if this event refers to a car
	lap_time: f32,		//Lap time if this event is fastest lap
}

//
#[derive(Debug, Default)]
pub struct PacketHeader {   
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

impl PacketHeader {
    pub fn get_type(&self) -> PacketType {
        self.packet_type    
    }

    pub fn new(packet_format: u16, maj_version: u8, min_version: u8, packet_version: u8, 
            packet_type: PacketType, session_id: u64, session_time: f32, frame_id: u16, player_car_index: u8) -> Self {
        PacketHeader {
            packet_format,
            maj_version,
            min_version,
            packet_version,
            packet_type,
            session_id,
            session_time,
            frame_id,
            player_car_index,
        }
    }
}

#[derive(Debug)]
pub struct CarTelemetry {				//65 bytes? 
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

pub struct Telemetry {
    header:             PacketHeader, 
    car_telemetry_data: [CarTelemetry; 20],
    buttonStatus:       u32,               // Bit flags specifying which buttons are being pressed currently - see appendices 
}

#[derive(Debug)]
pub struct MarshalZone {
	zone_start:	f32,	// Fraction (0..1) of way through the lap the marshal zone starts
	flag:		i8,		// -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red   
}

#[derive(Debug)]
pub struct SessionData {
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
pub struct LapData {          
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
pub struct Lap	{
    header:	PacketHeader,     // Header
    lap_data: [LapData; 20]  // Lap data for all cars on track
}



// This is a list of participants in the race. If the vehicle is controlled by AI, 
//then the name will be the driver name. If this is a multiplayer game, the names will be the Steam Id on PC, or the LAN name if appropriate.
//N.B. on Xbox One, the names will always be the driver name, on PS4 the name will be the LAN name if playing a LAN game, otherwise it will be the driver name.
//The array should be indexed by vehicle index.

//Frequency: Every 5 seconds
//Size: 1104 bytes
//Version: 1 
//Can't Derive Debug due to array size
//		arrays only have std trait implementations for lengths 0..=32
//	  = note: required because of the requirements on the impl of `std::fmt::Debug` for `[u8; 48]`
//    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&[u8; 48]`
//    = note: required for the cast to the object type `dyn std::fmt::Debug`
pub struct ParticipantData {
    ai_controlled: 	u8,         // Whether the vehicle is AI (1) or Human (0) controlled
    driver_id: 		u8,			// Driver id - see appendix
    team_id: 		u8,         // Team id - see appendix
    race_number: 	u8,         // Race number of the car
    nationality: 	u8,         // Nationality of the driver
    name: 			[u8; 48],   // Name of participant in UTF-8 format – null terminated Will be truncated with … (U+2026) if too long
    priv_telemetry: u8,         // The player's UDP setting, 0 = restricted, 1 = public
}

pub struct Participants {
    header: 			PacketHeader,			// Header
    num_cars_active:	u8,						// Number of active cars in the data – should match number of cars on HUD
    participant_data:	[ParticipantData; 20],	// Data for all cars
}

//Car Setups Packet
//This packet details the car setups for each vehicle in the session. Note that in multiplayer games, other player cars will appear as blank, 
//you will only be able to see your car setup and AI cars.
//Frequency: 2 per second
//Size: 843 bytes
//Version: 1

pub struct CarSetupData {
    front_wing: 			u8,             // Front wing aero
    rear_wing:				u8,             // Rear wing aero
    on_throttle: 			u8,             // Differential adjustment on throttle (percentage)
    off_throttle: 			u8,             // Differential adjustment off throttle (percentage)
    front_camber: 			f32,            // Front camber angle (suspension geometry)
    rear_camber: 			f32,            // Rear camber angle (suspension geometry)
    front_toe: 				f32,            // Front toe angle (suspension geometry)
    rear_toe: 				f32,            // Rear toe angle (suspension geometry)
    front_suspension: 		u8,         	// Front suspension
    rear_suspension:		u8,           	// Rear suspension
    front_anti_roll_bar: 	u8,         	// Front anti-roll bar
    rear_anti_roll_bar: 	u8,          	// Front anti-roll bar
    front_suspension_height:u8,    			// Front ride height
    rear_suspension_height: u8,     		// Rear ride height
    brake_pressure: 		u8,            	// Brake pressure (percentage)
    brake_bias: 			u8,             // Brake bias (percentage)
    front_tyre_pressure: 	f32,       		// Front tyre pressure (PSI)
    rear_tyre_pressure: 	f32,         	// Rear tyre pressure (PSI)
    ballast: 				u8,             // Ballast
    fuel_load: 				f32,            // Fuel load
}

pub struct CarSetups {
    header: PacketHeader,
    car_setups: [CarSetupData; 20],
}

//---------------Car Status Packet
//This packet details car statuses for all the cars in the race. It includes values such as the damage readings on the car.
//Frequency: Rate as specified in menus
//Size: 1143 bytes
//Version: 1 
pub struct CarStatusData {
	header: 			PacketHeader,			// Header
    car_status_data: 	[CarStatus; 20],		// Array of car status'
}

pub struct CarStatus {
    traction_control: 		u8,         // 0 (off) - 2 (high)
    anti_lock_brakes: 		u8,         // 0 (off) - 1 (on)
    fuel_mix: 				u8,         // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    front_brake_bias: 		u8,         // Front brake bias (percentage)
    pit_limiter_status: 	u8,       	// Pit limiter status - 0 = off, 1 = on
    fuel_in_tank: 			f32,        // Current fuel mass
	fuel_capacity: 			f32,        // Fuel capacity
	fuel_remaining_laps:	f32,       	// Fuel remaining in terms of laps (value on MFD)
    max_rpm: 				u16,        // Cars max RPM, point of rev limiter
    idle_rpm:				u16,        // Cars idle RPM
    max_gears: 				u8,         // Maximum number of gears
    drs_allowed: 			u8,         // 0 = not allowed, 1 = allowed, -1 = unknown
    tyres_wear: 			[u8; 4],    // Tyre wear percentage
    actual_tyre_compound: 	u8,	   		// F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
   					   						// 7 = inter, 8 = wet
   					   						// F1 Classic - 9 = dry, 10 = wet
   					   						// F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
   					   						// 15 = wet
	tyre_visual_compound: 	u8,       	// F1 visual (can be different from actual compound)
   											// 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
   											// F1 Classic – same as above
   											// F2 – same as above
    tyres_damage: 			[u8; 4],            	// Tyre damage (percentage)
    front_left_wing_damage: u8,      		// Front left wing damage (percentage)
    front_right_wing_damage:u8,     		// Front right wing damage (percentage)
    rear_wing_damage: 		u8,           	// Rear wing damage (percentage)
    engine_damage: 			u8,             		// Engine damage (percentage)
    gear_box_damage: 		u8,            	// Gear box damage (percentage)
    vehicle_fia_flags: 		i8,	   			// -1 = invalid/unknown, 0 = none, 1 = green
                                            // 2 = blue, 3 = yellow, 4 = red
    ers_store_energy: 		f32,           	// ERS energy store in Joules
    ers_deploy_mode: 		u8,            	// ERS deployment mode, 0 = none, 1 = low, 2 = medium
   					   						// 3 = high, 4 = overtake, 5 = hotlap
    ers_harvested_this_lap_mguk: 	f32,  	// ERS energy harvested this lap by MGU-K
    ers_harvested_this_lap_mguh: 	f32,  	// ERS energy harvested this lap by MGU-H
    ers_deployed_this_lap: 			f32,    // ERS energy deployed this lap
}

//-------------Motion Packet
//The motion packet gives physics data for all the cars being driven. There is additional data for the car being driven with the 
//goal of being able to drive a motion platform setup.
//N.B. For the normalised vectors below, to convert to float values divide by 32767.0f – 16-bit signed values are used to pack 
//the data and on the assumption that direction values are always between -1.0f and 1.0f.
//Frequency: Rate as specified in menus
//Size: 1343 bytes
//Version: 1

pub struct CarMotion {
    world_pos_x: f32,			//World Space pos
    world_pos_y: f32,
    world_pos_z: f32,
    world_vel_x: f32,			//World velocity
    world_vel_y: f32,
    world_vel_z: f32,
    world_fwd_dir_x: i16,	 	// World space forward X direction (normalised)
    world_fwd_dir_y: i16,		// World space forward Y direction (normalised)
    world_fwd_dir_z: i16,		// World space forward Z direction (normalised)
    world_right_dir_x: i16,		// World space right X direction (normalised)
    world_right_dir_y: i16,		// World space right Y direction (normalised)
    world_right_dir_z: i16,		// World space right Z direction (normalised)
    lateral_g: f32,
    longitudinal_g: f32,
    vertical_g: f32,
    yaw: f32,					//Radians
    pitch: f32,					//Radians
    roll: f32,					//Radians
}

pub struct MotionData {
    header: PacketHeader,               	// Header
    car_motion_data: [CarMotion; 20],    	// Data for all cars on track

    //Player car only
    suspension_pos: 	[f32; 4],
    suspension_vel: 	[f32; 4],
    suspension_acc:		[f32; 4],
    wheel_speed: 		[f32; 4],
    wheel_slip:			[f32; 4],
    local_vel_x:		f32,		//Local space
    local_vel_y:		f32,
    local_vel_z:		f32,
    angular_vel_x:		f32,
    angular_vel_y:		f32,
    angular_vel_z:		f32,
    angular_acc_x:		f32,
    angular_acc_y:		f32,
    angular_acc_z:		f32,
    front_wheels_angle: f32, //Radians
}
