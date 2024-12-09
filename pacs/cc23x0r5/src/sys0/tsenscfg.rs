#[doc = "Register `TSENSCFG` reader"]
pub type R = crate::R<TsenscfgSpec>;
#[doc = "Register `TSENSCFG` writer"]
pub type W = crate::W<TsenscfgSpec>;
#[doc = "1:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Gnd = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Value = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Disable = 0,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            2 => Some(Sel::Gnd),
            1 => Some(Sel::Value),
            0 => Some(Sel::Disable),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Sel::Gnd
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == Sel::Value
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sel::Disable
    }
}
#[doc = "Field `SEL` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Gnd)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn value(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Value)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Disable)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SPARE` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<TsenscfgSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<TsenscfgSpec> {
        SpareW::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsenscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsenscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsenscfgSpec;
impl crate::RegisterSpec for TsenscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsenscfg::R`](R) reader structure"]
impl crate::Readable for TsenscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tsenscfg::W`](W) writer structure"]
impl crate::Writable for TsenscfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSENSCFG to value 0"]
impl crate::Resettable for TsenscfgSpec {
    const RESET_VALUE: u32 = 0;
}
