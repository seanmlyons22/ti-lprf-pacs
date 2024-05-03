#[doc = "Register `DATAIDLE` reader"]
pub type R = crate::R<DataidleSpec>;
#[doc = "Register `DATAIDLE` writer"]
pub type W = crate::W<DataidleSpec>;
#[doc = "Field `STAT` reader - 0:0\\]
Wait for data idle. Read operation stalls until the SCLK period associated with LSB transmission completes. Read then returns 1. AUX_SCE can use this to control CS deassertion."]
pub type StatR = crate::BitReader;
#[doc = "Field `STAT` writer - 0:0\\]
Wait for data idle. Read operation stalls until the SCLK period associated with LSB transmission completes. Read then returns 1. AUX_SCE can use this to control CS deassertion."]
pub type StatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Wait for data idle. Read operation stalls until the SCLK period associated with LSB transmission completes. Read then returns 1. AUX_SCE can use this to control CS deassertion."]
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
Wait for data idle. Read operation stalls until the SCLK period associated with LSB transmission completes. Read then returns 1. AUX_SCE can use this to control CS deassertion."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<DataidleSpec> {
        StatW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DataidleSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Data Idle Read operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataidle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataidle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataidleSpec;
impl crate::RegisterSpec for DataidleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataidle::R`](R) reader structure"]
impl crate::Readable for DataidleSpec {}
#[doc = "`write(|w| ..)` method takes [`dataidle::W`](W) writer structure"]
impl crate::Writable for DataidleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAIDLE to value 0x01"]
impl crate::Resettable for DataidleSpec {
    const RESET_VALUE: u32 = 0x01;
}
