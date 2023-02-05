#[doc = "Register `EVTOAONFLAGS` reader"]
pub struct R(crate::R<EVTOAONFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOAONFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOAONFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOAONFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOAONFLAGS` writer"]
pub struct W(crate::W<EVTOAONFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOAONFLAGS_SPEC>;
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
impl From<crate::W<EVTOAONFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOAONFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWEV0` reader - 0:0\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV0."]
pub type SWEV0_R = crate::BitReader<bool>;
#[doc = "Field `SWEV0` writer - 0:0\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV0."]
pub type SWEV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `SWEV1` reader - 1:1\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV1."]
pub type SWEV1_R = crate::BitReader<bool>;
#[doc = "Field `SWEV1` writer - 1:1\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV1."]
pub type SWEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `SWEV2` reader - 2:2\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV2."]
pub type SWEV2_R = crate::BitReader<bool>;
#[doc = "Field `SWEV2` writer - 2:2\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV2."]
pub type SWEV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `AUX_COMPA` reader - 3:3\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
pub type AUX_COMPA_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPA` writer - 3:3\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
pub type AUX_COMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `AUX_COMPB` reader - 4:4\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
pub type AUX_COMPB_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPB` writer - 4:4\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
pub type AUX_COMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `ADC_DONE` reader - 5:5\\]
This event flag is set when level selected by EVTOAONPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
pub type ADC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_DONE` writer - 5:5\\]
This event flag is set when level selected by EVTOAONPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
pub type ADC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `TDC_DONE` reader - 6:6\\]
This event flag is set when level selected by EVTOAONPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
pub type TDC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TDC_DONE` writer - 6:6\\]
This event flag is set when level selected by EVTOAONPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
pub type TDC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `TIMER0_EV` reader - 7:7\\]
This event flag is set when level selected by EVTOAONPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
pub type TIMER0_EV_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_EV` writer - 7:7\\]
This event flag is set when level selected by EVTOAONPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
pub type TIMER0_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `TIMER1_EV` reader - 8:8\\]
This event flag is set when level selected by EVTOAONPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
pub type TIMER1_EV_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_EV` writer - 8:8\\]
This event flag is set when level selected by EVTOAONPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
pub type TIMER1_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOAONFLAGS_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVTOAONFLAGS_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV0."]
    #[inline(always)]
    pub fn swev0(&self) -> SWEV0_R {
        SWEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV1."]
    #[inline(always)]
    pub fn swev1(&self) -> SWEV1_R {
        SWEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV2."]
    #[inline(always)]
    pub fn swev2(&self) -> SWEV2_R {
        SWEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This event flag is set when level selected by EVTOAONPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This event flag is set when level selected by EVTOAONPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This event flag is set when level selected by EVTOAONPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This event flag is set when level selected by EVTOAONPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV0."]
    #[inline(always)]
    #[must_use]
    pub fn swev0(&mut self) -> SWEV0_W<0> {
        SWEV0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV1."]
    #[inline(always)]
    #[must_use]
    pub fn swev1(&mut self) -> SWEV1_W<1> {
        SWEV1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
This event flag is set when software writes a 1 to SWEVSET.SWEV2."]
    #[inline(always)]
    #[must_use]
    pub fn swev2(&mut self) -> SWEV2_W<2> {
        SWEV2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W<3> {
        AUX_COMPA_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
This event flag is set when edge selected by EVTOAONPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W<4> {
        AUX_COMPB_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
This event flag is set when level selected by EVTOAONPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> ADC_DONE_W<5> {
        ADC_DONE_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
This event flag is set when level selected by EVTOAONPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TDC_DONE_W<6> {
        TDC_DONE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
This event flag is set when level selected by EVTOAONPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W<7> {
        TIMER0_EV_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
This event flag is set when level selected by EVTOAONPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W<8> {
        TIMER1_EV_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonflags](index.html) module"]
pub struct EVTOAONFLAGS_SPEC;
impl crate::RegisterSpec for EVTOAONFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtoaonflags::R](R) reader structure"]
impl crate::Readable for EVTOAONFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtoaonflags::W](W) writer structure"]
impl crate::Writable for EVTOAONFLAGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTOAONFLAGS to value 0"]
impl crate::Resettable for EVTOAONFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
