#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UARTEN` reader - 0:0\\]
UART Enable"]
pub type UARTEN_R = crate::BitReader<UARTEN_A>;
#[doc = "0:0\\]
UART Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UARTEN_A {
    #[doc = "1: UART enabled"]
    EN = 1,
    #[doc = "0: UART disabled"]
    DIS = 0,
}
impl From<UARTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UARTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UARTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTEN_A {
        match self.bits {
            true => UARTEN_A::EN,
            false => UARTEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UARTEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UARTEN_A::DIS
    }
}
#[doc = "Field `UARTEN` writer - 0:0\\]
UART Enable"]
pub type UARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, UARTEN_A, O>;
impl<'a, const O: u8> UARTEN_W<'a, O> {
    #[doc = "UART enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UARTEN_A::EN)
    }
    #[doc = "UART disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UARTEN_A::DIS)
    }
}
#[doc = "Field `RESERVED1` reader - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `LBE` reader - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
pub type LBE_R = crate::BitReader<LBE_A>;
#[doc = "7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBE_A {
    #[doc = "1: Loop Back enabled"]
    EN = 1,
    #[doc = "0: Loop Back disabled"]
    DIS = 0,
}
impl From<LBE_A> for bool {
    #[inline(always)]
    fn from(variant: LBE_A) -> Self {
        variant as u8 != 0
    }
}
impl LBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBE_A {
        match self.bits {
            true => LBE_A::EN,
            false => LBE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == LBE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LBE_A::DIS
    }
}
#[doc = "Field `LBE` writer - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
pub type LBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, LBE_A, O>;
impl<'a, const O: u8> LBE_W<'a, O> {
    #[doc = "Loop Back enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(LBE_A::EN)
    }
    #[doc = "Loop Back disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LBE_A::DIS)
    }
}
#[doc = "Field `TXE` reader - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
pub type TXE_R = crate::BitReader<TXE_A>;
#[doc = "8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE_A {
    #[doc = "1: UART Transmit enabled"]
    EN = 1,
    #[doc = "0: UART Transmit disabled"]
    DIS = 0,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            true => TXE_A::EN,
            false => TXE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TXE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TXE_A::DIS
    }
}
#[doc = "Field `TXE` writer - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, TXE_A, O>;
impl<'a, const O: u8> TXE_W<'a, O> {
    #[doc = "UART Transmit enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TXE_A::EN)
    }
    #[doc = "UART Transmit disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TXE_A::DIS)
    }
}
#[doc = "Field `RXE` reader - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
pub type RXE_R = crate::BitReader<RXE_A>;
#[doc = "9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXE_A {
    #[doc = "1: UART Receive enabled"]
    EN = 1,
    #[doc = "0: UART Receive disabled"]
    DIS = 0,
}
impl From<RXE_A> for bool {
    #[inline(always)]
    fn from(variant: RXE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXE_A {
        match self.bits {
            true => RXE_A::EN,
            false => RXE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RXE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RXE_A::DIS
    }
}
#[doc = "Field `RXE` writer - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
pub type RXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, RXE_A, O>;
impl<'a, const O: u8> RXE_W<'a, O> {
    #[doc = "UART Receive enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RXE_A::EN)
    }
    #[doc = "UART Receive disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RXE_A::DIS)
    }
}
#[doc = "Field `RESERVED10` reader - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED10` writer - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RTS` reader - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
pub type RTS_R = crate::BitReader<bool>;
#[doc = "Field `RTS` writer - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
pub type RTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTSEN` reader - 14:14\\]
RTS hardware flow control enable"]
pub type RTSEN_R = crate::BitReader<RTSEN_A>;
#[doc = "14:14\\]
RTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSEN_A {
    #[doc = "1: RTS hardware flow control enabled"]
    EN = 1,
    #[doc = "0: RTS hardware flow control disabled"]
    DIS = 0,
}
impl From<RTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSEN_A {
        match self.bits {
            true => RTSEN_A::EN,
            false => RTSEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RTSEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RTSEN_A::DIS
    }
}
#[doc = "Field `RTSEN` writer - 14:14\\]
RTS hardware flow control enable"]
pub type RTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, RTSEN_A, O>;
impl<'a, const O: u8> RTSEN_W<'a, O> {
    #[doc = "RTS hardware flow control enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RTSEN_A::EN)
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RTSEN_A::DIS)
    }
}
#[doc = "Field `CTSEN` reader - 15:15\\]
CTS hardware flow control enable"]
pub type CTSEN_R = crate::BitReader<CTSEN_A>;
#[doc = "15:15\\]
CTS hardware flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSEN_A {
    #[doc = "1: CTS hardware flow control enabled"]
    EN = 1,
    #[doc = "0: CTS hardware flow control disabled"]
    DIS = 0,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            true => CTSEN_A::EN,
            false => CTSEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CTSEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CTSEN_A::DIS
    }
}
#[doc = "Field `CTSEN` writer - 15:15\\]
CTS hardware flow control enable"]
pub type CTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, CTSEN_A, O>;
impl<'a, const O: u8> CTSEN_W<'a, O> {
    #[doc = "CTS hardware flow control enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CTSEN_A::EN)
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CTSEN_A::DIS)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
UART Enable"]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
    #[inline(always)]
    pub fn lbe(&self) -> LBE_R {
        LBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
RTS hardware flow control enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
CTS hardware flow control enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
UART Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uarten(&mut self) -> UARTEN_W<0> {
        UARTEN_W::new(self)
    }
    #[doc = "Bits 1:6 - 6:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Loop Back Enable: Enabling the loop-back mode connects the UARTTXD output from the UART to UARTRXD input of the UART."]
    #[inline(always)]
    #[must_use]
    pub fn lbe(&mut self) -> LBE_W<7> {
        LBE_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
UART Transmit Enable If the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<8> {
        TXE_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
UART Receive Enable If the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<9> {
        RXE_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Request to Send This bit is the complement of the active-low UART RTS output. That is, when the bit is programmed to a 1 then RTS output on the pins is LOW."]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<11> {
        RTS_W::new(self)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
RTS hardware flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<14> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
CTS hardware flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<15> {
        CTSEN_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0300"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
