#[doc = "Register `INFRCLKDIVDS` reader"]
pub struct R(crate::R<INFRCLKDIVDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFRCLKDIVDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFRCLKDIVDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFRCLKDIVDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INFRCLKDIVDS` writer"]
pub struct W(crate::W<INFRCLKDIVDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INFRCLKDIVDS_SPEC>;
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
impl From<crate::W<INFRCLKDIVDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INFRCLKDIVDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RATIO` reader - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in deepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
pub type RATIO_R = crate::FieldReader<u8, RATIO_A>;
#[doc = "1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in deepsleep mode. Division ratio affects both infrastructure clock and perbusull clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RATIO_A {
    #[doc = "3: Divide by 32"]
    DIV32 = 3,
    #[doc = "2: Divide by 8"]
    DIV8 = 2,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "0: Divide by 1"]
    DIV1 = 0,
}
impl From<RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as _
    }
}
impl RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            3 => RATIO_A::DIV32,
            2 => RATIO_A::DIV8,
            1 => RATIO_A::DIV2,
            0 => RATIO_A::DIV1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == RATIO_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RATIO_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RATIO_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RATIO_A::DIV1
    }
}
#[doc = "Field `RATIO` writer - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in deepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
pub type RATIO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, INFRCLKDIVDS_SPEC, u8, RATIO_A, 2, O>;
impl<'a, const O: u8> RATIO_W<'a, O> {
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(RATIO_A::DIV32)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RATIO_A::DIV8)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RATIO_A::DIV2)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RATIO_A::DIV1)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INFRCLKDIVDS_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in deepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in deepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RATIO_W<0> {
        RATIO_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infrclkdivds](index.html) module"]
pub struct INFRCLKDIVDS_SPEC;
impl crate::RegisterSpec for INFRCLKDIVDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [infrclkdivds::R](R) reader structure"]
impl crate::Readable for INFRCLKDIVDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [infrclkdivds::W](W) writer structure"]
impl crate::Writable for INFRCLKDIVDS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INFRCLKDIVDS to value 0"]
impl crate::Resettable for INFRCLKDIVDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
