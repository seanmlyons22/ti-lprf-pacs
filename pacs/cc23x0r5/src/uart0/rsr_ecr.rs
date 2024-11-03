#[doc = "Register `RSR_ECR` reader"]
pub type R = crate::R<RsrEcrSpec>;
#[doc = "Register `RSR_ECR` writer"]
pub type W = crate::W<RsrEcrSpec>;
#[doc = "0:0\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "1: Clears error flag if error is set. Write value is not important."]
    ClearError1 = 1,
    #[doc = "0: Error flag is not set"]
    ErrorNotset = 0,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - 0:0\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1)."]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            true => Fe::ClearError1,
            false => Fe::ErrorNotset,
        }
    }
    #[doc = "Clears error flag if error is set. Write value is not important."]
    #[inline(always)]
    pub fn is_clear_error_1(&self) -> bool {
        *self == Fe::ClearError1
    }
    #[doc = "Error flag is not set"]
    #[inline(always)]
    pub fn is_error_notset(&self) -> bool {
        *self == Fe::ErrorNotset
    }
}
#[doc = "Field `FE` writer - 0:0\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1)."]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG, Fe>;
impl<'a, REG> FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears error flag if error is set. Write value is not important."]
    #[inline(always)]
    pub fn clear_error_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::ClearError1)
    }
    #[doc = "Error flag is not set"]
    #[inline(always)]
    pub fn error_notset(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::ErrorNotset)
    }
}
#[doc = "1:1\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "1: Clears error flag if error is set. Write value is not important."]
    ClearError1 = 1,
    #[doc = "0: Error flag is not set"]
    ErrorNotset = 0,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - 1:1\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            true => Pe::ClearError1,
            false => Pe::ErrorNotset,
        }
    }
    #[doc = "Clears error flag if error is set. Write value is not important."]
    #[inline(always)]
    pub fn is_clear_error_1(&self) -> bool {
        *self == Pe::ClearError1
    }
    #[doc = "Error flag is not set"]
    #[inline(always)]
    pub fn is_error_notset(&self) -> bool {
        *self == Pe::ErrorNotset
    }
}
#[doc = "Field `PE` writer - 1:1\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears error flag if error is set. Write value is not important."]
    #[inline(always)]
    pub fn clear_error_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::ClearError1)
    }
    #[doc = "Error flag is not set"]
    #[inline(always)]
    pub fn error_notset(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::ErrorNotset)
    }
}
#[doc = "2:2\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be {
    #[doc = "1: Clears error flag if error is set. Write value is not important."]
    ClearError1 = 1,
    #[doc = "0: Error flag is not set"]
    ErrorNotset = 0,
}
impl From<Be> for bool {
    #[inline(always)]
    fn from(variant: Be) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BE` reader - 2:2\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received."]
pub type BeR = crate::BitReader<Be>;
impl BeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Be {
        match self.bits {
            true => Be::ClearError1,
            false => Be::ErrorNotset,
        }
    }
    #[doc = "Clears error flag if error is set. Write value is not important."]
    #[inline(always)]
    pub fn is_clear_error_1(&self) -> bool {
        *self == Be::ClearError1
    }
    #[doc = "Error flag is not set"]
    #[inline(always)]
    pub fn is_error_notset(&self) -> bool {
        *self == Be::ErrorNotset
    }
}
#[doc = "Field `BE` writer - 2:2\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received."]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG, Be>;
impl<'a, REG> BeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears error flag if error is set. Write value is not important."]
    #[inline(always)]
    pub fn clear_error_1(self) -> &'a mut crate::W<REG> {
        self.variant(Be::ClearError1)
    }
    #[doc = "Error flag is not set"]
    #[inline(always)]
    pub fn error_notset(self) -> &'a mut crate::W<REG> {
        self.variant(Be::ErrorNotset)
    }
}
#[doc = "3:3\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oe {
    #[doc = "1: Clears error flag if error is set. Write value is not important."]
    ClearError1 = 1,
    #[doc = "0: Error flag is not set"]
    ErrorNotset = 0,
}
impl From<Oe> for bool {
    #[inline(always)]
    fn from(variant: Oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - 3:3\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
pub type OeR = crate::BitReader<Oe>;
impl OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oe {
        match self.bits {
            true => Oe::ClearError1,
            false => Oe::ErrorNotset,
        }
    }
    #[doc = "Clears error flag if error is set. Write value is not important."]
    #[inline(always)]
    pub fn is_clear_error_1(&self) -> bool {
        *self == Oe::ClearError1
    }
    #[doc = "Error flag is not set"]
    #[inline(always)]
    pub fn is_error_notset(&self) -> bool {
        *self == Oe::ErrorNotset
    }
}
#[doc = "Field `OE` writer - 3:3\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG, Oe>;
impl<'a, REG> OeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears error flag if error is set. Write value is not important."]
    #[inline(always)]
    pub fn clear_error_1(self) -> &'a mut crate::W<REG> {
        self.variant(Oe::ClearError1)
    }
    #[doc = "Error flag is not set"]
    #[inline(always)]
    pub fn error_notset(self) -> &'a mut crate::W<REG> {
        self.variant(Oe::ErrorNotset)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1)."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1)."]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<RsrEcrSpec> {
        FeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<RsrEcrSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<RsrEcrSpec> {
        BeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<RsrEcrSpec> {
        OeW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<RsrEcrSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR_ECR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR_ECR. The status information for overrun is set immediately when an overrun condition occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr_ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr_ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsrEcrSpec;
impl crate::RegisterSpec for RsrEcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr_ecr::R`](R) reader structure"]
impl crate::Readable for RsrEcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rsr_ecr::W`](W) writer structure"]
impl crate::Writable for RsrEcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSR_ECR to value 0"]
impl crate::Resettable for RsrEcrSpec {
    const RESET_VALUE: u32 = 0;
}
