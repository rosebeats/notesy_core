use std::ffi::c_void;
use std::{ptr};
use prost::Message;

// Include the `messaging` module, which is generated from threading.proto.
pub mod messaging {
    include!(concat!(env!("OUT_DIR"), "/notesy.messaging.rs"));
}

#[no_mangle]
pub extern "C" fn test_thread(context_ptr: *mut c_void) -> *mut c_void {
    // get context from main thread and set up a connection
    let ctx = unsafe{ zmq::Context::from_raw(context_ptr) };
    let main_socket = ctx.socket(zmq::PAIR).unwrap();
    main_socket.connect("inproc://test").unwrap();
    
    let mut encoded_msg = Vec::new();
    let mut message = messaging::ServerMsg::default();
    message.test = "this is a test".to_string();
    message.encode(&mut encoded_msg).unwrap();
    main_socket.send(encoded_msg, 0).unwrap();
    
    loop {
        let encoded_msg = main_socket.recv_bytes(0).unwrap();
        let message = messaging::ClientMsg::decode(&encoded_msg[..]).unwrap();
        if message.shutdown {
            println!("shutting down");
            break;
        }
    }
    ptr::null_mut()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
