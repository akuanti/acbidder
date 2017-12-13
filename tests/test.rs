extern crate acbidder;
extern crate rocket;

use rocket::local::Client;
use rocket::http::{Status, ContentType};

use acbidder::server::start_server;


// struct Bid {
// 	#[form(field = "type")]
// 	bid_type: String,
// 	publisher: String,
// 	user_qulaity: u8
// }

// struct BidResponse {
// 	#[form(field = "type")]
// 	bid_type: String,
// 	bid: u32
// }

//simple request and response
#[test]
fn simple_server() {
    acbidder::server::initialize_test();
	let client = Client::new(start_server()).unwrap();

    
    let mut req = client.post("/acq/nba.com")
        .header(ContentType::JSON)
        .body(r#"{"bid_type": "BID_REQ", "publisher": "nba.com", "user_quality": 2}"#)
        .dispatch();
    
    assert_eq!(req.status(), Status::Ok);
    assert_eq!(req.body_string(),
    	Some(String::from("{\"bid_type\":\"BID_RESP\",\"bid\":5}")));
		
	let mut req = client.post("/acq/nba.com")
        .header(ContentType::JSON)
        .body(r#"{"bid_type": "BID_REQ", "publisher": "nba.com", "user_quality": 1}"#)
        .dispatch();
    
    assert_eq!(req.status(), Status::Ok);
    assert_eq!(req.body_string(),
    	Some(String::from("{\"bid_type\":\"BID_RESP\",\"bid\":6}")));
}

#[test]
fn request_not_found() {
    let client = Client::new(start_server()).unwrap();
    let mut req = client.post("/acq/fakenews.com")
        .header(ContentType::JSON)
        .body(r#"{"bid_type": "BID_REQ", "publisher": "fakenews.com", "user_quality": 2}"#)
        .dispatch();

    assert_eq!(req.status(), Status::NotFound);
    assert_eq!(req.body_string(), None);
}

//checking inproper formatting error
#[test]
fn json_client_error() {
	let client = Client::new(start_server()).unwrap();

	let req = client.post("/acq/nyt.com")
	    .body(r#"{"bid_type": "BID_REQ", "publisher": "nyt", "user_quality": 2}"#)
        .dispatch();

    assert_eq!(req.status(), Status::NotFound);
}

//checking improper formatting error
#[test]
#[should_panic]
fn json_client_error2() {
	let client = Client::new(start_server()).unwrap();

	let req = client.post("/acq/:nyt.com")
	    .body(r#"{"bid_type": "BID_REQ", "publisher": "nyt", "user_quality": 2}"#)
        .dispatch();

    assert_eq!(req.status(), Status::Ok);
}
