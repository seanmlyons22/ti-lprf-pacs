#[doc = "Register `WUSTA` reader"]
pub type R = crate::R<WustaSpec>;
#[doc = "Register `WUSTA` writer"]
pub type W = crate::W<WustaSpec>;
#[doc = "1:0\\]
This field shows the device wakeup source.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "2: Wakeup from STANDBY mode."]
    Stby = 2,
    #[doc = "1: Wakeup from system reset / SHUTDOWN mode. See RSTSTA for more status information."]
    RstShtdwn = 1,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - 1:0\\]
This field shows the device wakeup source."]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            2 => Some(Src::Stby),
            1 => Some(Src::RstShtdwn),
            _ => None,
        }
    }
    #[doc = "Wakeup from STANDBY mode."]
    #[inline(always)]
    pub fn is_stby(&self) -> bool {
        *self == Src::Stby
    }
    #[doc = "Wakeup from system reset / SHUTDOWN mode. See RSTSTA for more status information."]
    #[inline(always)]
    pub fn is_rst_shtdwn(&self) -> bool {
        *self == Src::RstShtdwn
    }
}
#[doc = "Field `SRC` writer - 1:0\\]
This field shows the device wakeup source."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wakeup from STANDBY mode."]
    #[inline(always)]
    pub fn stby(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Stby)
    }
    #[doc = "Wakeup from system reset / SHUTDOWN mode. See RSTSTA for more status information."]
    #[inline(always)]
    pub fn rst_shtdwn(self) -> &'a mut crate::W<REG> {
        self.variant(Src::RstShtdwn)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
This field shows the device wakeup source."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
This field shows the device wakeup source."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<WustaSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<WustaSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Wakeup Status Register This register shows the device wakeup source. Used to distinguish between wakeup from STANDBY, SHUTDOWN and reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wusta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wusta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WustaSpec;
impl crate::RegisterSpec for WustaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wusta::R`](R) reader structure"]
impl crate::Readable for WustaSpec {}
#[doc = "`write(|w| ..)` method takes [`wusta::W`](W) writer structure"]
impl crate::Writable for WustaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUSTA to value 0x01"]
impl crate::Resettable for WustaSpec {
    const RESET_VALUE: u32 = 0x01;
}
