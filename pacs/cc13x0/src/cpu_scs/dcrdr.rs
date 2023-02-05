#[doc = "Register `DCRDR` reader"]
pub struct R(crate::R<DCRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCRDR` writer"]
pub struct W(crate::W<DCRDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRDR_SPEC>;
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
impl From<crate::W<DCRDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCRDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCRDR` reader - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
pub type DCRDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DCRDR` writer - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
pub type DCRDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCRDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
    #[inline(always)]
    pub fn dcrdr(&self) -> DCRDR_R {
        DCRDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
    #[inline(always)]
    #[must_use]
    pub fn dcrdr(&mut self) -> DCRDR_W<0> {
        DCRDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Core Register Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrdr](index.html) module"]
pub struct DCRDR_SPEC;
impl crate::RegisterSpec for DCRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcrdr::R](R) reader structure"]
impl crate::Readable for DCRDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcrdr::W](W) writer structure"]
impl crate::Writable for DCRDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCRDR to value 0"]
impl crate::Resettable for DCRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
