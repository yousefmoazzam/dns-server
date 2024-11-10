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

    pub fn step(&mut self, step: usize) {
        self.pos += step;
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
        packet_buffer.step(step);
        assert_eq!(step, packet_buffer.pos());
    }
}
