#[doc = "Register `STIM20` reader"]
pub type R = crate::R<Stim20Spec>;
#[doc = "Register `STIM20` writer"]
pub type W = crate::W<Stim20Spec>;
#[doc = "Field `STIM20` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA20 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim20R = crate::FieldReader<u32>;
#[doc = "Field `STIM20` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA20 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA20 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim20(&self) -> Stim20R {
        Stim20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA20 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim20(&mut self) -> Stim20W<Stim20Spec> {
        Stim20W::new(self, 0)
    }
}
#[doc = "Stimulus Port 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim20Spec;
impl crate::RegisterSpec for Stim20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim20::R`](R) reader structure"]
impl crate::Readable for Stim20Spec {}
#[doc = "`write(|w| ..)` method takes [`stim20::W`](W) writer structure"]
impl crate::Writable for Stim20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM20 to value 0"]
impl crate::Resettable for Stim20Spec {
    const RESET_VALUE: u32 = 0;
}
