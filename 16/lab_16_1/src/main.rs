use std::ptr;
use winapi::um::winuser;

fn main(){
    let message = "An error has occurred.";
let caption = "Error";
let flags = winuser::MB_ABORTRETRYIGNORE;


let result = unsafe {
    winuser::MessageBoxW(ptr::null_mut(), message.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr(), caption.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr(), flags | winuser::MB_ICONERROR)
};

match result {
    3 => println!("Abort"),
    4 => println!("Retry"),
    5 => println!("Ignore"),
    _ => println!("Unknown"),
}
}
