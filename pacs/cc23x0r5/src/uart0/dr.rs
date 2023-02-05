#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DATA` reader - 7:0\\]
Data transmitted or received: On writes, the transmit data character is pushed into the FIFO. On reads, the oldest received data character since the last read is returned."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - 7:0\\]
Data transmitted or received: On writes, the transmit data character is pushed into the FIFO. On reads, the oldest received data character since the last read is returned."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FE` reader - 8:8\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO (i.e., the oldest received data character since last read)."]
pub type FeR = crate::BitReader;
#[doc = "Field `PE` reader - 9:9\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select. In FIFO mode, this error is associated with the character at the top of the FIFO (i.e., the oldest received data character since last read)."]
pub type PeR = crate::BitReader;
#[doc = "Field `BE` reader - 10:10\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO (i.e., the oldest received data character since last read). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received."]
pub type BeR = crate::BitReader;
#[doc = "Field `OE` reader - 11:11\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
pub type OeR = crate::BitReader;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data transmitted or received: On writes, the transmit data character is pushed into the FIFO. On reads, the oldest received data character since the last read is returned."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO (i.e., the oldest received data character since last read)."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select. In FIFO mode, this error is associated with the character at the top of the FIFO (i.e., the oldest received data character since last read)."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO (i.e., the oldest received data character since last read). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data transmitted or received: On writes, the transmit data character is pushed into the FIFO. On reads, the oldest received data character since the last read is returned."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
