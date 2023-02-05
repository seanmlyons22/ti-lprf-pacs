#[doc = "Register `STARTCFG` reader"]
pub type R = crate::R<StartcfgSpec>;
#[doc = "Register `STARTCFG` writer"]
pub type W = crate::W<StartcfgSpec>;
#[doc = "1:0\\]
LGPT start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lgpt0 {
    #[doc = "0: LGPT starts when synchronized event input is high. Configured here EVTSVT.LGPTSYNCSEL."]
    EvSync = 0,
}
impl From<Lgpt0> for u8 {
    #[inline(always)]
    fn from(variant: Lgpt0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lgpt0 {
    type Ux = u8;
}
impl crate::IsEnum for Lgpt0 {}
#[doc = "Field `LGPT0` reader - 1:0\\]
LGPT start"]
pub type Lgpt0R = crate::FieldReader<Lgpt0>;
impl Lgpt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lgpt0> {
        match self.bits {
            0 => Some(Lgpt0::EvSync),
            _ => None,
        }
    }
    #[doc = "LGPT starts when synchronized event input is high. Configured here EVTSVT.LGPTSYNCSEL."]
    #[inline(always)]
    pub fn is_ev_sync(&self) -> bool {
        *self == Lgpt0::EvSync
    }
}
#[doc = "Field `LGPT0` writer - 1:0\\]
LGPT start"]
pub type Lgpt0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Lgpt0>;
impl<'a, REG> Lgpt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LGPT starts when synchronized event input is high. Configured here EVTSVT.LGPTSYNCSEL."]
    #[inline(always)]
    pub fn ev_sync(self) -> &'a mut crate::W<REG> {
        self.variant(Lgpt0::EvSync)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
LGPT start"]
    #[inline(always)]
    pub fn lgpt0(&self) -> Lgpt0R {
        Lgpt0R::new((self.bits & 3) as u8)
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
LGPT start"]
    #[inline(always)]
    #[must_use]
    pub fn lgpt0(&mut self) -> Lgpt0W<StartcfgSpec> {
        Lgpt0W::new(self, 0)
    }
}
#[doc = "Start Configuration This register is only for when CTL.MODE is configured to one of the SYNC modes. This register defines when this LGPT starts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartcfgSpec;
impl crate::RegisterSpec for StartcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startcfg::R`](R) reader structure"]
impl crate::Readable for StartcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`startcfg::W`](W) writer structure"]
impl crate::Writable for StartcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTCFG to value 0"]
impl crate::Resettable for StartcfgSpec {
    const RESET_VALUE: u32 = 0;
}
