#[doc = "Register `RECHARGEDET` reader"]
pub type R = crate::R<RechargedetSpec>;
#[doc = "Register `RECHARGEDET` writer"]
pub type W = crate::W<RechargedetSpec>;
#[doc = "Field `EN` reader - 0:0\\]
VDDR recharge detector enable. 0: Disable recharge detection. STAT becomes zero. 1: Enable recharge detection."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
VDDR recharge detector enable. 0: Disable recharge detection. STAT becomes zero. 1: Enable recharge detection."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAT` reader - 1:1\\]
VDDR recharge detector status. 0: No recharge of VDDR has occurred since EN was set. 1: Recharge of VDDR has occurred since EN was set."]
pub type StatR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
VDDR recharge detector enable. 0: Disable recharge detection. STAT becomes zero. 1: Enable recharge detection."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VDDR recharge detector status. 0: No recharge of VDDR has occurred since EN was set. 1: Recharge of VDDR has occurred since EN was set."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
VDDR recharge detector enable. 0: Disable recharge detection. STAT becomes zero. 1: Enable recharge detection."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<RechargedetSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargedet::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargedet::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RechargedetSpec;
impl crate::RegisterSpec for RechargedetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rechargedet::R`](R) reader structure"]
impl crate::Readable for RechargedetSpec {}
#[doc = "`write(|w| ..)` method takes [`rechargedet::W`](W) writer structure"]
impl crate::Writable for RechargedetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECHARGEDET to value 0"]
impl crate::Resettable for RechargedetSpec {
    const RESET_VALUE: u32 = 0;
}
