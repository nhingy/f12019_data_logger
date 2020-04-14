mod data_types {

struct PacketHeader {   
    packet_format: 		u16,		// 2019
    maj_version: 		u8,			// Game major version - "X.00"
    min_version: 		u8,			// Game minor version - "1.XX"
    packet_version: 	u8,			// Version of this packet type, all start from 1
    packet_id: 			u8,			// Identifier for the packet type, see below
    session_id: 		u64,		// Unique identifier for the session
    session_time: 		f32,		// Session timestamp
    frame_id:			u16,  		//Identifier for the frame the data was retrieved on. JP 2020 this spec had typo 'uint' guessing 16 bit as 8 bit not enought to contain num frames in game - could be u64 but seems excessive.
    player_car_index: 	u8			// Index of player's car in the array
}

struct CarTelemetryData {
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



}