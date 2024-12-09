#[doc = "Register `RECHARGESTAT` reader"]
pub type R = crate::R<RechargestatSpec>;
#[doc = "Register `RECHARGESTAT` writer"]
pub type W = crate::W<RechargestatSpec>;
#[doc = "Field `MAX_USED_PER` reader - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
pub type MaxUsedPerR = crate::FieldReader<u16>;
#[doc = "Field `MAX_USED_PER` writer - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
pub type MaxUsedPerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VDDR_SMPLS` reader - 19:16\\]
The last 4 VDDR samples. For each bit: 0: VDDR was below VDDR_OK threshold when recharge started 1: VDDR was above VDDR_OK threshold when recharge started The register is updated prior to every recharge period with a shift left, and bit 0 is updated with the last VDDR sample."]
pub type VddrSmplsR = crate::FieldReader;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn max_used_per(&self) -> MaxUsedPerR {
        MaxUsedPerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The last 4 VDDR samples. For each bit: 0: VDDR was below VDDR_OK threshold when recharge started 1: VDDR was above VDDR_OK threshold when recharge started The register is updated prior to every recharge period with a shift left, and bit 0 is updated with the last VDDR sample."]
    #[inline(always)]
    pub fn vddr_smpls(&self) -> VddrSmplsR {
        VddrSmplsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    #[must_use]
    pub fn max_used_per(&mut self) -> MaxUsedPerW<RechargestatSpec> {
        MaxUsedPerW::new(self, 0)
    }
}
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargestat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargestat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RechargestatSpec;
impl crate::RegisterSpec for RechargestatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rechargestat::R`](R) reader structure"]
impl crate::Readable for RechargestatSpec {}
#[doc = "`write(|w| ..)` method takes [`rechargestat::W`](W) writer structure"]
impl crate::Writable for RechargestatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECHARGESTAT to value 0"]
impl crate::Resettable for RechargestatSpec {
    const RESET_VALUE: u32 = 0;
}
