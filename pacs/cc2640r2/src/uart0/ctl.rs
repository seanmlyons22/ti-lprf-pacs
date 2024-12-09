#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "0:0\\]
UART Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uarten {
    #[doc = "1: UART enabled"]
    En = 1,
    #[doc = "0: UART disabled"]
    Dis = 0,
}
impl From<Uarten> for bool {
    #[inline(always)]
    fn from(variant: Uarten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTEN` reader - 0:0\\]
UART Enable"]
pub type UartenR = crate::BitReader<Uarten>;
impl UartenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uarten {
        match self.bits {
            true => Uarten::En,
            false => Uarten::Dis,
        }
    }
    #[doc = "UART enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Uarten::En
    }
    #[doc = "UART disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Uarten::Dis
    }
}
#[doc = "Field `UARTEN` writer - 0:0\\]
UART Enable"]
pub type UartenW<'a, REG> = crate::BitWriter<'a, REG, Uarten>;
impl<'a, REG> UartenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Uarten::En)
    }
    #[doc = "UART disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Uarten::Dis)
    }
}
#[doc = "Field `RESERVED1` reader - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbe {
    #[doc = "1: Loop Back enabled"]
    En = 1,
    #[doc = "0: Loop Back disabled"]
    Dis = 0,
}
impl From<Lbe> for bool {
    #[inline(always)]
    fn from(variant: Lbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBE` reader - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
pub type LbeR = crate::BitReader<Lbe>;
impl LbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbe {
        match self.bits {
            true => Lbe::En,
            false => Lbe::Dis,
        }
    }
    #[doc = "Loop Back enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Lbe::En
    }
    #[doc = "Loop Back disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Lbe::Dis
    }
}
#[doc = "Field `LBE` writer - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
pub type LbeW<'a, REG> = crate::BitWriter<'a, REG, Lbe>;
impl<'a, REG> LbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop Back enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Lbe::En)
    }
    #[doc = "Loop Back disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lbe::Dis)
    }
}
#[doc = "8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txe {
    #[doc = "1: UART Transmit enabled"]
    En = 1,
    #[doc = "0: UART Transmit disabled"]
    Dis = 0,
}
impl From<Txe> for bool {
    #[inline(always)]
    fn from(variant: Txe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
pub type TxeR = crate::BitReader<Txe>;
impl TxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txe {
        match self.bits {
            true => Txe::En,
            false => Txe::Dis,
        }
    }
    #[doc = "UART Transmit enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Txe::En
    }
    #[doc = "UART Transmit disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Txe::Dis
    }
}
#[doc = "Field `TXE` writer - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG, Txe>;
impl<'a, REG> TxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART Transmit enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::En)
    }
    #[doc = "UART Transmit disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::Dis)
    }
}
#[doc = "9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxe {
    #[doc = "1: UART Receive enabled"]
    En = 1,
    #[doc = "0: UART Receive disabled"]
    Dis = 0,
}
impl From<Rxe> for bool {
    #[inline(always)]
    fn from(variant: Rxe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXE` reader - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
pub type RxeR = crate::BitReader<Rxe>;
impl RxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxe {
        match self.bits {
            true => Rxe::En,
            false => Rxe::Dis,
        }
    }
    #[doc = "UART Receive enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rxe::En
    }
    #[doc = "UART Receive disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rxe::Dis
    }
}
#[doc = "Field `RXE` writer - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
pub type RxeW<'a, REG> = crate::BitWriter<'a, REG, Rxe>;
impl<'a, REG> RxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART Receive enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rxe::En)
    }
    #[doc = "UART Receive disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rxe::Dis)
    }
}
#[doc = "Field `RESERVED10` reader - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `RESERVED10` writer - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader;
#[doc = "Field `RESERVED12` writer - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "14:14\\]
RTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsen {
    #[doc = "1: RTS hardware flow control enabled"]
    En = 1,
    #[doc = "0: RTS hardware flow control disabled"]
    Dis = 0,
}
impl From<Rtsen> for bool {
    #[inline(always)]
    fn from(variant: Rtsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` reader - 14:14\\]
RTS hardware flow control enable"]
pub type RtsenR = crate::BitReader<Rtsen>;
impl RtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsen {
        match self.bits {
            true => Rtsen::En,
            false => Rtsen::Dis,
        }
    }
    #[doc = "RTS hardware flow control enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Rtsen::En
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rtsen::Dis
    }
}
#[doc = "Field `RTSEN` writer - 14:14\\]
RTS hardware flow control enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG, Rtsen>;
impl<'a, REG> RtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS hardware flow control enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::En)
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsen::Dis)
    }
}
#[doc = "15:15\\]
CTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "1: CTS hardware flow control enabled"]
    En = 1,
    #[doc = "0: CTS hardware flow control disabled"]
    Dis = 0,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - 15:15\\]
CTS hardware flow control enable"]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            true => Ctsen::En,
            false => Ctsen::Dis,
        }
    }
    #[doc = "CTS hardware flow control enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ctsen::En
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ctsen::Dis
    }
}
#[doc = "Field `CTSEN` writer - 15:15\\]
CTS hardware flow control enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS hardware flow control enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::En)
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Dis)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
UART Enable"]
    #[inline(always)]
    pub fn uarten(&self) -> UartenR {
        UartenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
    #[inline(always)]
    pub fn lbe(&self) -> LbeR {
        LbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn rxe(&self) -> RxeR {
        RxeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
UART Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UartenW<CtlSpec> {
        UartenW::new(self, 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CtlSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
    #[inline(always)]
    #[must_use]
    pub fn lbe(&mut self) -> LbeW<CtlSpec> {
        LbeW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TxeW<CtlSpec> {
        TxeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RxeW<CtlSpec> {
        RxeW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<CtlSpec> {
        Reserved10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RtsW<CtlSpec> {
        RtsW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<CtlSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
RTS hardware flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<CtlSpec> {
        RtsenW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
CTS hardware flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<CtlSpec> {
        CtsenW::new(self, 15)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0300"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0300;
}
