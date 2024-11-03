#[doc = "Register `I2VCFG` reader"]
pub type R = crate::R<I2vcfgSpec>;
#[doc = "Register `I2VCFG` writer"]
pub type W = crate::W<I2vcfgSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    VaAtestA0 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    VrAtestA0 = 0,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            true => Sel::VaAtestA0,
            false => Sel::VrAtestA0,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_va_atest_a0(&self) -> bool {
        *self == Sel::VaAtestA0
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_vr_atest_a0(&self) -> bool {
        *self == Sel::VrAtestA0
    }
}
#[doc = "Field `SEL` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn va_atest_a0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::VaAtestA0)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vr_atest_a0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::VrAtestA0)
    }
}
#[doc = "3:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resval {
    #[doc = "7: Internal. Only to be used through TI provided API."]
    OffRes = 7,
    #[doc = "6: Internal. Only to be used through TI provided API."]
    Mo1 = 6,
    #[doc = "5: Internal. Only to be used through TI provided API."]
    Ko100 = 5,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Ko50 = 4,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    Ko20 = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Ko10 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Ko2 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Off = 0,
}
impl From<Resval> for u8 {
    #[inline(always)]
    fn from(variant: Resval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resval {
    type Ux = u8;
}
impl crate::IsEnum for Resval {}
#[doc = "Field `RESVAL` reader - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type ResvalR = crate::FieldReader<Resval>;
impl ResvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resval {
        match self.bits {
            7 => Resval::OffRes,
            6 => Resval::Mo1,
            5 => Resval::Ko100,
            4 => Resval::Ko50,
            3 => Resval::Ko20,
            2 => Resval::Ko10,
            1 => Resval::Ko2,
            0 => Resval::Off,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_off_res(&self) -> bool {
        *self == Resval::OffRes
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_mo1(&self) -> bool {
        *self == Resval::Mo1
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ko100(&self) -> bool {
        *self == Resval::Ko100
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ko50(&self) -> bool {
        *self == Resval::Ko50
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ko20(&self) -> bool {
        *self == Resval::Ko20
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ko10(&self) -> bool {
        *self == Resval::Ko10
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ko2(&self) -> bool {
        *self == Resval::Ko2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Resval::Off
    }
}
#[doc = "Field `RESVAL` writer - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type ResvalW<'a, REG> = crate::FieldWriter<'a, REG, 3, Resval, crate::Safe>;
impl<'a, REG> ResvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn off_res(self) -> &'a mut crate::W<REG> {
        self.variant(Resval::OffRes)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mo1(self) -> &'a mut crate::W<REG> {
        self.variant(Resval::Mo1)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ko100(self) -> &'a mut crate::W<REG> {
        self.variant(Resval::Ko100)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ko50(self) -> &'a mut crate::W<REG> {
        self.variant(Resval::Ko50)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ko20(self) -> &'a mut crate::W<REG> {
        self.variant(Resval::Ko20)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ko10(self) -> &'a mut crate::W<REG> {
        self.variant(Resval::Ko10)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ko2(self) -> &'a mut crate::W<REG> {
        self.variant(Resval::Ko2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Resval::Off)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn resval(&self) -> ResvalR {
        ResvalR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<I2vcfgSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn resval(&mut self) -> ResvalW<I2vcfgSpec> {
        ResvalW::new(self, 1)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<I2vcfgSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2vcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2vcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2vcfgSpec;
impl crate::RegisterSpec for I2vcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2vcfg::R`](R) reader structure"]
impl crate::Readable for I2vcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`i2vcfg::W`](W) writer structure"]
impl crate::Writable for I2vcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2VCFG to value 0"]
impl crate::Resettable for I2vcfgSpec {
    const RESET_VALUE: u32 = 0;
}
