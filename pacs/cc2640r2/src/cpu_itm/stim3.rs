#[doc = "Register `STIM3` reader"]
pub struct R(crate::R<STIM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIM3` writer"]
pub struct W(crate::W<STIM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIM3_SPEC>;
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
impl From<crate::W<STIM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIM3` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STIM3` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STIM3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim3(&self) -> STIM3_R {
        STIM3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim3(&mut self) -> STIM3_W<0> {
        STIM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stimulus Port 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim3](index.html) module"]
pub struct STIM3_SPEC;
impl crate::RegisterSpec for STIM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stim3::R](R) reader structure"]
impl crate::Readable for STIM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stim3::W](W) writer structure"]
impl crate::Writable for STIM3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIM3 to value 0"]
impl crate::Resettable for STIM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
