#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_MODE` reader - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
pub type POWER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `POWER_MODE` writer - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
pub type POWER_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `SAMHOLD_DIS` reader - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
pub type SAMHOLD_DIS_R = crate::BitReader<bool>;
#[doc = "Field `SAMHOLD_DIS` writer - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
pub type SAMHOLD_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 5, O>;
#[doc = "Field `EFUSE_ERRCODE` reader - 12:8\\]
Same as EFUSEERROR.CODE"]
pub type EFUSE_ERRCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFUSE_ERRCODE` writer - 12:8\\]
Same as EFUSEERROR.CODE"]
pub type EFUSE_ERRCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 5, O>;
#[doc = "Field `SPRS_BYTE_NOT_OK` reader - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
pub type SPRS_BYTE_NOT_OK_R = crate::BitReader<bool>;
#[doc = "Field `SPRS_BYTE_NOT_OK` writer - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
pub type SPRS_BYTE_NOT_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `EFUSE_TIMEOUT` reader - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub type EFUSE_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_TIMEOUT` writer - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
pub type EFUSE_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `EFUSE_BLANK` reader - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub type EFUSE_BLANK_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_BLANK` writer - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
pub type EFUSE_BLANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
    #[inline(always)]
    pub fn samhold_dis(&self) -> SAMHOLD_DIS_R {
        SAMHOLD_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    pub fn efuse_errcode(&self) -> EFUSE_ERRCODE_R {
        EFUSE_ERRCODE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
    #[inline(always)]
    pub fn sprs_byte_not_ok(&self) -> SPRS_BYTE_NOT_OK_R {
        SPRS_BYTE_NOT_OK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    pub fn efuse_timeout(&self) -> EFUSE_TIMEOUT_R {
        EFUSE_TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    pub fn efuse_blank(&self) -> EFUSE_BLANK_R {
        EFUSE_BLANK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power state of the flash sub-system. 0 : Active 1 : Low power"]
    #[inline(always)]
    #[must_use]
    pub fn power_mode(&mut self) -> POWER_MODE_W<0> {
        POWER_MODE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<1> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable"]
    #[inline(always)]
    #[must_use]
    pub fn samhold_dis(&mut self) -> SAMHOLD_DIS_W<2> {
        SAMHOLD_DIS_W::new(self)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Same as EFUSEERROR.CODE"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_errcode(&mut self) -> EFUSE_ERRCODE_W<8> {
        EFUSE_ERRCODE_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Efuse scanning resulted in scan chain Sparse byte error. 0 : No Sparse error 1 : Sparse Error"]
    #[inline(always)]
    #[must_use]
    pub fn sprs_byte_not_ok(&mut self) -> SPRS_BYTE_NOT_OK_W<13> {
        SPRS_BYTE_NOT_OK_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_timeout(&mut self) -> EFUSE_TIMEOUT_W<14> {
        EFUSE_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_blank(&mut self) -> EFUSE_BLANK_W<15> {
        EFUSE_BLANK_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC and Efuse Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
