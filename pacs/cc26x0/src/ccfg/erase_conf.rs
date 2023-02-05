#[doc = "Register `ERASE_CONF` reader"]
pub struct R(crate::R<ERASE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASE_CONF` writer"]
pub struct W(crate::W<ERASE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASE_CONF_SPEC>;
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
impl From<crate::W<ERASE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BANK_ERASE_DIS_N` reader - 0:0\\]
Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
pub type BANK_ERASE_DIS_N_R = crate::BitReader<bool>;
#[doc = "Field `BANK_ERASE_DIS_N` writer - 0:0\\]
Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
pub type BANK_ERASE_DIS_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERASE_CONF_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ERASE_CONF_SPEC, u8, u8, 7, O>;
#[doc = "Field `CHIP_ERASE_DIS_N` reader - 8:8\\]
Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
pub type CHIP_ERASE_DIS_N_R = crate::BitReader<bool>;
#[doc = "Field `CHIP_ERASE_DIS_N` writer - 8:8\\]
Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
pub type CHIP_ERASE_DIS_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERASE_CONF_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ERASE_CONF_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
    #[inline(always)]
    pub fn bank_erase_dis_n(&self) -> BANK_ERASE_DIS_N_R {
        BANK_ERASE_DIS_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
    #[inline(always)]
    pub fn chip_erase_dis_n(&self) -> CHIP_ERASE_DIS_N_R {
        CHIP_ERASE_DIS_N_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
    #[inline(always)]
    #[must_use]
    pub fn bank_erase_dis_n(&mut self) -> BANK_ERASE_DIS_N_W<0> {
        BANK_ERASE_DIS_N_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
    #[inline(always)]
    #[must_use]
    pub fn chip_erase_dis_n(&mut self) -> CHIP_ERASE_DIS_N_W<8> {
        CHIP_ERASE_DIS_N_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<9> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Erase Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erase_conf](index.html) module"]
pub struct ERASE_CONF_SPEC;
impl crate::RegisterSpec for ERASE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erase_conf::R](R) reader structure"]
impl crate::Readable for ERASE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erase_conf::W](W) writer structure"]
impl crate::Writable for ERASE_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERASE_CONF to value 0xffff_ffff"]
impl crate::Resettable for ERASE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
