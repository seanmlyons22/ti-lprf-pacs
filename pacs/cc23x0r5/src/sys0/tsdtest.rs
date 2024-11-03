#[doc = "Register `TSDTEST` reader"]
pub type R = crate::R<TsdtestSpec>;
#[doc = "Register `TSDTEST` writer"]
pub type W = crate::W<TsdtestSpec>;
#[doc = "2:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atbset {
    #[doc = "6: Internal. Only to be used through TI provided API."]
    Tsdout = 6,
    #[doc = "5: Internal. Only to be used through TI provided API."]
    Compout = 5,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    EnDelay = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    EaCur = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Vref = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Vsense = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Open = 0,
}
impl From<Atbset> for u8 {
    #[inline(always)]
    fn from(variant: Atbset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atbset {
    type Ux = u8;
}
impl crate::IsEnum for Atbset {}
#[doc = "Field `ATBSET` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type AtbsetR = crate::FieldReader<Atbset>;
impl AtbsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atbset {
        match self.bits {
            6 => Atbset::Tsdout,
            5 => Atbset::Compout,
            4 => Atbset::EnDelay,
            3 => Atbset::EaCur,
            2 => Atbset::Vref,
            1 => Atbset::Vsense,
            0 => Atbset::Open,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_tsdout(&self) -> bool {
        *self == Atbset::Tsdout
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_compout(&self) -> bool {
        *self == Atbset::Compout
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_en_delay(&self) -> bool {
        *self == Atbset::EnDelay
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ea_cur(&self) -> bool {
        *self == Atbset::EaCur
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == Atbset::Vref
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vsense(&self) -> bool {
        *self == Atbset::Vsense
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Atbset::Open
    }
}
#[doc = "Field `ATBSET` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type AtbsetW<'a, REG> = crate::FieldWriter<'a, REG, 3, Atbset>;
impl<'a, REG> AtbsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tsdout(self) -> &'a mut crate::W<REG> {
        self.variant(Atbset::Tsdout)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn compout(self) -> &'a mut crate::W<REG> {
        self.variant(Atbset::Compout)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn en_delay(self) -> &'a mut crate::W<REG> {
        self.variant(Atbset::EnDelay)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ea_cur(self) -> &'a mut crate::W<REG> {
        self.variant(Atbset::EaCur)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(Atbset::Vref)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsense(self) -> &'a mut crate::W<REG> {
        self.variant(Atbset::Vsense)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Atbset::Open)
    }
}
#[doc = "Field `ATBCLR` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type AtbclrR = crate::FieldReader;
#[doc = "Field `ATBCLR` writer - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type AtbclrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atbset(&self) -> AtbsetR {
        AtbsetR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atbclr(&self) -> AtbclrR {
        AtbclrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atbset(&mut self) -> AtbsetW<TsdtestSpec> {
        AtbsetW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atbclr(&mut self) -> AtbclrW<TsdtestSpec> {
        AtbclrW::new(self, 3)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<TsdtestSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsdtest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsdtest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsdtestSpec;
impl crate::RegisterSpec for TsdtestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsdtest::R`](R) reader structure"]
impl crate::Readable for TsdtestSpec {}
#[doc = "`write(|w| ..)` method takes [`tsdtest::W`](W) writer structure"]
impl crate::Writable for TsdtestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSDTEST to value 0"]
impl crate::Resettable for TsdtestSpec {
    const RESET_VALUE: u32 = 0;
}
