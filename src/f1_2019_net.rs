use std::collections::HashMap;
use std::fmt;

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
    fn default() -> Self {PacketType::InvalidPacket} //ok to use as default and error state as byte can't be > 7
}

impl PacketType {
    pub fn getName(&self, event_type: PacketType) -> &str {
        match event_type {
            Motion      => return &"motion packet",
            Session     => return &"session packet", 
            Lap         => return &"lap packet",
            Event       => return &"event packet",
            Participant => return &"participant packet",
            Setup       => return &"setup packet",
            Telemetry   => return &"telemetry packet",
            CarStatus   => return &"car status packet",
            InvalidPacket => return &"invalid packet",   
        }
    }
}

//Design is that events are differentiated by way of a 4 byte array of ascii chars???
//We have no need to read these as chars so plan to create an instance of these enum vals with the int val of the ascii chars in question 
//Possible improvement calculate the 32 bit val of concating these bytes together as we could read the data stream as one 32 bit val rather than 4 8 bit vals
#[derive(Debug)]
pub enum EventType { 
    SessionStarted,     // Session Started "SSTA" S=83, S=83, T=84, A=65                    / TOT = 315
    SessionEnded,       // Session Ended "SEND" S=83, E=69, N=78, D=68                      / TOT = 298 
    FastestLap,         // Fastest Lap "FTLP" F=70, T=84, L=76, P=80                        / TOT = 310
    Retirement,         // Retirement "RTMT" R=82, T=84, M=77, T=84                         / TOT = 327
    DrsEnabled,         // DRS Enabled "DRSE" D=68, R=82, S=83, E=69                        / TOT = 302
    DrsDisabled,        // DRS Disabled "DRSD" D=68, R=82, S=83, D=68                       / TOT = 301
    TeamMateInPits,     // Team mate in the pits "TMPT" T=84, M=77, P=80, T=84              / TOT = 325
    ChequeredFlag,      // The chequered flag has been waved "CHQF" C=67, H=72, Q=81, F=70  / TOT = 290
    RaceWinner,         // The race winner is announced "RCWN" R=82, C=67, W=87, N=78       / TOT = 314
}

#[derive(Debug)]
pub struct Event {
	pub header: PacketHeader,
	pub event_type: EventType,
	pub car_idx: u8,		//The car if this event refers to a car
	pub lap_time: f32,		//Lap time if this event is fastest lap
}

impl Event {
    
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
    frame_id:			u32,  		// Identifier for the frame the data was retrieved on. JP 2020- working from stated total packet sizes this is u32
                                    // even though 32 bit int isn't in the data type list. Stated type is uint..? 
    player_car_index: 	u8			// Index of player's car in the array
}

impl PacketHeader {
    pub fn get_type(&self) -> PacketType {
        self.packet_type    
    }

