#[doc = "Register `STIM5` reader"]
pub struct R(crate::R<STIM5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIM5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIM5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIM5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIM5` writer"]
pub struct W(crate::W<STIM5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIM5_SPEC>;
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
impl From<crate::W<STIM5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIM5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIM5` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA5 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STIM5` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA5 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STIM5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA5 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim5(&self) -> STIM5_R {
        STIM5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA5 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim5(&mut self) -> STIM5_W<0> {
        STIM5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stimulus Port 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim5](index.html) module"]
pub struct STIM5_SPEC;
impl crate::RegisterSpec for STIM5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stim5::R](R) reader structure"]
impl crate::Readable for STIM5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stim5::W](W) writer structure"]
impl crate::Writable for STIM5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIM5 to value 0"]
impl crate::Resettable for STIM5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
