#[doc = "Register `CLKSHIFTDET` reader"]
pub type R = crate::R<ClkshiftdetSpec>;
#[doc = "Register `CLKSHIFTDET` writer"]
pub type W = crate::W<ClkshiftdetSpec>;
#[doc = "Field `STAT` reader - 0:0\\]
Clock shift detection. Write: 0: Restart clock shift detection. 1: Do not use. Read: 0: MCU domain did not enter or exit active state since you wrote 0 to STAT. 1: MCU domain entered or exited active state since you wrote 0 to STAT."]
pub type StatR = crate::BitReader;
#[doc = "Field `STAT` writer - 0:0\\]
Clock shift detection. Write: 0: Restart clock shift detection. 1: Do not use. Read: 0: MCU domain did not enter or exit active state since you wrote 0 to STAT. 1: MCU domain entered or exited active state since you wrote 0 to STAT."]
pub type StatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clock shift detection. Write: 0: Restart clock shift detection. 1: Do not use. Read: 0: MCU domain did not enter or exit active state since you wrote 0 to STAT. 1: MCU domain entered or exited active state since you wrote 0 to STAT."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 1) != 0)
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
Clock shift detection. Write: 0: Restart clock shift detection. 1: Do not use. Read: 0: MCU domain did not enter or exit active state since you wrote 0 to STAT. 1: MCU domain entered or exited active state since you wrote 0 to STAT."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<ClkshiftdetSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkshiftdet::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkshiftdet::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkshiftdetSpec;
impl crate::RegisterSpec for ClkshiftdetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkshiftdet::R`](R) reader structure"]
impl crate::Readable for ClkshiftdetSpec {}
#[doc = "`write(|w| ..)` method takes [`clkshiftdet::W`](W) writer structure"]
impl crate::Writable for ClkshiftdetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSHIFTDET to value 0x01"]
impl crate::Resettable for ClkshiftdetSpec {
    const RESET_VALUE: u32 = 0x01;
}
