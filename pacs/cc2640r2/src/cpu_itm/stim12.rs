#[doc = "Register `STIM12` reader"]
pub struct R(crate::R<STIM12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIM12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIM12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIM12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIM12` writer"]
pub struct W(crate::W<STIM12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIM12_SPEC>;
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
impl From<crate::W<STIM12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIM12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIM12` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA12 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STIM12` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA12 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STIM12_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA12 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim12(&self) -> STIM12_R {
        STIM12_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA12 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim12(&mut self) -> STIM12_W<0> {
        STIM12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stimulus Port 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim12](index.html) module"]
pub struct STIM12_SPEC;
impl crate::RegisterSpec for STIM12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stim12::R](R) reader structure"]
impl crate::Readable for STIM12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stim12::W](W) writer structure"]
impl crate::Writable for STIM12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIM12 to value 0"]
impl crate::Resettable for STIM12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
