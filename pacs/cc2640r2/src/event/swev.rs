#[doc = "Register `SWEV` reader"]
pub struct R(crate::R<SWEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWEV` writer"]
pub struct W(crate::W<SWEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWEV_SPEC>;
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
impl From<crate::W<SWEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWEV0` reader - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
pub type SWEV0_R = crate::BitReader<bool>;
#[doc = "Field `SWEV0` writer - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
pub type SWEV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWEV_SPEC, bool, O>;
#[doc = "Field `RESERVED0` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWEV_SPEC, u8, u8, 7, O>;
#[doc = "Field `SWEV1` reader - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
pub type SWEV1_R = crate::BitReader<bool>;
#[doc = "Field `SWEV1` writer - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
pub type SWEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWEV_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWEV_SPEC, u8, u8, 7, O>;
#[doc = "Field `SWEV2` reader - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
pub type SWEV2_R = crate::BitReader<bool>;
#[doc = "Field `SWEV2` writer - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
pub type SWEV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWEV_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWEV_SPEC, u8, u8, 7, O>;
#[doc = "Field `SWEV3` reader - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
pub type SWEV3_R = crate::BitReader<bool>;
#[doc = "Field `SWEV3` writer - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
pub type SWEV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWEV_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWEV_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
    #[inline(always)]
    pub fn swev0(&self) -> SWEV0_R {
        SWEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
    #[inline(always)]
    pub fn swev1(&self) -> SWEV1_R {
        SWEV1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
    #[inline(always)]
    pub fn swev2(&self) -> SWEV2_R {
        SWEV2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
    #[inline(always)]
    pub fn swev3(&self) -> SWEV3_R {
        SWEV3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
    #[inline(always)]
    #[must_use]
    pub fn swev0(&mut self) -> SWEV0_W<0> {
        SWEV0_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<1> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
    #[inline(always)]
    #[must_use]
    pub fn swev1(&mut self) -> SWEV1_W<8> {
        SWEV1_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<9> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
    #[inline(always)]
    #[must_use]
    pub fn swev2(&mut self) -> SWEV2_W<16> {
        SWEV2_W::new(self)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<17> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
    #[inline(always)]
    #[must_use]
    pub fn swev3(&mut self) -> SWEV3_W<24> {
        SWEV3_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<25> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set or Clear Software Events\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swev](index.html) module"]
pub struct SWEV_SPEC;
impl crate::RegisterSpec for SWEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swev::R](R) reader structure"]
impl crate::Readable for SWEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swev::W](W) writer structure"]
impl crate::Writable for SWEV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEV to value 0"]
impl crate::Resettable for SWEV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
