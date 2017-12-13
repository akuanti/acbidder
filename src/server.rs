extern crate rocket;

extern crate serde_json;
extern crate serde_derive;
extern crate serde;

use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;

use rocket_contrib::Json;


use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::fairing::{Fairing, Info, Kind, AdHoc};

use rocket::{Request, Response, State};
use rocket::http::Status;


use std::io::Cursor;

use web3;


//bid request format to convert form json
#[derive(Serialize, Deserialize, FromForm)]
struct Bid {
	bid_type: String,
	publisher: String,
	user_quality: u8
}

//bid response format to convert to json
#[derive(Debug, Serialize, Deserialize, FromForm)]
struct BidResponse {
	bid_type: String,
	bid: u32
}

//the current bid count being returned
#[derive(Default)]
struct BidCounter {
    count: AtomicUsize
}

//middleware
impl Fairing for BidCounter {
    //initialize fairing
    fn info(&self) -> Info {
        Info {
            name: "POST BidCounter",
            kind: Kind::Response
        }
    }
	
	//checks if domain was not white listed
    fn on_response(&self, request: &Request, response: &mut Response) {
        if response.status() == Status::NotFound {
		    return
		}
		let bid_resp = format!("{:?}",response.body_string().unwrap());
		let len = bid_resp.len()-2;
		let id = &bid_resp[36..len];
		let id: u32 = id.parse().unwrap();
		if id > 0 {
		    let bid_resp = BidResponse {
			    bid_type: String::from("BID_RESP"),
				bid: id
			};
			let body = serde_json::to_string(&bid_resp).unwrap();
			
			let body = String::from(body);
			response.set_sized_body(Cursor::new(body));
			return
		}
		response.set_status(Status::NotFound);
		return
    }
}



//writing to the bid request file to keep record of actions
fn write_to_file_bid_req(bid: &Bid) {
    let mut f = match OpenOptions::new().append(true).open("BID_REQ.txt") {
    	Err(err) => File::create("BID_REQ.txt").expect("File problems."),
    	Ok(f) => f,
    };

    f.write(serde_json::to_string_pretty(&bid).unwrap().as_bytes())
        .expect("Could not write!");
    f.write(b"\n")
        .expect("Could not write!");
}

//write to bid response file to keep record of responses
fn write_to_file_bid_resp(bid_resp: &BidResponse) {
    let mut f = match OpenOptions::new().append(true).open("BID_RESP.txt") {
    	Err(err) => File::create("BID_RESP.txt").expect("File problems."),
    	Ok(f) => f,
    };

    f.write(serde_json::to_string_pretty(&bid_resp).unwrap().as_bytes())
        .expect("Could not write!");
    f.write(b"\n")
        .expect("Could not write!");
}

//aquire request handling
#[post("/acq/<domain>", format = "application/json", data = "<bid>")]
fn acq(domain: String, bid: Json<Bid>, bid_counter: State<BidCounter>) -> Json<BidResponse> {
    const RPC_ENDPOINT: &str = "http://localhost:8545";
    let (_eloop, http) = web3::transports::Http::new(RPC_ENDPOINT)
        .unwrap();
    let web3 = web3::Web3::new(http);
    let adchain_registry = super::adchain_registry::RegistryInstance::new(&web3);
    if adchain_registry.is_in_registry(&domain[..]) {
    	let bid_response_json = Json(BidResponse {
		    bid_type: format!("BID_RESP"),
		    bid: (bid_counter.count.load(Ordering::Relaxed) as u32)
	    });
		let bid_response_json_recording = Json(BidResponse {
		    bid_type: format!("BID_RESP"),
		    bid: (bid_counter.count.load(Ordering::Relaxed) as u32)
	    });
		write_to_file_bid_req(&bid.into_inner());
		bid_counter.count.fetch_add(1, Ordering::Relaxed);
		write_to_file_bid_resp(&bid_response_json_recording.into_inner());
		return bid_response_json;
    }

	Json(BidResponse {
		bid_type: format!("BID_RESP"),
		bid: 0
	})
}

//tell client how to use service
#[get("/")]
fn instructions() -> String{
    format!("Use json with bid_type, publisher, user_quality.")
}

//mounting appropriate routers to rocket
pub fn start_server() -> rocket::Rocket {
	rocket::ignite().mount("/", routes![instructions, acq])
	    .attach(BidCounter::default())
		.attach(AdHoc::on_attach(|rocket| {
		    //let val = rocket.config().get_int("count").unwrap() as usize;
			let mut f = OpenOptions::new().read(true).open("RESPONSE_NUMBER.txt").unwrap();
			let mut content = String::new();
			f.read_to_string(&mut content).expect("File Read!");
			let val: usize = content.trim().parse().unwrap();
		    Ok(rocket.manage(BidCounter {count: AtomicUsize::new(val)}))
		}))
}

//initialize the bid response id file for testing purposes
pub fn initialize_test() {
    let mut f = match OpenOptions::new().write(true).open("RESPONSE_NUMBER.txt") {
	    Err(err) => File::create("RESPONSE_NUMBER.txt").expect("File problems."),
		Ok(f) => f,
	};
	f.write(b"5").expect("Could not write!");
}