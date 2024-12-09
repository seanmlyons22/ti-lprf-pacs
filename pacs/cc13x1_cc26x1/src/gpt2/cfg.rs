#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg {
    #[doc = "4: 16-bit timer configuration. Configure for two 16-bit timers. Also see TAMR.TAMR and TBMR.TBMR."]
    _16bitTimer = 4,
    #[doc = "0: 32-bit timer configuration"]
    _32bitTimer = 0,
}
impl From<Cfg> for u8 {
    #[inline(always)]
    fn from(variant: Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg {
    type Ux = u8;
}
impl crate::IsEnum for Cfg {}
#[doc = "Field `CFG` reader - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
pub type CfgR = crate::FieldReader<Cfg>;
impl CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfg> {
        match self.bits {
            4 => Some(Cfg::_16bitTimer),
            0 => Some(Cfg::_32bitTimer),
            _ => None,
        }
    }
    #[doc = "16-bit timer configuration. Configure for two 16-bit timers. Also see TAMR.TAMR and TBMR.TBMR."]
    #[inline(always)]
    pub fn is_16bit_timer(&self) -> bool {
        *self == Cfg::_16bitTimer
    }
    #[doc = "32-bit timer configuration"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        *self == Cfg::_32bitTimer
    }
}
#[doc = "Field `CFG` writer - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfg>;
impl<'a, REG> CfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit timer configuration. Configure for two 16-bit timers. Also see TAMR.TAMR and TBMR.TBMR."]
    #[inline(always)]
    pub fn _16bit_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg::_16bitTimer)
    }
    #[doc = "32-bit timer configuration"]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg::_32bitTimer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<CfgSpec> {
        CfgW::new(self, 0)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
