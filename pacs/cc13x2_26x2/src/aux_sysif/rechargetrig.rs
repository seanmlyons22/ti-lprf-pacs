#[doc = "Register `RECHARGETRIG` reader"]
pub type R = crate::R<RechargetrigSpec>;
#[doc = "Register `RECHARGETRIG` writer"]
pub type W = crate::W<RechargetrigSpec>;
#[doc = "Field `TRIG` reader - 0:0\\]
Recharge trigger. 0: No effect. 1: Request VDDR recharge. Request VDDR recharge only when AUX_EVCTL:EVSTAT2.PWR_DWN is 1. Follow this sequence when OPMODEREQ.REQ is LP: - Set TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 1. - Clear TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 0. Follow this sequence when OPMODEREQ.REQ is PDA or PDLP: - Set TRIG. - Clear TRIG."]
pub type TrigR = crate::BitReader;
#[doc = "Field `TRIG` writer - 0:0\\]
Recharge trigger. 0: No effect. 1: Request VDDR recharge. Request VDDR recharge only when AUX_EVCTL:EVSTAT2.PWR_DWN is 1. Follow this sequence when OPMODEREQ.REQ is LP: - Set TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 1. - Clear TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 0. Follow this sequence when OPMODEREQ.REQ is PDA or PDLP: - Set TRIG. - Clear TRIG."]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Recharge trigger. 0: No effect. 1: Request VDDR recharge. Request VDDR recharge only when AUX_EVCTL:EVSTAT2.PWR_DWN is 1. Follow this sequence when OPMODEREQ.REQ is LP: - Set TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 1. - Clear TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 0. Follow this sequence when OPMODEREQ.REQ is PDA or PDLP: - Set TRIG. - Clear TRIG."]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Recharge trigger. 0: No effect. 1: Request VDDR recharge. Request VDDR recharge only when AUX_EVCTL:EVSTAT2.PWR_DWN is 1. Follow this sequence when OPMODEREQ.REQ is LP: - Set TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 1. - Clear TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 0. Follow this sequence when OPMODEREQ.REQ is PDA or PDLP: - Set TRIG. - Clear TRIG."]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TrigW<RechargetrigSpec> {
        TrigW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<RechargetrigSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "VDDR Recharge Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargetrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargetrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RechargetrigSpec;
impl crate::RegisterSpec for RechargetrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rechargetrig::R`](R) reader structure"]
impl crate::Readable for RechargetrigSpec {}
#[doc = "`write(|w| ..)` method takes [`rechargetrig::W`](W) writer structure"]
impl crate::Writable for RechargetrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECHARGETRIG to value 0"]
impl crate::Resettable for RechargetrigSpec {
    const RESET_VALUE: u32 = 0;
}
