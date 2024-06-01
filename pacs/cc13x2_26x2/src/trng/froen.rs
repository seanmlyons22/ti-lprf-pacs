#[doc = "Register `FROEN` reader"]
pub type R = crate::R<FroenSpec>;
#[doc = "Register `FROEN` writer"]
pub type W = crate::W<FroenSpec>;
#[doc = "Field `FRO_MASK` reader - 23:0\\]
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
pub type FroMaskR = crate::FieldReader<u32>;
#[doc = "Field `FRO_MASK` writer - 23:0\\]
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
pub type FroMaskW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
    #[inline(always)]
    pub fn fro_mask(&self) -> FroMaskR {
        FroMaskR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
    #[inline(always)]
    #[must_use]
    pub fn fro_mask(&mut self) -> FroMaskW<FroenSpec> {
        FroMaskW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<FroenSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "FRO Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`froen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`froen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FroenSpec;
impl crate::RegisterSpec for FroenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`froen::R`](R) reader structure"]
impl crate::Readable for FroenSpec {}
#[doc = "`write(|w| ..)` method takes [`froen::W`](W) writer structure"]
impl crate::Writable for FroenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FROEN to value 0x00ff_ffff"]
impl crate::Resettable for FroenSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
