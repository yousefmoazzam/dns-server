const PACKET_BYTES_LENGTH: usize = 512;

pub struct PacketBuffer {
    buf: [u8; PACKET_BYTES_LENGTH],
    pos: usize,
}

impl PacketBuffer {
    pub fn new(buf: [u8; PACKET_BYTES_LENGTH]) -> PacketBuffer {
        PacketBuffer { buf, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self, step: usize) -> Result<(), String> {
        if self.pos + step >= PACKET_BYTES_LENGTH {
            let err_str = format!(
                "Invalid step, stepping past buffer boundary: buffer length={}, pos={}, step={}",
                PACKET_BYTES_LENGTH, self.pos, step
            );
            return Err(err_str);
        }

        self.pos += step;
        Ok(())
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), String> {
        if pos >= PACKET_BYTES_LENGTH {
            let err_str = format!(
                "Invalid seek, seeking past buffer boundary: buffer length={}, seek={}",
                PACKET_BYTES_LENGTH, pos
            );
            return Err(err_str);
        }

        self.pos = pos;
        Ok(())
    }

    pub fn read(&mut self) -> u8 {
        let res = self.buf[self.pos];
        self.pos += 1;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_position_within_new_packet_buffer_is_zero() {
        let buf = [0; PACKET_BYTES_LENGTH];
        let packet_buffer = PacketBuffer::new(buf);
        assert_eq!(0, packet_buffer.pos());
    }

    #[test]
    fn step_position_forward_in_buffer() {
        let buf = [0; PACKET_BYTES_LENGTH];
        let mut packet_buffer = PacketBuffer::new(buf);
        let step = 5;
        let res = packet_buffer.step(step);
        assert_eq!(true, res.is_ok());
        assert_eq!(step, packet_buffer.pos());
    }

    #[test]
    fn return_error_if_stepping_past_end_of_buffer() {
        let buf = [0; PACKET_BYTES_LENGTH];
        let mut packet_buffer = PacketBuffer::new(buf);
        let invalid_step = PACKET_BYTES_LENGTH + 1;
        let res = packet_buffer.step(invalid_step);
        let expected_err_str =
            "Invalid step, stepping past buffer boundary: buffer length=512, pos=0, step=513";
        assert_eq!(true, res.is_err_and(|err_str| err_str == expected_err_str));
    }

    #[test]
    fn seek_to_position_within_buffer() {
        let buf = [0; PACKET_BYTES_LENGTH];
        let mut packet_buffer = PacketBuffer::new(buf);
        let new_pos = 51;
        let res = packet_buffer.seek(new_pos);
        assert_eq!(true, res.is_ok());
        assert_eq!(new_pos, packet_buffer.pos());
    }

    #[test]
    fn return_error_if_seeking_past_end_of_buffer() {
        let buf = [0; PACKET_BYTES_LENGTH];
        let mut packet_buffer = PacketBuffer::new(buf);
        let invalid_pos = 600;
        let res = packet_buffer.seek(invalid_pos);
        let expected_str =
            "Invalid seek, seeking past buffer boundary: buffer length=512, seek=600";
        assert_eq!(true, res.is_err_and(|err_str| err_str == expected_str));
    }

    #[test]
    fn correct_value_read_at_pos_zero_and_pos_moved_up_by_one() {
        let mut buf = [0; PACKET_BYTES_LENGTH];
        let zeroth_element = 1;
        buf[0] = zeroth_element;
        let mut packet_buffer = PacketBuffer::new(buf);
        assert_eq!(zeroth_element, packet_buffer.read());
        assert_eq!(1, packet_buffer.pos());
    }
}
