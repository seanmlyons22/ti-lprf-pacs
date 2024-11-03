#[doc = "Register `TEST0` reader"]
pub type R = crate::R<Test0Spec>;
#[doc = "Register `TEST0` writer"]
pub type W = crate::W<Test0Spec>;
#[doc = "4:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atest0Muxsel {
    #[doc = "16: Internal. Only to be used through TI provided API."]
    Val16 = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Val8 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Val4 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Val2 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Val1 = 1,
}
impl From<Atest0Muxsel> for u8 {
    #[inline(always)]
    fn from(variant: Atest0Muxsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atest0Muxsel {
    type Ux = u8;
}
impl crate::IsEnum for Atest0Muxsel {}
#[doc = "Field `ATEST0_MUXSEL` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type Atest0MuxselR = crate::FieldReader<Atest0Muxsel>;
impl Atest0MuxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Atest0Muxsel> {
        match self.bits {
            16 => Some(Atest0Muxsel::Val16),
            8 => Some(Atest0Muxsel::Val8),
            4 => Some(Atest0Muxsel::Val4),
            2 => Some(Atest0Muxsel::Val2),
            1 => Some(Atest0Muxsel::Val1),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        *self == Atest0Muxsel::Val16
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == Atest0Muxsel::Val8
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == Atest0Muxsel::Val4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Atest0Muxsel::Val2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Atest0Muxsel::Val1
    }
}
#[doc = "Field `ATEST0_MUXSEL` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type Atest0MuxselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Atest0Muxsel>;
impl<'a, REG> Atest0MuxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val16(self) -> &'a mut crate::W<REG> {
        self.variant(Atest0Muxsel::Val16)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val8(self) -> &'a mut crate::W<REG> {
        self.variant(Atest0Muxsel::Val8)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(Atest0Muxsel::Val4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Atest0Muxsel::Val2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Atest0Muxsel::Val1)
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "12:8\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atest1Muxsel {
    #[doc = "16: Internal. Only to be used through TI provided API."]
    Val16 = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    Val8 = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    Val4 = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    Val2 = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Val1 = 1,
}
impl From<Atest1Muxsel> for u8 {
    #[inline(always)]
    fn from(variant: Atest1Muxsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atest1Muxsel {
    type Ux = u8;
}
impl crate::IsEnum for Atest1Muxsel {}
#[doc = "Field `ATEST1_MUXSEL` reader - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type Atest1MuxselR = crate::FieldReader<Atest1Muxsel>;
impl Atest1MuxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Atest1Muxsel> {
        match self.bits {
            16 => Some(Atest1Muxsel::Val16),
            8 => Some(Atest1Muxsel::Val8),
            4 => Some(Atest1Muxsel::Val4),
            2 => Some(Atest1Muxsel::Val2),
            1 => Some(Atest1Muxsel::Val1),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        *self == Atest1Muxsel::Val16
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == Atest1Muxsel::Val8
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == Atest1Muxsel::Val4
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == Atest1Muxsel::Val2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == Atest1Muxsel::Val1
    }
}
#[doc = "Field `ATEST1_MUXSEL` writer - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type Atest1MuxselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Atest1Muxsel>;
impl<'a, REG> Atest1MuxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val16(self) -> &'a mut crate::W<REG> {
        self.variant(Atest1Muxsel::Val16)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val8(self) -> &'a mut crate::W<REG> {
        self.variant(Atest1Muxsel::Val8)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(Atest1Muxsel::Val4)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(Atest1Muxsel::Val2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(Atest1Muxsel::Val1)
    }
}
#[doc = "Field `RESERVED13` reader - 28:13\\]
Internal. Only to be used through TI provided API."]
pub type Reserved13R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED13` writer - 28:13\\]
Internal. Only to be used through TI provided API."]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "29:29\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atest1En {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Enable = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Disable = 0,
}
impl From<Atest1En> for bool {
    #[inline(always)]
    fn from(variant: Atest1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATEST1_EN` reader - 29:29\\]
Internal. Only to be used through TI provided API."]
pub type Atest1EnR = crate::BitReader<Atest1En>;
impl Atest1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atest1En {
        match self.bits {
            true => Atest1En::Enable,
            false => Atest1En::Disable,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Atest1En::Enable
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Atest1En::Disable
    }
}
#[doc = "Field `ATEST1_EN` writer - 29:29\\]
Internal. Only to be used through TI provided API."]
pub type Atest1EnW<'a, REG> = crate::BitWriter<'a, REG, Atest1En>;
impl<'a, REG> Atest1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Atest1En::Enable)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Atest1En::Disable)
    }
}
#[doc = "30:30\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atest0En {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Enable = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Disable = 0,
}
impl From<Atest0En> for bool {
    #[inline(always)]
    fn from(variant: Atest0En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATEST0_EN` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type Atest0EnR = crate::BitReader<Atest0En>;
impl Atest0EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Atest0En {
        match self.bits {
            true => Atest0En::Enable,
            false => Atest0En::Disable,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Atest0En::Enable
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Atest0En::Disable
    }
}
#[doc = "Field `ATEST0_EN` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type Atest0EnW<'a, REG> = crate::BitWriter<'a, REG, Atest0En>;
impl<'a, REG> Atest0EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Atest0En::Enable)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Atest0En::Disable)
    }
}
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest0_muxsel(&self) -> Atest0MuxselR {
        Atest0MuxselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest1_muxsel(&self) -> Atest1MuxselR {
        Atest1MuxselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:28 - 28:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest1_en(&self) -> Atest1EnR {
        Atest1EnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn atest0_en(&self) -> Atest0EnR {
        Atest0EnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atest0_muxsel(&mut self) -> Atest0MuxselW<Test0Spec> {
        Atest0MuxselW::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<Test0Spec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atest1_muxsel(&mut self) -> Atest1MuxselW<Test0Spec> {
        Atest1MuxselW::new(self, 8)
    }
    #[doc = "Bits 13:28 - 28:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<Test0Spec> {
        Reserved13W::new(self, 13)
    }
    #[doc = "Bit 29 - 29:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atest1_en(&mut self) -> Atest1EnW<Test0Spec> {
        Atest1EnW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn atest0_en(&mut self) -> Atest0EnW<Test0Spec> {
        Atest0EnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<Test0Spec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Test0Spec;
impl crate::RegisterSpec for Test0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test0::R`](R) reader structure"]
impl crate::Readable for Test0Spec {}
#[doc = "`write(|w| ..)` method takes [`test0::W`](W) writer structure"]
impl crate::Writable for Test0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST0 to value 0"]
impl crate::Resettable for Test0Spec {
    const RESET_VALUE: u32 = 0;
}
