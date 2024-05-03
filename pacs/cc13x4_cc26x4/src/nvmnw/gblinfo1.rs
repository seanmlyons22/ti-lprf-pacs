#[doc = "Register `GBLINFO1` reader"]
pub type R = crate::R<Gblinfo1Spec>;
#[doc = "Register `GBLINFO1` writer"]
pub type W = crate::W<Gblinfo1Spec>;
#[doc = "7:0\\]
Data width in bits\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datawidth {
    #[doc = "128: Data width is 128 bits"]
    W128bit = 128,
    #[doc = "64: Data width is 64 bits"]
    W64bit = 64,
}
impl From<Datawidth> for u8 {
    #[inline(always)]
    fn from(variant: Datawidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datawidth {
    type Ux = u8;
}
impl crate::IsEnum for Datawidth {}
#[doc = "Field `DATAWIDTH` reader - 7:0\\]
Data width in bits"]
pub type DatawidthR = crate::FieldReader<Datawidth>;
impl DatawidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datawidth> {
        match self.bits {
            128 => Some(Datawidth::W128bit),
            64 => Some(Datawidth::W64bit),
            _ => None,
        }
    }
    #[doc = "Data width is 128 bits"]
    #[inline(always)]
    pub fn is_w128bit(&self) -> bool {
        *self == Datawidth::W128bit
    }
    #[doc = "Data width is 64 bits"]
    #[inline(always)]
    pub fn is_w64bit(&self) -> bool {
        *self == Datawidth::W64bit
    }
}
#[doc = "Field `DATAWIDTH` writer - 7:0\\]
Data width in bits"]
pub type DatawidthW<'a, REG> = crate::FieldWriter<'a, REG, 8, Datawidth>;
impl<'a, REG> DatawidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data width is 128 bits"]
    #[inline(always)]
    pub fn w128bit(self) -> &'a mut crate::W<REG> {
        self.variant(Datawidth::W128bit)
    }
    #[doc = "Data width is 64 bits"]
    #[inline(always)]
    pub fn w64bit(self) -> &'a mut crate::W<REG> {
        self.variant(Datawidth::W64bit)
    }
}
#[doc = "12:8\\]
ECC data width in bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eccwidth {
    #[doc = "16: ECC data width is 16 bits"]
    W16bit = 16,
    #[doc = "8: ECC data width is 8 bits"]
    W8bit = 8,
    #[doc = "0: ECC data width is 0. ECC not used."]
    W0bit = 0,
}
impl From<Eccwidth> for u8 {
    #[inline(always)]
    fn from(variant: Eccwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eccwidth {
    type Ux = u8;
}
impl crate::IsEnum for Eccwidth {}
#[doc = "Field `ECCWIDTH` reader - 12:8\\]
ECC data width in bits"]
pub type EccwidthR = crate::FieldReader<Eccwidth>;
impl EccwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eccwidth> {
        match self.bits {
            16 => Some(Eccwidth::W16bit),
            8 => Some(Eccwidth::W8bit),
            0 => Some(Eccwidth::W0bit),
            _ => None,
        }
    }
    #[doc = "ECC data width is 16 bits"]
    #[inline(always)]
    pub fn is_w16bit(&self) -> bool {
        *self == Eccwidth::W16bit
    }
    #[doc = "ECC data width is 8 bits"]
    #[inline(always)]
    pub fn is_w8bit(&self) -> bool {
        *self == Eccwidth::W8bit
    }
    #[doc = "ECC data width is 0. ECC not used."]
    #[inline(always)]
    pub fn is_w0bit(&self) -> bool {
        *self == Eccwidth::W0bit
    }
}
#[doc = "Field `ECCWIDTH` writer - 12:8\\]
ECC data width in bits"]
pub type EccwidthW<'a, REG> = crate::FieldWriter<'a, REG, 5, Eccwidth>;
impl<'a, REG> EccwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECC data width is 16 bits"]
    #[inline(always)]
    pub fn w16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Eccwidth::W16bit)
    }
    #[doc = "ECC data width is 8 bits"]
    #[inline(always)]
    pub fn w8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Eccwidth::W8bit)
    }
    #[doc = "ECC data width is 0. ECC not used."]
    #[inline(always)]
    pub fn w0bit(self) -> &'a mut crate::W<REG> {
        self.variant(Eccwidth::W0bit)
    }
}
#[doc = "18:16\\]
Redundant data width in bits\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Redwidth {
    #[doc = "4: Redundant data width is 4 bits"]
    W4bit = 4,
    #[doc = "2: Redundant data width is 2 bits"]
    W2bit = 2,
    #[doc = "0: Redundant data width is 0. Redundancy/Repair not present."]
    W0bit = 0,
}
impl From<Redwidth> for u8 {
    #[inline(always)]
    fn from(variant: Redwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Redwidth {
    type Ux = u8;
}
impl crate::IsEnum for Redwidth {}
#[doc = "Field `REDWIDTH` reader - 18:16\\]
Redundant data width in bits"]
pub type RedwidthR = crate::FieldReader<Redwidth>;
impl RedwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Redwidth> {
        match self.bits {
            4 => Some(Redwidth::W4bit),
            2 => Some(Redwidth::W2bit),
            0 => Some(Redwidth::W0bit),
            _ => None,
        }
    }
    #[doc = "Redundant data width is 4 bits"]
    #[inline(always)]
    pub fn is_w4bit(&self) -> bool {
        *self == Redwidth::W4bit
    }
    #[doc = "Redundant data width is 2 bits"]
    #[inline(always)]
    pub fn is_w2bit(&self) -> bool {
        *self == Redwidth::W2bit
    }
    #[doc = "Redundant data width is 0. Redundancy/Repair not present."]
    #[inline(always)]
    pub fn is_w0bit(&self) -> bool {
        *self == Redwidth::W0bit
    }
}
#[doc = "Field `REDWIDTH` writer - 18:16\\]
Redundant data width in bits"]
pub type RedwidthW<'a, REG> = crate::FieldWriter<'a, REG, 3, Redwidth>;
impl<'a, REG> RedwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Redundant data width is 4 bits"]
    #[inline(always)]
    pub fn w4bit(self) -> &'a mut crate::W<REG> {
        self.variant(Redwidth::W4bit)
    }
    #[doc = "Redundant data width is 2 bits"]
    #[inline(always)]
    pub fn w2bit(self) -> &'a mut crate::W<REG> {
        self.variant(Redwidth::W2bit)
    }
    #[doc = "Redundant data width is 0. Redundancy/Repair not present."]
    #[inline(always)]
    pub fn w0bit(self) -> &'a mut crate::W<REG> {
        self.variant(Redwidth::W0bit)
    }
}
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Data width in bits"]
    #[inline(always)]
    pub fn datawidth(&self) -> DatawidthR {
        DatawidthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
ECC data width in bits"]
    #[inline(always)]
    pub fn eccwidth(&self) -> EccwidthR {
        EccwidthR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Redundant data width in bits"]
    #[inline(always)]
    pub fn redwidth(&self) -> RedwidthR {
        RedwidthR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Data width in bits"]
    #[inline(always)]
    #[must_use]
    pub fn datawidth(&mut self) -> DatawidthW<Gblinfo1Spec> {
        DatawidthW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
ECC data width in bits"]
    #[inline(always)]
    #[must_use]
    pub fn eccwidth(&mut self) -> EccwidthW<Gblinfo1Spec> {
        EccwidthW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Redundant data width in bits"]
    #[inline(always)]
    #[must_use]
    pub fn redwidth(&mut self) -> RedwidthW<Gblinfo1Spec> {
        RedwidthW::new(self, 16)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<Gblinfo1Spec> {
        Reserved19W::new(self, 19)
    }
}
#[doc = "Global Info 1 Register Read only register detailing information about data, ecc and redundant data widths in bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gblinfo1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gblinfo1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gblinfo1Spec;
impl crate::RegisterSpec for Gblinfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gblinfo1::R`](R) reader structure"]
impl crate::Readable for Gblinfo1Spec {}
#[doc = "`write(|w| ..)` method takes [`gblinfo1::W`](W) writer structure"]
impl crate::Writable for Gblinfo1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GBLINFO1 to value 0x0004_0080"]
impl crate::Resettable for Gblinfo1Spec {
    const RESET_VALUE: u32 = 0x0004_0080;
}
