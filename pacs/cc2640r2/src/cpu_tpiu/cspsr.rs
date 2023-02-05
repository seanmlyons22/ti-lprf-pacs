#[doc = "Register `CSPSR` reader"]
pub struct R(crate::R<CSPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSPSR` writer"]
pub struct W(crate::W<CSPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSPSR_SPEC>;
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
impl From<crate::W<CSPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ONE` reader - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type ONE_R = crate::BitReader<bool>;
#[doc = "Field `ONE` writer - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type ONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSPSR_SPEC, bool, O>;
#[doc = "Field `TWO` reader - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type TWO_R = crate::BitReader<bool>;
#[doc = "Field `TWO` writer - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type TWO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSPSR_SPEC, bool, O>;
#[doc = "Field `THREE` reader - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type THREE_R = crate::BitReader<bool>;
#[doc = "Field `THREE` writer - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type THREE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSPSR_SPEC, bool, O>;
#[doc = "Field `FOUR` reader - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type FOUR_R = crate::BitReader<bool>;
#[doc = "Field `FOUR` writer - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
pub type FOUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSPSR_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSPSR_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn two(&self) -> TWO_R {
        TWO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn three(&self) -> THREE_R {
        THREE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub fn four(&self) -> FOUR_R {
        FOUR_R::new(((self.bits >> 3) & 1) != 0)
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
1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> ONE_W<0> {
        ONE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn two(&mut self) -> TWO_W<1> {
        TWO_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn three(&mut self) -> THREE_W<2> {
        THREE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    #[must_use]
    pub fn four(&mut self) -> FOUR_W<3> {
        FOUR_W::new(self)
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
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspsr](index.html) module"]
pub struct CSPSR_SPEC;
impl crate::RegisterSpec for CSPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspsr::R](R) reader structure"]
impl crate::Readable for CSPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cspsr::W](W) writer structure"]
impl crate::Writable for CSPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSPSR to value 0x01"]
impl crate::Resettable for CSPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
