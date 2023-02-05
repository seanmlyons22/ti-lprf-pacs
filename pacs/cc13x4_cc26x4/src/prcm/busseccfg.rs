#[doc = "Register `BUSSECCFG` reader"]
pub struct R(crate::R<BUSSECCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSECCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSECCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSECCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSSECCFG` writer"]
pub struct W(crate::W<BUSSECCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSSECCFG_SPEC>;
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
impl From<crate::W<BUSSECCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSSECCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_CFG` reader - 7:0\\]
Bus interconnect security and firewall configuration 0xFF : Trustzone enabled 0xF9 : Trustzone disabled Others: Reserved. Software should not rely on the value of reserved and should not write reserved settings."]
pub type BUS_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUS_CFG` writer - 7:0\\]
Bus interconnect security and firewall configuration 0xFF : Trustzone enabled 0xF9 : Trustzone disabled Others: Reserved. Software should not rely on the value of reserved and should not write reserved settings."]
pub type BUS_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUSSECCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 30:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 30:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BUSSECCFG_SPEC, u32, u32, 23, O>;
#[doc = "Field `VALID` reader - 31:31\\]
Security configuration valid Registers that needs to be followed by VALID before settings being applied are: - NVMNSCADDR - NVMNSADDR - SRAMNSCADDR - SRAMNSADDR - BUSSECCFG - CPULOCK"]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - 31:31\\]
Security configuration valid Registers that needs to be followed by VALID before settings being applied are: - NVMNSCADDR - NVMNSADDR - SRAMNSCADDR - SRAMNSADDR - BUSSECCFG - CPULOCK"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSSECCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bus interconnect security and firewall configuration 0xFF : Trustzone enabled 0xF9 : Trustzone disabled Others: Reserved. Software should not rely on the value of reserved and should not write reserved settings."]
    #[inline(always)]
    pub fn bus_cfg(&self) -> BUS_CFG_R {
        BUS_CFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:30 - 30:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x007f_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Security configuration valid Registers that needs to be followed by VALID before settings being applied are: - NVMNSCADDR - NVMNSADDR - SRAMNSCADDR - SRAMNSADDR - BUSSECCFG - CPULOCK"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Bus interconnect security and firewall configuration 0xFF : Trustzone enabled 0xF9 : Trustzone disabled Others: Reserved. Software should not rely on the value of reserved and should not write reserved settings."]
    #[inline(always)]
    #[must_use]
    pub fn bus_cfg(&mut self) -> BUS_CFG_W<0> {
        BUS_CFG_W::new(self)
    }
    #[doc = "Bits 8:30 - 30:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Security configuration valid Registers that needs to be followed by VALID before settings being applied are: - NVMNSCADDR - NVMNSADDR - SRAMNSCADDR - SRAMNSADDR - BUSSECCFG - CPULOCK"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BUS Security Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busseccfg](index.html) module"]
pub struct BUSSECCFG_SPEC;
impl crate::RegisterSpec for BUSSECCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busseccfg::R](R) reader structure"]
impl crate::Readable for BUSSECCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busseccfg::W](W) writer structure"]
impl crate::Writable for BUSSECCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSECCFG to value 0xff"]
impl crate::Resettable for BUSSECCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
