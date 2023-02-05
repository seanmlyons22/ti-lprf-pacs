#[doc = "Register `SWWUTRIG` reader"]
pub struct R(crate::R<SWWUTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWWUTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWWUTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWWUTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWWUTRIG` writer"]
pub struct W(crate::W<SWWUTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWWUTRIG_SPEC>;
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
impl From<crate::W<SWWUTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWWUTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_WU0` reader - 0:0\\]
Software wakeup 0 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU0 and trigger AUX wakeup."]
pub type SW_WU0_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU0` writer - 0:0\\]
Software wakeup 0 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU0 and trigger AUX wakeup."]
pub type SW_WU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWWUTRIG_SPEC, bool, O>;
#[doc = "Field `SW_WU1` reader - 1:1\\]
Software wakeup 1 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU1 and trigger AUX wakeup."]
pub type SW_WU1_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU1` writer - 1:1\\]
Software wakeup 1 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU1 and trigger AUX wakeup."]
pub type SW_WU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWWUTRIG_SPEC, bool, O>;
#[doc = "Field `SW_WU2` reader - 2:2\\]
Software wakeup 2 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU2 and trigger AUX wakeup."]
pub type SW_WU2_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU2` writer - 2:2\\]
Software wakeup 2 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU2 and trigger AUX wakeup."]
pub type SW_WU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWWUTRIG_SPEC, bool, O>;
#[doc = "Field `SW_WU3` reader - 3:3\\]
Software wakeup 3 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU3 and trigger AUX wakeup."]
pub type SW_WU3_R = crate::BitReader<bool>;
#[doc = "Field `SW_WU3` writer - 3:3\\]
Software wakeup 3 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU3 and trigger AUX wakeup."]
pub type SW_WU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWWUTRIG_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWWUTRIG_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software wakeup 0 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU0 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu0(&self) -> SW_WU0_R {
        SW_WU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software wakeup 1 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU1 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu1(&self) -> SW_WU1_R {
        SW_WU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software wakeup 2 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU2 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu2(&self) -> SW_WU2_R {
        SW_WU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software wakeup 3 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU3 and trigger AUX wakeup."]
    #[inline(always)]
    pub fn sw_wu3(&self) -> SW_WU3_R {
        SW_WU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software wakeup 0 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU0 and trigger AUX wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu0(&mut self) -> SW_WU0_W<0> {
        SW_WU0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Software wakeup 1 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU1 and trigger AUX wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu1(&mut self) -> SW_WU1_W<1> {
        SW_WU1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Software wakeup 2 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU2 and trigger AUX wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu2(&mut self) -> SW_WU2_W<2> {
        SW_WU2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Software wakeup 3 trigger. 0: No effect. 1: Set WUFLAGS.SW_WU3 and trigger AUX wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn sw_wu3(&mut self) -> SW_WU3_W<3> {
        SW_WU3_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swwutrig](index.html) module"]
pub struct SWWUTRIG_SPEC;
impl crate::RegisterSpec for SWWUTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swwutrig::R](R) reader structure"]
impl crate::Readable for SWWUTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swwutrig::W](W) writer structure"]
impl crate::Writable for SWWUTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWWUTRIG to value 0"]
impl crate::Resettable for SWWUTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
