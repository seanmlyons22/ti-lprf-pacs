#[doc = "Register `MEASCFG` reader"]
pub type R = crate::R<MeascfgSpec>;
#[doc = "Register `MEASCFG` writer"]
pub type W = crate::W<MeascfgSpec>;
#[doc = "1:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Per {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _32cyc = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _16cyc = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _8cyc = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Cont = 0,
}
impl From<Per> for u8 {
    #[inline(always)]
    fn from(variant: Per) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Per {
    type Ux = u8;
}
impl crate::IsEnum for Per {}
#[doc = "Field `PER` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type PerR = crate::FieldReader<Per>;
impl PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Per {
        match self.bits {
            3 => Per::_32cyc,
            2 => Per::_16cyc,
            1 => Per::_8cyc,
            0 => Per::Cont,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_32cyc(&self) -> bool {
        *self == Per::_32cyc
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_16cyc(&self) -> bool {
        *self == Per::_16cyc
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_8cyc(&self) -> bool {
        *self == Per::_8cyc
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_cont(&self) -> bool {
        *self == Per::Cont
    }
}
#[doc = "Field `PER` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 2, Per, crate::Safe>;
impl<'a, REG> PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _32cyc(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_32cyc)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _16cyc(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_16cyc)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _8cyc(self) -> &'a mut crate::W<REG> {
        self.variant(Per::_8cyc)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cont(self) -> &'a mut crate::W<REG> {
        self.variant(Per::Cont)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<MeascfgSpec> {
        PerW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<MeascfgSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`meascfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`meascfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeascfgSpec;
impl crate::RegisterSpec for MeascfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meascfg::R`](R) reader structure"]
impl crate::Readable for MeascfgSpec {}
#[doc = "`write(|w| ..)` method takes [`meascfg::W`](W) writer structure"]
impl crate::Writable for MeascfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEASCFG to value 0"]
impl crate::Resettable for MeascfgSpec {
    const RESET_VALUE: u32 = 0;
}
