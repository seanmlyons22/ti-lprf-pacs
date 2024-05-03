#[doc = "Register `STIM3` reader"]
pub type R = crate::R<Stim3Spec>;
#[doc = "Register `STIM3` writer"]
pub type W = crate::W<Stim3Spec>;
#[doc = "Field `STIM3` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim3R = crate::FieldReader<u32>;
#[doc = "Field `STIM3` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim3(&self) -> Stim3R {
        Stim3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim3(&mut self) -> Stim3W<Stim3Spec> {
        Stim3W::new(self, 0)
    }
}
#[doc = "Stimulus Port 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim3Spec;
impl crate::RegisterSpec for Stim3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim3::R`](R) reader structure"]
impl crate::Readable for Stim3Spec {}
#[doc = "`write(|w| ..)` method takes [`stim3::W`](W) writer structure"]
impl crate::Writable for Stim3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM3 to value 0"]
impl crate::Resettable for Stim3Spec {
    const RESET_VALUE: u32 = 0;
}
