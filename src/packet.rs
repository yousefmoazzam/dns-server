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

    pub fn read(&mut self) -> Result<u8, String> {
        if self.pos >= PACKET_BYTES_LENGTH {
            let err_str = format!(
                "Invalid read, reading past buffer boundary: buffer length={}, pos={}",
                PACKET_BYTES_LENGTH, self.pos
            );
            return Err(err_str);
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        Ok(((self.read()? as u16) << 8) | (self.read()? as u16))
    }

    pub fn get(&self) -> Result<u8, String> {
        if self.pos >= PACKET_BYTES_LENGTH {
            let err_str = format!(
                "Invalid get, getting value past buffer boundary: buffer length={}, pos={}",
                PACKET_BYTES_LENGTH, self.pos
            );
            return Err(err_str);
        }

        Ok(self.buf[self.pos])
    }

    pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8], String> {
        if start + len >= PACKET_BYTES_LENGTH {
            let err_str = format!(
                "Invalid range, getting range past buffer boundary: buffer length={}, start={}, len={}",
                PACKET_BYTES_LENGTH, start, len
            );
            return Err(err_str);
        }

        Ok(&self.buf[start..start + len])
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
        assert_eq!(
            true,
            packet_buffer.read().is_ok_and(|val| val == zeroth_element)
        );
        assert_eq!(1, packet_buffer.pos());
    }

    #[test]
    fn return_error_if_reading_at_index_past_end_of_buffer() {
        let buf = [0; PACKET_BYTES_LENGTH];
        let mut packet_buffer = PacketBuffer::new(buf);
        _ = packet_buffer.seek(PACKET_BYTES_LENGTH - 1); // seek to last byte - valid
        _ = packet_buffer.read(); // read last byte + step forward - valid
        let res = packet_buffer.read(); // try to read past end of buffer - invalid
        let expected_str = "Invalid read, reading past buffer boundary: buffer length=512, pos=512";
        assert_eq!(true, res.is_err_and(|err_str| err_str == expected_str));
    }

    #[test]
    fn get_correct_value_and_pos_not_moved_forward() {
        let mut buf = [0; PACKET_BYTES_LENGTH];
        let zeroth_element = 1;
        buf[0] = zeroth_element;
        let packet_buffer = PacketBuffer::new(buf);
        assert_eq!(
            true,
            packet_buffer.get().is_ok_and(|val| val == zeroth_element)
        );
        assert_eq!(0, packet_buffer.pos());
    }

    #[test]
    fn return_error_if_getting_value_at_index_past_end_of_buffer() {
        let buf = [0; PACKET_BYTES_LENGTH];
        let mut packet_buffer = PacketBuffer::new(buf);
        _ = packet_buffer.seek(PACKET_BYTES_LENGTH - 1); // seek to last byte - valid
        _ = packet_buffer.read(); // read last byte + step forward - valid
        let res = packet_buffer.get(); // try to get value past end of buffer - invalid
        let expected_str =
            "Invalid get, getting value past buffer boundary: buffer length=512, pos=512";
        assert_eq!(true, res.is_err_and(|err_str| err_str == expected_str));
    }

    #[test]
    fn get_correct_range_within_buffer() {
        let buf: [u8; PACKET_BYTES_LENGTH] = core::array::from_fn(|idx| idx as u8);
        let start = 0;
        let len = 10;
        let packet_buffer = PacketBuffer::new(buf);
        let expected_slice = &buf[start..start + len];
        assert_eq!(
            true,
            packet_buffer
                .get_range(start, len)
                .is_ok_and(|val| val == expected_slice)
        );
    }

    #[test]
    fn return_error_if_getting_range_past_end_of_buffer() {
        let buf = [0; PACKET_BYTES_LENGTH];
        let start = 500;
        let len = 20;
        let packet_buffer = PacketBuffer::new(buf);
        let res = packet_buffer.get_range(start, len);
        let expected_str = "Invalid range, getting range past buffer boundary: buffer length=512, start=500, len=20";
        assert_eq!(true, res.is_err_and(|err_str| err_str == expected_str));
    }

    #[test]
    fn get_correct_value_from_u16_read() {
        let mut buf = [0; PACKET_BYTES_LENGTH];
        let lo = 0x05;
        let hi = 0x03;
        let expected_u16_value = ((hi as u16) << 8) | (lo as u16);
        buf[0] = hi;
        buf[1] = lo;
        let mut packet_buffer = PacketBuffer::new(buf);
        assert_eq!(
            true,
            packet_buffer
                .read_u16()
                .is_ok_and(|val| val == expected_u16_value)
        );
        assert_eq!(2, packet_buffer.pos());
    }
}
