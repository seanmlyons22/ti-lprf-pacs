#[doc = "Register `BUSSECCFG` reader"]
pub type R = crate::R<BusseccfgSpec>;
#[doc = "Register `BUSSECCFG` writer"]
pub type W = crate::W<BusseccfgSpec>;
#[doc = "Field `BUS_CFG` reader - 7:0\\]
Bus interconnect security and firewall configuration 0xFF : Trustzone enabled 0xF9 : Trustzone disabled Others: Reserved. Software should not rely on the value of reserved and should not write reserved settings."]
pub type BusCfgR = crate::FieldReader;
#[doc = "Field `BUS_CFG` writer - 7:0\\]
Bus interconnect security and firewall configuration 0xFF : Trustzone enabled 0xF9 : Trustzone disabled Others: Reserved. Software should not rely on the value of reserved and should not write reserved settings."]
pub type BusCfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 30:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 30:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `VALID` reader - 31:31\\]
Security configuration valid Registers that needs to be followed by VALID before settings being applied are: - NVMNSCADDR - NVMNSADDR - SRAMNSCADDR - SRAMNSADDR - BUSSECCFG - CPULOCK"]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - 31:31\\]
Security configuration valid Registers that needs to be followed by VALID before settings being applied are: - NVMNSCADDR - NVMNSADDR - SRAMNSCADDR - SRAMNSADDR - BUSSECCFG - CPULOCK"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bus interconnect security and firewall configuration 0xFF : Trustzone enabled 0xF9 : Trustzone disabled Others: Reserved. Software should not rely on the value of reserved and should not write reserved settings."]
    #[inline(always)]
    pub fn bus_cfg(&self) -> BusCfgR {
        BusCfgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:30 - 30:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x007f_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Security configuration valid Registers that needs to be followed by VALID before settings being applied are: - NVMNSCADDR - NVMNSADDR - SRAMNSCADDR - SRAMNSADDR - BUSSECCFG - CPULOCK"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Bus interconnect security and firewall configuration 0xFF : Trustzone enabled 0xF9 : Trustzone disabled Others: Reserved. Software should not rely on the value of reserved and should not write reserved settings."]
    #[inline(always)]
    #[must_use]
    pub fn bus_cfg(&mut self) -> BusCfgW<BusseccfgSpec> {
        BusCfgW::new(self, 0)
    }
    #[doc = "Bits 8:30 - 30:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<BusseccfgSpec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bit 31 - 31:31\\]
Security configuration valid Registers that needs to be followed by VALID before settings being applied are: - NVMNSCADDR - NVMNSADDR - SRAMNSCADDR - SRAMNSADDR - BUSSECCFG - CPULOCK"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<BusseccfgSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "BUS Security Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busseccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busseccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusseccfgSpec;
impl crate::RegisterSpec for BusseccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busseccfg::R`](R) reader structure"]
impl crate::Readable for BusseccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`busseccfg::W`](W) writer structure"]
impl crate::Writable for BusseccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSSECCFG to value 0xff"]
impl crate::Resettable for BusseccfgSpec {
    const RESET_VALUE: u32 = 0xff;
}
