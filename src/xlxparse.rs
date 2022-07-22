// This is a horrible fix to something that shouldn't be an issue!
// XLXD uses illegal spaces in XML tags so we need to remove/replace
// them before we pass off to the XML parser
pub fn parse(buf: &mut [u8]) {
    for i in 0..buf.len() {
        if i == 0 {
            continue;
        }
        if buf[i] == 0x3c {
            if buf.len() - i < 3 {
                continue;
            }
            if (buf[i + 1] == 0x4e && buf[i + 2] == 0x4f)
                || (buf[i + 1] == 0x53 && buf[i + 2] == 0x54)
            {
                continue;
            }

            for s in &mut buf[i..] {
                if *s == 0x3e {
                    break;
                }
                if *s == 0x20 {
                    *s = 0x5f;
                }
            }
        }
    }
}