    pub fn new(packet_format: u16, maj_version: u8, min_version: u8, packet_version: u8, 
            packet_type: PacketType, session_id: u64, session_time: f32, frame_id: u32, player_car_index: u8) -> Self {
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

#[derive(Debug, Default, Clone, Copy)]
pub struct CarTelemetry {				//66 bytes 
    pub car_speed: 			u16,                    // Speed of car in kilometres per hour
    pub throttle_pos: 		f32,                    // Amount of throttle applied (0.0 to 1.0)
    pub steering_pos:		f32,                    // Steering (-1.0 (full lock left) to 1.0 (full lock right))
    pub brake_pos: 			f32,                    // Amount of brake applied (0.0 to 1.0)
    pub clutch_pos:			u8,						// Amount of clutch applied (0 to 100)         
    pub gear: 				i8,						// Gear selected (1-8, N=0, R=-1)
    pub engine_rpm:			u16,					// Engine RPM
    pub drs_active: 		u8,						// 0 = off, 1 = on
    pub change_light_perc: 	u8,						// Rev lights indicator (percentage)
    pub brake_temps: 		[u16; 4],				// Brakes temperature (celsius)
    pub tyre_surface_temps: [u16; 4],				// Tyres surface temperature (celsius)
    pub tyre_inner_temps: 	[u16; 4],				// Tyres inner temperature (celsius)
    pub engine_temp: 		u16,					// Engine temperature (celsius)
    pub tyre_pressures: 	[f32; 4], 				// Tyres pressure (PSI)
    pub tyre_contact_types: [u8; 4] 				// Driving surface, see appendices
}

pub struct Telemetry {
    pub header:             PacketHeader, 
    pub car_telemetry_data: [CarTelemetry; 20],
    pub button_status:      u32,               // Bit flags specifying which buttons are being pressed currently - see appendices 
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MarshalZone {
	pub zone_start:	f32,	// Fraction (0..1) of way through the lap the marshal zone starts
	pub flag:		i8,		// -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red   
}

#[derive(Debug)]
pub struct SessionData {
	pub header:			PacketHeader,
	pub weather:		u8,					// Weather - 0 = clear, 1 = light cloud, 2 = overcast, 3 = light rain, 4 = heavy rain, 5 = storm
	pub track_temp:		i8,					// Track temp. in degrees celsius
	pub air_temp:		i8,					// Air temp. in degrees celsius
	pub total_laps:		u8,					// Total number of laps in this race
	pub track_len:		u16,				// Track length in metres
	pub session_type:	u8,					// 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short, P5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ, 10 = R, 11 = R2, 12 = Time Trial
	pub track_id:		i8,					// -1 for unknown, 0-21 for tracks, see appendix
	pub formual:		u8,					// Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2, 3 = F1 Generic
	pub session_ttl:	u16,				// Time left in session in seconds
	pub session_len:	u16,				// Session duration in seconds
	pub pit_spd_lim:	u8,					// Pit speed limit in kilometres per hour
	pub is_paused:		u8,					// Whether the game is paused
	pub is_spectating:	u8,					// Whether the player is spectating
	pub spectator_car:	u8,					// Index of the car being spectated
	pub sli_native:		u8,					// SLI Pro support, 0 = inactive, 1 = active
	pub num_zones:		u8,					// Number of marshal zones to follow
	pub zones:			[MarshalZone; 21], 	// List of marshal zones – max 21
	pub safety_car:		u8,					// 0 = no safety car, 1 = full safety car, 2 = virtual safety car
	pub is_network_game:u8,					// 0 = offline, 1 = online
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LapData {          
	pub last_lap: f32,			// Last lap time in seconds
	pub current_lap: f32,		// Current time around the lap in seconds
	pub best_lap: f32,			// Best lap time of the session in seconds
	pub best_sec_1: f32,		// Sector 1 time in seconds
	pub best_sec_2: f32,		// Sector 2 time in seconds
	pub lap_distance: f32,		// Distance vehicle is around current lap in metres – could be negative if line hasn’t been crossed yet
	pub total_distance: f32,	// Total distance travelled in session in metres – could be negative if line hasn’t been crossed yet
	pub safety_car_delta: f32,	// Delta in seconds for safety car
	pub position: u8,			// Car race position
	pub lap_num: u8,			// Current lap number
	pub pit_status: u8,			// 0 = none, 1 = pitting, 2 = in pit area
	pub sector: u8,				// 0 = sector1, 1 = sector2, 2 = sector3
	pub is_lap_valid: u8,		// Current lap invalid - 0 = valid, 1 = invalid
	pub penalties: u8,			// Accumulated time penalties in seconds to be added
	pub grid_position: u8,		// Grid position the vehicle started the race in
	pub driver_status: u8,		// Status of driver - 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
	pub result_status: u8,		// Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = disqualified, 5 = not classified, 6 = retired
}

#[derive(Debug)]
pub struct Lap	{
    pub header:	PacketHeader,     // Header
    pub lap_data: [LapData; 20]  // Lap data for all cars on track
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
//    = note: required for the cast to the object type `dyn std::fmt::Debug
#[derive(Clone, Copy)]
pub struct ParticipantData {
    pub ai_controlled: 	u8,         // Whether the vehicle is AI (1) or Human (0) controlled
    pub driver_id: 		u8,			// Driver id - see appendix
    pub team_id: 		u8,         // Team id - see appendix
    pub race_number: 	u8,         // Race number of the car
    pub nationality: 	u8,         // Nationality of the driver
    pub name: 			[u8; 48],   // Name of participant in UTF-8 format – null terminated Will be truncated with … (U+2026) if too long
    pub priv_telemetry: u8,         // The player's UDP setting, 0 = restricted, 1 = public
}

impl ParticipantData {
    pub fn new() -> Self {
        ParticipantData {
            ai_controlled:  0,
            driver_id:      0,
            team_id:        0,
            race_number:    0,
            nationality:    0,
            name:           [0u8; 48],
            priv_telemetry: 0,
        }
    }
}

pub struct Participants {
    pub header: 			PacketHeader,			// Header
    pub num_cars_active:	u8,						// Number of active cars in the data – should match number of cars on HUD
    pub participant_data:	[ParticipantData; 20],	// Data for all cars
}

//Car Setups Packet
//This packet details the car setups for each vehicle in the session. Note that in multiplayer games, other player cars will appear as blank, 
//you will only be able to see your car setup and AI cars.
//Frequency: 2 per second
//Size: 843 bytes
//Version: 1
#[derive(Default, Clone, Copy)]
pub struct CarSetupData {
    pub front_wing: 			u8,             // Front wing aero
    pub rear_wing:				u8,             // Rear wing aero
    pub on_throttle: 			u8,             // Differential adjustment on throttle (percentage)
    pub off_throttle: 			u8,             // Differential adjustment off throttle (percentage)
    pub front_camber: 			f32,            // Front camber angle (suspension geometry)
    pub rear_camber: 			f32,            // Rear camber angle (suspension geometry)
    pub front_toe: 				f32,            // Front toe angle (suspension geometry)
    pub rear_toe: 				f32,            // Rear toe angle (suspension geometry)
    pub front_suspension: 		u8,         	// Front suspension
    pub rear_suspension:		u8,           	// Rear suspension
    pub front_anti_roll_bar: 	u8,         	// Front anti-roll bar
    pub rear_anti_roll_bar: 	u8,          	// Front anti-roll bar
    pub front_suspension_height:u8,    			// Front ride height
    pub rear_suspension_height: u8,     		// Rear ride height
    pub brake_pressure: 		u8,            	// Brake pressure (percentage)
    pub brake_bias: 			u8,             // Brake bias (percentage)
    pub front_tyre_pressure: 	f32,       		// Front tyre pressure (PSI)
    pub rear_tyre_pressure: 	f32,         	// Rear tyre pressure (PSI)
    pub ballast: 				u8,             // Ballast
    pub fuel_load: 				f32,            // Fuel load
}

pub struct CarSetups {
    pub header: PacketHeader,
    pub car_setups: [CarSetupData; 20],
}

//---------------Car Status Packet
//This packet details car statuses for all the cars in the race. It includes values such as the damage readings on the car.
//Frequency: Rate as specified in menus
//Size: 1143 bytes
//Version: 1 
pub struct CarStatusData {
	pub header: 			PacketHeader,			// Header
    pub car_status_data: 	[CarStatus; 20],		// Array of car status'
}

#[derive(Default, Clone, Copy)] 
pub struct CarStatus {                      //56 bytes
    pub traction_control: 		u8,         // 0 (off) - 2 (high)
    pub anti_lock_brakes: 		u8,         // 0 (off) - 1 (on)
    pub fuel_mix: 				u8,         // Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    pub front_brake_bias: 		u8,         // Front brake bias (percentage)
    pub pit_limiter_status: 	u8,       	// Pit limiter status - 0 = off, 1 = on
    pub fuel_in_tank: 			f32,        // Current fuel mass
	pub fuel_capacity: 			f32,        // Fuel capacity
	pub fuel_remaining_laps:	f32,       	// Fuel remaining in terms of laps (value on MFD)
    pub max_rpm: 				u16,        // Cars max RPM, point of rev limiter
    pub idle_rpm:				u16,        // Cars idle RPM
    pub max_gears: 				u8,         // Maximum number of gears
    pub drs_allowed: 			u8,         // 0 = not allowed, 1 = allowed, -1 = unknown
    pub tyres_wear: 			[u8; 4],    // Tyre wear percentage
    pub actual_tyre_compound: 	u8,	   		// F1 Modern - 16 = C5, 17 = C4, 18 = C3, 19 = C2, 20 = C1
   					   						// 7 = inter, 8 = wet
   					   						// F1 Classic - 9 = dry, 10 = wet
   					   						// F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
   					   						// 15 = wet
	pub tyre_visual_compound: 	u8,       	// F1 visual (can be different from actual compound)
   											// 16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
   											// F1 Classic – same as above
   											// F2 – same as above
    pub tyres_damage: 			[u8; 4],            	// Tyre damage (percentage)
    pub front_left_wing_damage: u8,      		// Front left wing damage (percentage)
    pub front_right_wing_damage:u8,     		// Front right wing damage (percentage)
    pub rear_wing_damage: 		u8,           	// Rear wing damage (percentage)
    pub engine_damage: 			u8,             		// Engine damage (percentage)
    pub gear_box_damage: 		u8,            	// Gear box damage (percentage)
    pub vehicle_fia_flags: 		i8,	   			// -1 = invalid/unknown, 0 = none, 1 = green
                                            // 2 = blue, 3 = yellow, 4 = red
    pub ers_store_energy: 		f32,           	// ERS energy store in Joules
    pub ers_deploy_mode: 		u8,            	// ERS deployment mode, 0 = none, 1 = low, 2 = medium
   					   						// 3 = high, 4 = overtake, 5 = hotlap
    pub ers_harvested_this_lap_mguk: 	f32,  	// ERS energy harvested this lap by MGU-K
    pub ers_harvested_this_lap_mguh: 	f32,  	// ERS energy harvested this lap by MGU-H
    pub ers_deployed_this_lap: 			f32,    // ERS energy deployed this lap
}

//-------------Motion Packet
//The motion packet gives physics data for all the cars being driven. There is additional data for the car being driven with the 
//goal of being able to drive a motion platform setup.
//N.B. For the normalised vectors below, to convert to float values divide by 32767.0f – 16-bit signed values are used to pack 
//the data and on the assumption that direction values are always between -1.0f and 1.0f.
//Frequency: Rate as specified in menus
//Size: 1343 bytes
//Version: 1
#[derive(Default, Clone, Copy)]
pub struct CarMotion {
    pub world_pos_x: f32,			//World Space pos
    pub world_pos_y: f32,
    pub world_pos_z: f32,
    pub world_vel_x: f32,			//World velocity
    pub world_vel_y: f32,
    pub world_vel_z: f32,
    pub world_fwd_dir_x: i16,	 	// World space forward X direction (normalised)
    pub world_fwd_dir_y: i16,		// World space forward Y direction (normalised)
    pub world_fwd_dir_z: i16,		// World space forward Z direction (normalised)
    pub world_right_dir_x: i16,		// World space right X direction (normalised)
    pub world_right_dir_y: i16,		// World space right Y direction (normalised)
    pub world_right_dir_z: i16,		// World space right Z direction (normalised)
    pub lateral_g: f32,
    pub longitudinal_g: f32,
    pub vertical_g: f32,
    pub yaw: f32,					//Radians
    pub pitch: f32,					//Radians
    pub roll: f32,					//Radians
}

pub struct MotionData {
    pub header: PacketHeader,               	// Header
    pub car_motion_data: [CarMotion; 20],    	// Data for all cars on track

    //Player car only
    pub suspension_pos: 	[f32; 4],
    pub suspension_vel: 	[f32; 4],
    pub suspension_acc:		[f32; 4],
    pub wheel_speed: 		[f32; 4],
    pub wheel_slip:			[f32; 4],
    pub local_vel_x:		f32,		//Local space
    pub local_vel_y:		f32,
    pub local_vel_z:		f32,
    pub angular_vel_x:		f32,
    pub angular_vel_y:		f32,
    pub angular_vel_z:		f32,
    pub angular_acc_x:		f32,
    pub angular_acc_y:		f32,
    pub angular_acc_z:		f32,
    pub front_wheels_angle: f32, //Radians
}

pub fn init_teams(map: &mut HashMap<usize, &str>) {
    map.insert(0, "Mercedes");
    map.insert(1, "Ferrari");
    map.insert(2, "Red Bull Racing");
    map.insert(3, "Williams");
    map.insert(4, "Racing Point");
    map.insert(5, "Renault");
    map.insert(6, "Toro Rosso");
    map.insert(7, "Haas");
    map.insert(8, "McLaren");
    map.insert(9, "Alfa Romeo");
    map.insert(10, "McLaren 1988");
    map.insert(11, "McLaren 1991");
    map.insert(12, "Williams 1992");
    map.insert(13, "Ferrari 1995");
    map.insert(14, "Williams 1996");
    map.insert(15, "McLaren 1998");
    map.insert(16, "Ferrari 2002");
    map.insert(17, "Ferrari 2004");
    map.insert(18, "Renault 2006");
    map.insert(19, "Ferrari 2007");
    map.insert(21, "Red Bull 2010");
    map.insert(22, "Ferrari 1976");
    map.insert(23, "ART Grand Prix");
    map.insert(24, "Campos Vexatec Racing");
    map.insert(25, "Carlin");
    map.insert(26, "Charouz Racing System");
    map.insert(27, "DAMS");
    map.insert(28, "Russian Time");
    map.insert(29, "MP Motorsport");
    map.insert(30, "Pertamina");
    map.insert(31, "McLaren 1990");
    map.insert(32, "Trident");
    map.insert(33, "BWT Arden");
    map.insert(34, "McLaren 1976");
    map.insert(35, "Lotus 1972");
    map.insert(36, "Ferrari 1979");
    map.insert(37, "McLaren 1982");
    map.insert(38, "Williams 2003");
    map.insert(39, "Brawn 2009");
    map.insert(40, "Lotus 1978");
    map.insert(42, "Art GP ’19");
    map.insert(43, "Campos ’19");
    map.insert(44, "Carlin ’19");
    map.insert(45, "Sauber Junior Charouz ’19");
    map.insert(46, "Dams ’19");
    map.insert(47, "Uni-Virtuosi ‘19");
    map.insert(48, "MP Motorsport ‘19");
    map.insert(49, "Prema ’19");
    map.insert(50, "Trident ’19");
    map.insert(51, "Arden ’19");
    map.insert(63, "Ferrari 1990");
    map.insert(64, "McLaren 2010");
    map.insert(65, "Ferrari 2010");
}   

pub fn init_drivers(map: &mut HashMap<usize, &str>) {
    map.insert(0,  "Carlos Sainz");
    map.insert(37, "Peter Belousov");
    map.insert(1,  "Daniil Kvyat");
    map.insert(38, "Klimek Michalski");
    map.insert(70, "Rashid Nair");
    map.insert(2,  "Daniel Ricciardo");
    map.insert(39, "Santiago Moreno");
    map.insert(71, "Jack Tremblay");
    map.insert(6,  "Kimi Räikkönen");
    map.insert(40, "Benjamin Coppens");
    map.insert(74, "Antonio Giovinazzi");
    map.insert(7,  "Lewis Hamilton");
    map.insert(41, "Noah Visser");
    map.insert(75, "Robert Kubica");
    map.insert(9,  "Max Verstappen");
    map.insert(42, "Gert Waldmuller");
    map.insert(78, "Nobuharu Matsushita");
    map.insert(10, "Nico Hulkenburg");
    map.insert(43, "Julian Quesada");
    map.insert(79, "Nikita Mazepin");
    map.insert(11, "Kevin Magnussen");
    map.insert(44, "Daniel Jones");
    map.insert(80, "Guanya Zhou");
    map.insert(12, "Romain Grosjean");
    map.insert(45, "Artem Markelov");
    map.insert(81, "Mick Schumacher");
    map.insert(13, "Sebastian Vettel");
    map.insert(46, "Tadasuke Makino");
    map.insert(82, "Callum Ilott");
    map.insert(14, "Sergio Perez");
    map.insert(47, "Sean Gelael");
    map.insert(83, "Juan Manuel Correa");
    map.insert(15, "Valtteri Bottas");
    map.insert(48, "Nyck De Vries");
    map.insert(84, "Jordan King");
    map.insert(19, "Lance Stroll");
    map.insert(49, "Jack Aitken");
    map.insert(85, "Mahaveer Raghunathan");
    map.insert(20, "Arron Barnes");
    map.insert(50, "George Russell");
    map.insert(86, "Tatiana Calderon");
    map.insert(21, "Martin Giles");
    map.insert(51, "Maximilian Günther");
    map.insert(87, "Anthoine Hubert");
    map.insert(22, "Alex Murray");
    map.insert(52, "Nirei Fukuzumi");
    map.insert(88, "Guiliano Alesi");
    map.insert(23, "Lucas Roth");
    map.insert(53, "Luca Ghiotto");
    map.insert(89, "Ralph Boschung");
    map.insert(24, "Igor Correia");
    map.insert(54, "Lando Norris");
    map.insert(25, "Sophie Levasseur");
    map.insert(55, "Sérgio Sette Câmara");
    map.insert(26, "Jonas Schiffer");
    map.insert(56, "Louis Delétraz");
    map.insert(27, "Alain Forest");
    map.insert(57, "Antonio Fuoco");
    map.insert(28, "Jay Letourneau");
    map.insert(58, "Charles Leclerc");
    map.insert(29, "Esto Saari");
    map.insert(59, "Pierre Gasly");
    map.insert(30, "Yasar Atiyeh");
    map.insert(62, "Alexander Albon");
    map.insert(31, "Callisto Calabresi");
    map.insert(63, "Nicholas Latifi");
    map.insert(32, "Naota Izum");
    map.insert(64, "Dorian Boccolacci");
    map.insert(33, "Howard Clarke");
    map.insert(65, "Niko Kari");
    map.insert(34, "Wilheim Kaufmann");
    map.insert(66, "Roberto Merhi");
    map.insert(35, "Marie Laursen");
    map.insert(67, "Arjun Maini");
    map.insert(36, "Flavio Nieves");
    map.insert(68, "Alessio Lorandi"); 
    map.insert(69, "Ruben Meijer");
}

pub fn init_tracks(map: &mut HashMap<usize, &str>) {
    map.insert(0 , "Melbourne");
    map.insert(1 , "Paul Ricard");
    map.insert(2 , "Shanghai");
    map.insert(3 , "Sakhir Bahrain");
    map.insert(4 , "Catalunya");
    map.insert(5 , "Monaco");
    map.insert(6 , "Montreal");
    map.insert(7 , "Silverstone");
    map.insert(8 , "Hockenheim");
    map.insert(9 , "Hungaroring");
    map.insert(10, "Spa");
    map.insert(11, "Monza");
    map.insert(12, "Singapore");
    map.insert(13, "Suzuka");
    map.insert(14, "Abu Dhabi");
    map.insert(15, "Texas");
    map.insert(16, "Brazil");
    map.insert(17, "Austria");
    map.insert(18, "Sochi");
    map.insert(19, "Mexico");
    map.insert(20, "Baku Azerbaijan");
    map.insert(21, "Sakhir Short");
    map.insert(22, "Silverstone Short");
    map.insert(23, "Texas Short");
    map.insert(24, "Suzuka Short");
}

pub fn init_countries(map: &mut HashMap<usize, &str>) { 
    map.insert(1 ,  "American");
    map.insert(31,  "Greek");
    map.insert(61,  "Panamanian");
    map.insert(2 ,  "Argentinean");
    map.insert(32,  "Guatemalan");
    map.insert(62,  "Paraguayan");
    map.insert(3 ,  "Australian");
    map.insert(33,  "Honduran");
    map.insert(63,  "Peruvian");
    map.insert(4 ,  "Austrian");
    map.insert(34,  "Hong Konger");
    map.insert(64,  "Polish");
    map.insert(5 ,  "Azerbaijani");
    map.insert(35,  "Hungarian");
    map.insert(65,  "Portuguese");
    map.insert(6 ,  "Bahraini");
    map.insert(36,  "Icelander");
    map.insert(66,  "Qatari");
    map.insert(7 ,  "Belgian");
    map.insert(37,  "Indian");
    map.insert(67,  "Romanian");
    map.insert(8 ,  "Bolivian");
    map.insert(38,  "Indonesian");
    map.insert(68,  "Russian");
    map.insert(9 ,  "Brazilian");
    map.insert(39,  "Irish");
    map.insert(69,  "Salvadoran");
    map.insert(10,  "British");
    map.insert(40,  "Israeli");
    map.insert(70,  "Saudi");
    map.insert(11,  "Bulgarian");
    map.insert(41,  "Italian");
    map.insert(71,  "Scottish");
    map.insert(12,  "Cameroonian");
    map.insert(42,  "Jamaican");
    map.insert(72,  "Serbian");
    map.insert(13,  "Canadian");
    map.insert(43,  "Japanese");
    map.insert(73,  "Singaporean");
    map.insert(14,  "Chilean");
    map.insert(44,  "Jordanian");
    map.insert(74,  "Slovakian");
    map.insert(15,  "Chinese");
    map.insert(45,  "Kuwaiti");
    map.insert(75,  "Slovenian");
    map.insert(16,  "Colombian");
    map.insert(46,  "Latvian");
    map.insert(76,  "South Korean");
    map.insert(17,  "Costa Rican");
    map.insert(47,  "Lebanese");
    map.insert(77,  "South African");
    map.insert(18,  "Croatian");
    map.insert(48,  "Lithuanian");
    map.insert(78,  "Spanish");
    map.insert(19,  "Cypriot");
    map.insert(49,  "Luxembourger");
    map.insert(79,  "Swedish");
    map.insert(20,  "Czech");
    map.insert(50,  "Malaysian");
    map.insert(80,  "Swiss");
    map.insert(21,  "Danish");
    map.insert(51,  "Maltese");
    map.insert(81,  "Thai");
    map.insert(22,  "Dutch");
    map.insert(52,  "Mexican");
    map.insert(82,  "Turkish");
    map.insert(23,  "Ecuadorian");
    map.insert(53,  "Monegasque");
    map.insert(83,  "Uruguayan");
    map.insert(24,  "English");
    map.insert(54,  "New Zealander");
    map.insert(84,  "Ukrainian");
    map.insert(25,  "Emirian");
    map.insert(55,  "Nicaraguan");
    map.insert(85,  "Venezuelan");
    map.insert(26,  "Estonian");
    map.insert(56,  "North Korean");
    map.insert(86,  "Welsh");
    map.insert(27,  "Finnish");
    map.insert(57,  "Northern Irish");
    map.insert(28,  "French");
    map.insert(58,  "Norwegian");
    map.insert(29,  "German");
    map.insert(59,  "Omani");
    map.insert(30,  "Ghanaian");
    map.insert(60,  "Pakistani");
}

pub fn init_surfaces(map: &mut HashMap<usize, &str>) {
    map.insert(0 , "Tarmac");
    map.insert(1 , "Rumble strip");
    map.insert(2 , "Concrete");
    map.insert(3 , "Rock");
    map.insert(4 , "Gravel");
    map.insert(5 , "Mud");
    map.insert(6 , "Sand");
    map.insert(7 , "Grass");
    map.insert(8 , "Water");
    map.insert(9 , "Cobblestone");
    map.insert(10, "Metal"); 
    map.insert(11, "Ridged");
}

pub fn init_button_flags(map: &mut HashMap<&str, &str>) { //buttons being pressed by player
    map.insert("0x0001",  "Cross or A");
    map.insert("0x0002",  "Triangle or Y");
    map.insert("0x0004",  "Circle or B");
    map.insert("0x0008",  "Square or X");
    map.insert("0x0010",  "D-pad Left");
    map.insert("0x0020",  "D-pad Right");
    map.insert("0x0040",  "D-pad Up");
    map.insert("0x0080",  "D-pad Down");
    map.insert("0x0100",  "Options or Menu");
    map.insert("0x0200",  "L1 or LB");
    map.insert("0x0400",  "R1 or RB");
    map.insert("0x0800",  "L2 or LT");
    map.insert("0x1000",  "R2 or RT");
    map.insert("0x2000",  "Left Stick Click");
    map.insert("0x4000",  "Right Stick Click");
}  