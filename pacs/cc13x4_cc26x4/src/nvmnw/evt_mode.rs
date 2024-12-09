#[doc = "Register `EVT_MODE` reader"]
pub type R = crate::R<EvtModeSpec>;
#[doc = "Register `EVT_MODE` writer"]
pub type W = crate::W<EvtModeSpec>;
#[doc = "1:0\\]
Event line mode select for peripheral event\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Int0Cfg {
    #[doc = "2: The interrupt or event line is in hardware mode. Hardware should clear the RIS."]
    Hardware = 2,
    #[doc = "1: The interrupt or event line is in software mode. Software must clear the RIS."]
    Software = 1,
    #[doc = "0: The interrupt or event line is disabled."]
    Disable = 0,
}
impl From<Int0Cfg> for u8 {
    #[inline(always)]
    fn from(variant: Int0Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Int0Cfg {
    type Ux = u8;
}
impl crate::IsEnum for Int0Cfg {}
#[doc = "Field `INT0_CFG` reader - 1:0\\]
Event line mode select for peripheral event"]
pub type Int0CfgR = crate::FieldReader<Int0Cfg>;
impl Int0CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Int0Cfg> {
        match self.bits {
            2 => Some(Int0Cfg::Hardware),
            1 => Some(Int0Cfg::Software),
            0 => Some(Int0Cfg::Disable),
            _ => None,
        }
    }
    #[doc = "The interrupt or event line is in hardware mode. Hardware should clear the RIS."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == Int0Cfg::Hardware
    }
    #[doc = "The interrupt or event line is in software mode. Software must clear the RIS."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Int0Cfg::Software
    }
    #[doc = "The interrupt or event line is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Int0Cfg::Disable
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Event line mode select for peripheral event"]
    #[inline(always)]
    pub fn int0_cfg(&self) -> Int0CfgR {
        Int0CfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {}
#[doc = "Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtModeSpec;
impl crate::RegisterSpec for EvtModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_mode::R`](R) reader structure"]
impl crate::Readable for EvtModeSpec {}
#[doc = "`write(|w| ..)` method takes [`evt_mode::W`](W) writer structure"]
impl crate::Writable for EvtModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVT_MODE to value 0x01"]
impl crate::Resettable for EvtModeSpec {
    const RESET_VALUE: u32 = 0x01;
}
