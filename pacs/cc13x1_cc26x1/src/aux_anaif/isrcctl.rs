#[doc = "Register `ISRCCTL` reader"]
pub type R = crate::R<IsrcctlSpec>;
#[doc = "Register `ISRCCTL` writer"]
pub type W = crate::W<IsrcctlSpec>;
#[doc = "Field `RESET_N` reader - 0:0\\]
ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
pub type ResetNR = crate::BitReader;
#[doc = "Field `RESET_N` writer - 0:0\\]
ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
pub type ResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
    #[inline(always)]
    pub fn reset_n(&self) -> ResetNR {
        ResetNR::new((self.bits & 1) != 0)
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
ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
    #[inline(always)]
    #[must_use]
    pub fn reset_n(&mut self) -> ResetNW<IsrcctlSpec> {
        ResetNW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<IsrcctlSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Current Source Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isrcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isrcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrcctlSpec;
impl crate::RegisterSpec for IsrcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isrcctl::R`](R) reader structure"]
impl crate::Readable for IsrcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`isrcctl::W`](W) writer structure"]
impl crate::Writable for IsrcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISRCCTL to value 0x01"]
impl crate::Resettable for IsrcctlSpec {
    const RESET_VALUE: u32 = 0x01;
}
