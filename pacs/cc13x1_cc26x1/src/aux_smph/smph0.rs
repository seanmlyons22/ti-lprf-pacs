#[doc = "Register `SMPH0` reader"]
pub struct R(crate::R<SMPH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPH0` writer"]
pub struct W(crate::W<SMPH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPH0_SPEC>;
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
impl From<crate::W<SMPH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - 0:0\\]
Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
pub type STAT_R = crate::BitReader<bool>;
#[doc = "Field `STAT` writer - 0:0\\]
Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
pub type STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMPH0_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPH0_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<0> {
        STAT_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Semaphore 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph0](index.html) module"]
pub struct SMPH0_SPEC;
impl crate::RegisterSpec for SMPH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smph0::R](R) reader structure"]
impl crate::Readable for SMPH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smph0::W](W) writer structure"]
impl crate::Writable for SMPH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPH0 to value 0x01"]
impl crate::Resettable for SMPH0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
