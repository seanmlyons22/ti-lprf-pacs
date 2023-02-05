#[doc = "Register `DMAPROTCTL` reader"]
pub struct R(crate::R<DMAPROTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAPROTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAPROTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAPROTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAPROTCTL` writer"]
pub struct W(crate::W<DMAPROTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAPROTCTL_SPEC>;
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
impl From<crate::W<DMAPROTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAPROTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAPROTCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAPROTCTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
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
Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
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
#[doc = "Master Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaprotctl](index.html) module"]
pub struct DMAPROTCTL_SPEC;
impl crate::RegisterSpec for DMAPROTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaprotctl::R](R) reader structure"]
impl crate::Readable for DMAPROTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaprotctl::W](W) writer structure"]
impl crate::Writable for DMAPROTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAPROTCTL to value 0"]
impl crate::Resettable for DMAPROTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
