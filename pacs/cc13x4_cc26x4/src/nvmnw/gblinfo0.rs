#[doc = "Register `GBLINFO0` reader"]
pub type R = crate::R<Gblinfo0Spec>;
#[doc = "Register `GBLINFO0` writer"]
pub type W = crate::W<Gblinfo0Spec>;
#[doc = "15:0\\]
Sector size in bytes\n\nValue on reset: 2048"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Sectorsize {
    #[doc = "2048: Sector size is TWOKB"]
    Twokb = 2048,
    #[doc = "1024: Sector size is ONEKB"]
    Onekb = 1024,
}
impl From<Sectorsize> for u16 {
    #[inline(always)]
    fn from(variant: Sectorsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sectorsize {
    type Ux = u16;
}
impl crate::IsEnum for Sectorsize {}
#[doc = "Field `SECTORSIZE` reader - 15:0\\]
Sector size in bytes"]
pub type SectorsizeR = crate::FieldReader<Sectorsize>;
impl SectorsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sectorsize> {
        match self.bits {
            2048 => Some(Sectorsize::Twokb),
            1024 => Some(Sectorsize::Onekb),
            _ => None,
        }
    }
    #[doc = "Sector size is TWOKB"]
    #[inline(always)]
    pub fn is_twokb(&self) -> bool {
        *self == Sectorsize::Twokb
    }
    #[doc = "Sector size is ONEKB"]
    #[inline(always)]
    pub fn is_onekb(&self) -> bool {
        *self == Sectorsize::Onekb
    }
}
#[doc = "Field `SECTORSIZE` writer - 15:0\\]
Sector size in bytes"]
pub type SectorsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, Sectorsize>;
impl<'a, REG> SectorsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Sector size is TWOKB"]
    #[inline(always)]
    pub fn twokb(self) -> &'a mut crate::W<REG> {
        self.variant(Sectorsize::Twokb)
    }
    #[doc = "Sector size is ONEKB"]
    #[inline(always)]
    pub fn onekb(self) -> &'a mut crate::W<REG> {
        self.variant(Sectorsize::Onekb)
    }
}
#[doc = "18:16\\]
Number of banks instantiated Minimum:1 Maximum:5\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numbanks {
    #[doc = "5: Maximum value"]
    Maximum = 5,
    #[doc = "1: Minimum value"]
    Minimum = 1,
}
impl From<Numbanks> for u8 {
    #[inline(always)]
    fn from(variant: Numbanks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Numbanks {
    type Ux = u8;
}
impl crate::IsEnum for Numbanks {}
#[doc = "Field `NUMBANKS` reader - 18:16\\]
Number of banks instantiated Minimum:1 Maximum:5"]
pub type NumbanksR = crate::FieldReader<Numbanks>;
impl NumbanksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Numbanks> {
        match self.bits {
            5 => Some(Numbanks::Maximum),
            1 => Some(Numbanks::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Numbanks::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Numbanks::Minimum
    }
}
#[doc = "Field `NUMBANKS` writer - 18:16\\]
Number of banks instantiated Minimum:1 Maximum:5"]
pub type NumbanksW<'a, REG> = crate::FieldWriter<'a, REG, 3, Numbanks>;
impl<'a, REG> NumbanksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Numbanks::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Numbanks::Minimum)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Sector size in bytes"]
    #[inline(always)]
    pub fn sectorsize(&self) -> SectorsizeR {
        SectorsizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Number of banks instantiated Minimum:1 Maximum:5"]
    #[inline(always)]
    pub fn numbanks(&self) -> NumbanksR {
        NumbanksR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Sector size in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn sectorsize(&mut self) -> SectorsizeW<Gblinfo0Spec> {
        SectorsizeW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Number of banks instantiated Minimum:1 Maximum:5"]
    #[inline(always)]
    #[must_use]
    pub fn numbanks(&mut self) -> NumbanksW<Gblinfo0Spec> {
        NumbanksW::new(self, 16)
    }
}
#[doc = "Global Info 0 Register Read only register detailing information about sector size and number of banks present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gblinfo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gblinfo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gblinfo0Spec;
impl crate::RegisterSpec for Gblinfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gblinfo0::R`](R) reader structure"]
impl crate::Readable for Gblinfo0Spec {}
#[doc = "`write(|w| ..)` method takes [`gblinfo0::W`](W) writer structure"]
impl crate::Writable for Gblinfo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GBLINFO0 to value 0x0002_0800"]
impl crate::Resettable for Gblinfo0Spec {
    const RESET_VALUE: u32 = 0x0002_0800;
}
