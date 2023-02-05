#[doc = "Register `DMACH0LEN` reader"]
pub struct R(crate::R<DMACH0LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACH0LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACH0LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACH0LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACH0LEN` writer"]
pub struct W(crate::W<DMACH0LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACH0LEN_SPEC>;
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
impl From<crate::W<DMACH0LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACH0LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMALEN` reader - 15:0\\]
Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
pub type DMALEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMALEN` writer - 15:0\\]
Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
pub type DMALEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACH0LEN_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACH0LEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    pub fn dmalen(&self) -> DMALEN_R {
        DMALEN_R::new((self.bits & 0xffff) as u16)
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
Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn dmalen(&mut self) -> DMALEN_W<0> {
        DMALEN_W::new(self)
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
#[doc = "Channel 0 DMA Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0len](index.html) module"]
pub struct DMACH0LEN_SPEC;
impl crate::RegisterSpec for DMACH0LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmach0len::R](R) reader structure"]
impl crate::Readable for DMACH0LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmach0len::W](W) writer structure"]
impl crate::Writable for DMACH0LEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACH0LEN to value 0"]
impl crate::Resettable for DMACH0LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
