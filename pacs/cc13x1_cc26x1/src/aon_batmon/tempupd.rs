#[doc = "Register `TEMPUPD` reader"]
pub type R = crate::R<TempupdSpec>;
#[doc = "Register `TEMPUPD` writer"]
pub type W = crate::W<TempupdSpec>;
#[doc = "Field `STAT` reader - 0:0\\]
0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
pub type StatR = crate::BitReader;
#[doc = "Field `STAT` writer - 0:0\\]
0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
pub type StatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
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
0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<TempupdSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "Temperature Update Indicates TEMP Updates\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempupd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempupd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempupdSpec;
impl crate::RegisterSpec for TempupdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tempupd::R`](R) reader structure"]
impl crate::Readable for TempupdSpec {}
#[doc = "`write(|w| ..)` method takes [`tempupd::W`](W) writer structure"]
impl crate::Writable for TempupdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEMPUPD to value 0"]
impl crate::Resettable for TempupdSpec {
    const RESET_VALUE: u32 = 0;
}
