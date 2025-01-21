// use windows::Win32::Foundation::HINSTANCE;
// use windows::Win32::UI::WindowsAndMessaging::*;

use eventghost::eg::classes::main_frame::MainFrame;

fn main() {
    // Get instance handle
    let instance = unsafe { GetModuleHandleA(None) }.expect("Failed to get module handle");

    // Create main window
    let mut main_frame = MainFrame::new(instance).expect("Failed to create main window");

    // Show window
    main_frame.show();

    // Message loop
    // let mut msg = MSG::default();
    // unsafe {
    //     while GetMessageA(&mut msg, None, 0, 0).into() {
    //         TranslateMessage(&msg);
    //         DispatchMessageA(&msg);
    //     }
    // }
} 