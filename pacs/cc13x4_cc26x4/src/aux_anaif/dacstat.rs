#[doc = "Register `DACSTAT` reader"]
pub type R = crate::R<DacstatSpec>;
#[doc = "Register `DACSTAT` writer"]
pub type W = crate::W<DacstatSpec>;
#[doc = "Field `HOLD_ACTIVE` reader - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
pub type HoldActiveR = crate::BitReader;
#[doc = "Field `HOLD_ACTIVE` writer - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
pub type HoldActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP_ACTIVE` reader - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
pub type SetupActiveR = crate::BitReader;
#[doc = "Field `SETUP_ACTIVE` writer - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
pub type SetupActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
    #[inline(always)]
    pub fn hold_active(&self) -> HoldActiveR {
        HoldActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
    #[inline(always)]
    pub fn setup_active(&self) -> SetupActiveR {
        SetupActiveR::new(((self.bits >> 1) & 1) != 0)
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
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
    #[inline(always)]
    #[must_use]
    pub fn hold_active(&mut self) -> HoldActiveW<DacstatSpec> {
        HoldActiveW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
    #[inline(always)]
    #[must_use]
    pub fn setup_active(&mut self) -> SetupActiveW<DacstatSpec> {
        SetupActiveW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<DacstatSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "DAC Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacstatSpec;
impl crate::RegisterSpec for DacstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacstat::R`](R) reader structure"]
impl crate::Readable for DacstatSpec {}
#[doc = "`write(|w| ..)` method takes [`dacstat::W`](W) writer structure"]
impl crate::Writable for DacstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACSTAT to value 0"]
impl crate::Resettable for DacstatSpec {
    const RESET_VALUE: u32 = 0;
}
