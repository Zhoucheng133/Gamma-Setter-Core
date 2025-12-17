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
        let exponent = 4f64.powf(gamma); 

        for i in 0..256 {
            let val = ((i as f64 / 256.0).powf(exponent) * 65535.0 + 0.5)
                .clamp(0.0, 65535.0) as u16;
            
            ramp.red[i] = val;
            ramp.green[i] = val;
            ramp.blue[i] = val;
        }

        let hdc = GetDC(None);
        if !hdc.is_invalid() {
            let _ = SetDeviceGammaRamp(hdc, &ramp as *const _ as _);
            // 修复资源泄漏：释放 DC
            ReleaseDC(HWND(0), hdc); 
        }
    }
}