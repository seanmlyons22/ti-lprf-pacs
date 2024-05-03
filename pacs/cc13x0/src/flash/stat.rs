#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `POWER_MODE` reader - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
pub type PowerModeR = crate::BitReader;
#[doc = "Field `POWER_MODE` writer - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
pub type PowerModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMHOLD_DIS` reader - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
pub type SamholdDisR = crate::BitReader;
#[doc = "Field `SAMHOLD_DIS` writer - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
pub type SamholdDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFUSE_ERRCODE` reader - 12:8\\]
Same as EFUSEERROR.CODE"]
pub type EfuseErrcodeR = crate::FieldReader;
#[doc = "Field `EFUSE_ERRCODE` writer - 12:8\\]
Same as EFUSEERROR.CODE"]
pub type EfuseErrcodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFUSE_CRC_ERROR` reader - 13:13\\]
Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error"]
pub type EfuseCrcErrorR = crate::BitReader;
#[doc = "Field `EFUSE_CRC_ERROR` writer - 13:13\\]
Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error"]
pub type EfuseCrcErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_TIMEOUT` reader - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub type EfuseTimeoutR = crate::BitReader;
#[doc = "Field `EFUSE_TIMEOUT` writer - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub type EfuseTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_BLANK` reader - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub type EfuseBlankR = crate::BitReader;
#[doc = "Field `EFUSE_BLANK` writer - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub type EfuseBlankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
    #[inline(always)]
    pub fn power_mode(&self) -> PowerModeR {
        PowerModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
    #[inline(always)]
    pub fn samhold_dis(&self) -> SamholdDisR {
        SamholdDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    pub fn efuse_errcode(&self) -> EfuseErrcodeR {
        EfuseErrcodeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error"]
    #[inline(always)]
    pub fn efuse_crc_error(&self) -> EfuseCrcErrorR {
        EfuseCrcErrorR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    pub fn efuse_timeout(&self) -> EfuseTimeoutR {
        EfuseTimeoutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    pub fn efuse_blank(&self) -> EfuseBlankR {
        EfuseBlankR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode(&mut self) -> PowerModeW<StatSpec> {
        PowerModeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<StatSpec> {
        BusyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
    #[inline(always)]
    #[must_use]
    pub fn samhold_dis(&mut self) -> SamholdDisW<StatSpec> {
        SamholdDisW::new(self, 2)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<StatSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_errcode(&mut self) -> EfuseErrcodeW<StatSpec> {
        EfuseErrcodeW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_crc_error(&mut self) -> EfuseCrcErrorW<StatSpec> {
        EfuseCrcErrorW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_timeout(&mut self) -> EfuseTimeoutW<StatSpec> {
        EfuseTimeoutW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_blank(&mut self) -> EfuseBlankW<StatSpec> {
        EfuseBlankW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<StatSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "FMC and Efuse Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
