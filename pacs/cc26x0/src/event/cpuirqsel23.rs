#[doc = "Register `CPUIRQSEL23` reader"]
pub type R = crate::R<Cpuirqsel23Spec>;
#[doc = "Register `CPUIRQSEL23` writer"]
pub type W = crate::W<Cpuirqsel23Spec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 93"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "93: CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    CryptoResultAvailIrq = 93,
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
            93 => Some(Ev::CryptoResultAvailIrq),
            _ => None,
        }
    }
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    #[inline(always)]
    pub fn is_crypto_result_avail_irq(&self) -> bool {
        *self == Ev::CryptoResultAvailIrq
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
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    #[inline(always)]
    pub fn crypto_result_avail_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::CryptoResultAvailIrq)
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
    pub fn ev(&mut self) -> EvW<Cpuirqsel23Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel23Spec;
impl crate::RegisterSpec for Cpuirqsel23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel23::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel23Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel23::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL23 to value 0x5d"]
impl crate::Resettable for Cpuirqsel23Spec {
    const RESET_VALUE: u32 = 0x5d;
}
