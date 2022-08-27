/// Single virtual key code.
#[repr(transparent)]
pub struct KeyCode(u32);

#[allow(missing_docs)]
impl KeyCode {
    pub const VK_LBUTTON: Self = Self(0x01);
    pub const VK_RBUTTON: Self = Self(0x02);
    pub const VK_CANCEL: Self = Self(0x03);
    pub const VK_MBUTTON: Self = Self(0x04);
    pub const VK_XBUTTON1: Self = Self(0x05);
    pub const VK_XBUTTON2: Self = Self(0x06);
    pub const VK_BACK: Self = Self(0x08);
    pub const VK_TAB: Self = Self(0x09);
    pub const VK_CLEAR: Self = Self(0x0C);
    pub const VK_RETURN: Self = Self(0x0D);
    pub const VK_SHIFT: Self = Self(0x10);
    pub const VK_CONTROL: Self = Self(0x11);
    pub const VK_MENU: Self = Self(0x12);
    pub const VK_PAUSE: Self = Self(0x13);
    pub const VK_CAPITAL: Self = Self(0x14);
    pub const VK_KANA: Self = Self(0x15);
    pub const VK_HANGUEL: Self = Self(0x15);
    pub const VK_HANGUL: Self = Self(0x15);
    pub const VK_JUNJA: Self = Self(0x17);
    pub const VK_FINAL: Self = Self(0x18);
    pub const VK_HANJA: Self = Self(0x19);
    pub const VK_KANJI: Self = Self(0x19);
    pub const VK_ESCAPE: Self = Self(0x1B);
    pub const VK_CONVERT: Self = Self(0x1C);
    pub const VK_NONCONVERT: Self = Self(0x1D);
    pub const VK_ACCEPT: Self = Self(0x1E);
    pub const VK_MODECHANGE: Self = Self(0x1F);
    pub const VK_SPACE: Self = Self(0x20);
    pub const VK_PRIOR: Self = Self(0x21);
    pub const VK_NEXT: Self = Self(0x22);
    pub const VK_END: Self = Self(0x23);
    pub const VK_HOME: Self = Self(0x24);
    pub const VK_LEFT: Self = Self(0x25);
    pub const VK_UP: Self = Self(0x26);
    pub const VK_RIGHT: Self = Self(0x27);
    pub const VK_DOWN: Self = Self(0x28);
    pub const VK_SELECT: Self = Self(0x29);
    pub const VK_PRINT: Self = Self(0x2A);
    pub const VK_EXECUTE: Self = Self(0x2B);
    pub const VK_SNAPSHOT: Self = Self(0x2C);
    pub const VK_INSERT: Self = Self(0x2D);
    pub const VK_DELETE: Self = Self(0x2E);
    pub const VK_HELP: Self = Self(0x2F);
    pub const VK_0: Self = Self(0x30);
    pub const VK_1: Self = Self(0x31);
    pub const VK_2: Self = Self(0x32);
    pub const VK_3: Self = Self(0x33);
    pub const VK_4: Self = Self(0x34);
    pub const VK_5: Self = Self(0x35);
    pub const VK_6: Self = Self(0x36);
    pub const VK_7: Self = Self(0x37);
    pub const VK_8: Self = Self(0x38);
    pub const VK_9: Self = Self(0x39);
    pub const VK_A: Self = Self(0x41);
    pub const VK_B: Self = Self(0x42);
    pub const VK_C: Self = Self(0x43);
    pub const VK_D: Self = Self(0x44);
    pub const VK_E: Self = Self(0x45);
    pub const VK_F: Self = Self(0x46);
    pub const VK_G: Self = Self(0x47);
    pub const VK_H: Self = Self(0x48);
    pub const VK_I: Self = Self(0x49);
    pub const VK_J: Self = Self(0x4A);
    pub const VK_K: Self = Self(0x4B);
    pub const VK_L: Self = Self(0x4C);
    pub const VK_M: Self = Self(0x4D);
    pub const VK_N: Self = Self(0x4E);
    pub const VK_O: Self = Self(0x4F);
    pub const VK_P: Self = Self(0x50);
    pub const VK_Q: Self = Self(0x51);
    pub const VK_R: Self = Self(0x52);
    pub const VK_S: Self = Self(0x53);
    pub const VK_T: Self = Self(0x54);
    pub const VK_U: Self = Self(0x55);
    pub const VK_V: Self = Self(0x56);
    pub const VK_W: Self = Self(0x57);
    pub const VK_X: Self = Self(0x58);
    pub const VK_Y: Self = Self(0x59);
    pub const VK_Z: Self = Self(0x5A);
    pub const VK_LWIN: Self = Self(0x5B);
    pub const VK_RWIN: Self = Self(0x5C);
    pub const VK_APPS: Self = Self(0x5D);
    pub const VK_SLEEP: Self = Self(0x5F);
    pub const VK_NUMPAD0: Self = Self(0x60);
    pub const VK_NUMPAD1: Self = Self(0x61);
    pub const VK_NUMPAD2: Self = Self(0x62);
    pub const VK_NUMPAD3: Self = Self(0x63);
    pub const VK_NUMPAD4: Self = Self(0x64);
    pub const VK_NUMPAD5: Self = Self(0x65);
    pub const VK_NUMPAD6: Self = Self(0x66);
    pub const VK_NUMPAD7: Self = Self(0x67);
    pub const VK_NUMPAD8: Self = Self(0x68);
    pub const VK_NUMPAD9: Self = Self(0x69);
    pub const VK_MULTIPLY: Self = Self(0x6A);
    pub const VK_ADD: Self = Self(0x6B);
    pub const VK_SEPARATOR: Self = Self(0x6C);
    pub const VK_SUBTRACT: Self = Self(0x6D);
    pub const VK_DECIMAL: Self = Self(0x6E);
    pub const VK_DIVIDE: Self = Self(0x6F);
    pub const VK_F1: Self = Self(0x70);
    pub const VK_F2: Self = Self(0x71);
    pub const VK_F3: Self = Self(0x72);
    pub const VK_F4: Self = Self(0x73);
    pub const VK_F5: Self = Self(0x74);
    pub const VK_F6: Self = Self(0x75);
    pub const VK_F7: Self = Self(0x76);
    pub const VK_F8: Self = Self(0x77);
    pub const VK_F9: Self = Self(0x78);
    pub const VK_F10: Self = Self(0x79);
    pub const VK_F11: Self = Self(0x7A);
    pub const VK_F12: Self = Self(0x7B);
    pub const VK_F13: Self = Self(0x7C);
    pub const VK_F14: Self = Self(0x7D);
    pub const VK_F15: Self = Self(0x7E);
    pub const VK_F16: Self = Self(0x7F);
    pub const VK_F17: Self = Self(0x80);
    pub const VK_F18: Self = Self(0x81);
    pub const VK_F19: Self = Self(0x82);
    pub const VK_F20: Self = Self(0x83);
    pub const VK_F21: Self = Self(0x84);
    pub const VK_F22: Self = Self(0x85);
    pub const VK_F23: Self = Self(0x86);
    pub const VK_F24: Self = Self(0x87);
    pub const VK_NUMLOCK: Self = Self(0x90);
    pub const VK_SCROLL: Self = Self(0x91);
    pub const VK_LSHIFT: Self = Self(0xA0);
    pub const VK_RSHIFT: Self = Self(0xA1);
    pub const VK_LCONTROL: Self = Self(0xA2);
    pub const VK_RCONTROL: Self = Self(0xA3);
    pub const VK_LMENU: Self = Self(0xA4);
    pub const VK_RMENU: Self = Self(0xA5);
    pub const VK_BROWSER_BACK: Self = Self(0xA6);
    pub const VK_BROWSER_FORWARD: Self = Self(0xA7);
    pub const VK_BROWSER_REFRESH: Self = Self(0xA8);
    pub const VK_BROWSER_STOP: Self = Self(0xA9);
    pub const VK_BROWSER_SEARCH: Self = Self(0xAA);
    pub const VK_BROWSER_FAVORITES: Self = Self(0xAB);
    pub const VK_BROWSER_HOME: Self = Self(0xAC);
    pub const VK_VOLUME_MUTE: Self = Self(0xAD);
    pub const VK_VOLUME_DOWN: Self = Self(0xAE);
    pub const VK_VOLUME_UP: Self = Self(0xAF);
    pub const VK_MEDIA_NEXT_TRACK: Self = Self(0xB0);
    pub const VK_MEDIA_PREV_TRACK: Self = Self(0xB1);
    pub const VK_MEDIA_STOP: Self = Self(0xB2);
    pub const VK_MEDIA_PLAY_PAUSE: Self = Self(0xB3);
    pub const VK_LAUNCH_MAIL: Self = Self(0xB4);
    pub const VK_LAUNCH_MEDIA_SELECT: Self = Self(0xB5);
    pub const VK_LAUNCH_APP1: Self = Self(0xB6);
    pub const VK_LAUNCH_APP2: Self = Self(0xB7);
    pub const VK_OEM_1: Self = Self(0xBA);
    pub const VK_OEM_PLUS: Self = Self(0xBB);
    pub const VK_OEM_COMMA: Self = Self(0xBC);
    pub const VK_OEM_MINUS: Self = Self(0xBD);
    pub const VK_OEM_PERIOD: Self = Self(0xBE);
    pub const VK_OEM_2: Self = Self(0xBF);
    pub const VK_OEM_3: Self = Self(0xC0);
    pub const VK_OEM_4: Self = Self(0xDB);
    pub const VK_OEM_5: Self = Self(0xDC);
    pub const VK_OEM_6: Self = Self(0xDD);
    pub const VK_OEM_7: Self = Self(0xDE);
    pub const VK_OEM_8: Self = Self(0xDF);
    pub const VK_OEM_102: Self = Self(0xE2);
    pub const VK_PROCESSKEY: Self = Self(0xE5);
    pub const VK_PACKET: Self = Self(0xE7);
    pub const VK_ATTN: Self = Self(0xF6);
    pub const VK_CRSEL: Self = Self(0xF7);
    pub const VK_EXSEL: Self = Self(0xF8);
    pub const VK_EREOF: Self = Self(0xF9);
    pub const VK_PLAY: Self = Self(0xFA);
    pub const VK_ZOOM: Self = Self(0xFB);
    pub const VK_PA1: Self = Self(0xFD);
    pub const VK_OEM_CLEAR: Self = Self(0xFE);
}