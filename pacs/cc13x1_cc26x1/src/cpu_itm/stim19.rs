#[doc = "Register `STIM19` reader"]
pub type R = crate::R<Stim19Spec>;
#[doc = "Register `STIM19` writer"]
pub type W = crate::W<Stim19Spec>;
#[doc = "Field `STIM19` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA19 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim19R = crate::FieldReader<u32>;
#[doc = "Field `STIM19` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA19 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim19W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA19 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim19(&self) -> Stim19R {
        Stim19R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA19 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim19(&mut self) -> Stim19W<Stim19Spec> {
        Stim19W::new(self, 0)
    }
}
#[doc = "Stimulus Port 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim19Spec;
impl crate::RegisterSpec for Stim19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim19::R`](R) reader structure"]
impl crate::Readable for Stim19Spec {}
#[doc = "`write(|w| ..)` method takes [`stim19::W`](W) writer structure"]
impl crate::Writable for Stim19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM19 to value 0"]
impl crate::Resettable for Stim19Spec {
    const RESET_VALUE: u32 = 0;
}
