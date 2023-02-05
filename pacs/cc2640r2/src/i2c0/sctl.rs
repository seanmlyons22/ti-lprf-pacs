#[doc = "Register `SCTL` reader"]
pub struct R(crate::R<SCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTL` writer"]
pub struct W(crate::W<SCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTL_SPEC>;
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
impl From<crate::W<SCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DA` reader - 0:0\\]
Device active 0: Disables the I2C slave operation 1: Enables the I2C slave operation"]
pub type DA_R = crate::BitReader<bool>;
#[doc = "Field `DA` writer - 0:0\\]
Device active 0: Disables the I2C slave operation 1: Enables the I2C slave operation"]
pub type DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Device active 0: Disables the I2C slave operation 1: Enables the I2C slave operation"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Device active 0: Disables the I2C slave operation 1: Enables the I2C slave operation"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<0> {
        DA_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
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
#[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctl](index.html) module"]
pub struct SCTL_SPEC;
impl crate::RegisterSpec for SCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctl::R](R) reader structure"]
impl crate::Readable for SCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctl::W](W) writer structure"]
impl crate::Writable for SCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTL to value 0"]
impl crate::Resettable for SCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
