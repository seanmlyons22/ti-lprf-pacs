#[doc = "Register `CPUIRQSEL25` reader"]
pub type R = crate::R<Cpuirqsel25Spec>;
#[doc = "Register `CPUIRQSEL25` writer"]
pub type W = crate::W<Cpuirqsel25Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 38"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "38: DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    DmaErr = 38,
}
impl From<Ev> for u8 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u8;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 6:0\\]
Read only selection value"]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            38 => Some(Ev::DmaErr),
            _ => None,
        }
    }
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    #[inline(always)]
    pub fn is_dma_err(&self) -> bool {
        *self == Ev::DmaErr
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read only selection value"]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 7, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    #[inline(always)]
    pub fn dma_err(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::DmaErr)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Cpuirqsel25Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel25Spec;
impl crate::RegisterSpec for Cpuirqsel25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel25::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel25Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel25::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL25 to value 0x26"]
impl crate::Resettable for Cpuirqsel25Spec {
    const RESET_VALUE: u32 = 0x26;
}
