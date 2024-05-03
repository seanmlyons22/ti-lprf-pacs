#[doc = "Register `STIM31` reader"]
pub type R = crate::R<Stim31Spec>;
#[doc = "Register `STIM31` writer"]
pub type W = crate::W<Stim31Spec>;
#[doc = "Field `STIM31` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA31 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim31R = crate::FieldReader<u32>;
#[doc = "Field `STIM31` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA31 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA31 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim31(&self) -> Stim31R {
        Stim31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA31 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim31(&mut self) -> Stim31W<Stim31Spec> {
        Stim31W::new(self, 0)
    }
}
#[doc = "Stimulus Port 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim31Spec;
impl crate::RegisterSpec for Stim31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim31::R`](R) reader structure"]
impl crate::Readable for Stim31Spec {}
#[doc = "`write(|w| ..)` method takes [`stim31::W`](W) writer structure"]
impl crate::Writable for Stim31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM31 to value 0"]
impl crate::Resettable for Stim31Spec {
    const RESET_VALUE: u32 = 0;
}
