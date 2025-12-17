use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::ColorSystem::*;

#[repr(C)]
struct GammaRamp {
    red:   [u16; 256],
    green: [u16; 256],
    blue:  [u16; 256],
}

#[unsafe(no_mangle)]
pub extern "C" fn set_gamma(gamma: f64) {
    unsafe {
        let mut ramp = GammaRamp {
            red: [0; 256],
            green: [0; 256],
            blue: [0; 256],
        };

        for i in 0..256 {
            let v = ((i as f64 / 255.0).powf(1.0 / gamma) * 65535.0)
                .clamp(0.0, 65535.0) as u16;
            ramp.red[i] = v;
            ramp.green[i] = v;
            ramp.blue[i] = v;
        }

        let hdc = GetDC(None);
        let _ = SetDeviceGammaRamp(hdc, &ramp as *const _ as _);
    }
}