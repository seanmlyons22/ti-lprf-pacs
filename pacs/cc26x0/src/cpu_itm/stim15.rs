#[doc = "Register `STIM15` reader"]
pub struct R(crate::R<STIM15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIM15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIM15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIM15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIM15` writer"]
pub struct W(crate::W<STIM15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIM15_SPEC>;
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
impl From<crate::W<STIM15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIM15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIM15` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA15 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM15_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STIM15` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA15 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STIM15_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA15 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim15(&self) -> STIM15_R {
        STIM15_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA15 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim15(&mut self) -> STIM15_W<0> {
        STIM15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stimulus Port 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim15](index.html) module"]
pub struct STIM15_SPEC;
impl crate::RegisterSpec for STIM15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stim15::R](R) reader structure"]
impl crate::Readable for STIM15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stim15::W](W) writer structure"]
impl crate::Writable for STIM15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIM15 to value 0"]
impl crate::Resettable for STIM15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
