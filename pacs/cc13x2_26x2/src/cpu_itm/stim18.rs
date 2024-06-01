#[doc = "Register `STIM18` reader"]
pub type R = crate::R<Stim18Spec>;
#[doc = "Register `STIM18` writer"]
pub type W = crate::W<Stim18Spec>;
#[doc = "Field `STIM18` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA18 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim18R = crate::FieldReader<u32>;
#[doc = "Field `STIM18` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA18 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim18W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA18 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim18(&self) -> Stim18R {
        Stim18R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA18 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim18(&mut self) -> Stim18W<Stim18Spec> {
        Stim18W::new(self, 0)
    }
}
#[doc = "Stimulus Port 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim18Spec;
impl crate::RegisterSpec for Stim18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim18::R`](R) reader structure"]
impl crate::Readable for Stim18Spec {}
#[doc = "`write(|w| ..)` method takes [`stim18::W`](W) writer structure"]
impl crate::Writable for Stim18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM18 to value 0"]
impl crate::Resettable for Stim18Spec {
    const RESET_VALUE: u32 = 0;
}
