pub mod events;

use std::io;

/// Event commands received from the head-unit are always prefixed with `0xCA` and `0x34`.
static EVENT_PREFIX: [u8; 2] = [0xCA, 0x34];

/// An enumeration of events received from the head-unit.
///
/// Events from the head-unit are usually key presses or head-unit state changes, and are
/// represented by four bytes, two identity bytes (which are always `0xCA` and `0x34`), one command
/// byte (which represents the type of event received), and one parity byte (for validating the
/// command, it is effectively the inverse of the command byte).
///
/// See also:
///   - ["CD Changer Protocol -
///   Martinsuniverse"](https://martinsuniverse.de/projekte/cdc_protokoll/cdc_protokoll.html)
pub enum Event {
    /// A button press of the "scan" button.
    Scan,
    Random1CD,
    Random6CD,
    Radio,
    /// Long-press of the previous-track button to seek backward in the current track.
    Rewind,
    /// Long-press of the next-track button to seek forward in the current track.
    Forward,
    LoadCD,
    /// A button press of the previous-track button to play the previous track.
    PreviousTrack,
    /// A button press of the next-track button to play the next track.
    NextTrack,
    PowerOn,
    PowerOff,
    /// Playback should resume, fast-forward/reverse has ended.
    Resume,
    /// A button press of the disc 1 button.
    Disc1,
    /// A button press of the disc 2 button.
    Disc2,
    /// A button press of the disc 3 button.
    Disc3,
    /// A button press of the disc 4 button.
    Disc4,
    /// A button press of the disc 5 button.
    Disc5,
    /// A button press of the disc 6 button.
    Disc6,
    /// A button press of the disc 7 button (not present in '04 VW Golf).
    Disc7,
    /// A button press of the disc 8 button (not present in '04 VW Golf).
    Disc8,
    /// A button press of the disc 9 button (not present in '04 VW Golf).
    Disc9,
    CDChange,
}

impl Event {
    /// Attempt to deserialize an event from an unsigned 32-bit integer.
    ///
    /// Commands received from the head-unit are always four bytes long. Therefore, we use a packed
    /// 32-bit unsigned integer as a representation.
    ///
    /// The first two bytes will always be `0xCA` and `0x34`, the third byte is the command, and
    /// the final byte will be the inverse of the third byte for parity.
    pub fn deserialize(value: u32) -> Result<Self, io::Error> {
        let [header0, header1, command, _parity] = value.to_ne_bytes();

        // test command header
        if [header0, header1] != EVENT_PREFIX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Invalid command prefix: {:X}:{:X}, expected CA:34.",
                    header0, header1
                ),
            ));
        }

        // FIXME test command integrity via parity bits, upstream example:
        /*
         * uint8_t get_command(uint32_t cmd) {
         *     // test header
         *     if (((cmd>>24) & 0xFF) == CDC_PREFIX1 && ((cmd>>16) & 0xFF) == CDC_PREFIX2) {
         *         // test parity
         *         if (((cmd>>8) & 0xFF) == (0xFF^((cmd) & 0xFF))) {
         *             return (cmd>>8) & 0xFF;
         *         }
         *     }
         *
         *     return 0;
         * }
         */

        match command {
            0x05 => Ok(Event::Scan),
            0x06 => Ok(Event::Random1CD),
            0x07 => Ok(Event::Random6CD),
            0x08 => Ok(Event::Radio),
            _ => unimplemented!(),
        }
    }
}
