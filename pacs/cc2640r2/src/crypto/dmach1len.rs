#[doc = "Register `DMACH1LEN` reader"]
pub struct R(crate::R<DMACH1LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACH1LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACH1LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACH1LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACH1LEN` writer"]
pub struct W(crate::W<DMACH1LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACH1LEN_SPEC>;
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
impl From<crate::W<DMACH1LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACH1LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - 15:0\\]
DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
pub type LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LEN` writer - 15:0\\]
DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACH1LEN_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACH1LEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
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
#[doc = "DMA Channel 1 Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1len](index.html) module"]
pub struct DMACH1LEN_SPEC;
impl crate::RegisterSpec for DMACH1LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmach1len::R](R) reader structure"]
impl crate::Readable for DMACH1LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmach1len::W](W) writer structure"]
impl crate::Writable for DMACH1LEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACH1LEN to value 0"]
impl crate::Resettable for DMACH1LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
