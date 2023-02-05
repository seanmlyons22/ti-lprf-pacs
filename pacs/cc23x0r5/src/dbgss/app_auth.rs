#[doc = "Register `APP_AUTH` reader"]
pub type R = crate::R<AppAuthSpec>;
#[doc = "Register `APP_AUTH` writer"]
pub type W = crate::W<AppAuthSpec>;
#[doc = "0:0\\]
Controls invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "1: Invasive debug enabled"]
    En = 1,
    #[doc = "0: Invasive debug disabled"]
    Dis = 0,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - 0:0\\]
Controls invasive debug enable."]
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            true => Dbgen::En,
            false => Dbgen::Dis,
        }
    }
    #[doc = "Invasive debug enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgen::En
    }
    #[doc = "Invasive debug disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgen::Dis
    }
}
#[doc = "1:1\\]
Controls non-invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Niden {
    #[doc = "1: Non-invasive debug enabled"]
    En = 1,
    #[doc = "0: Non-invasive debug disabled"]
    Dis = 0,
}
impl From<Niden> for bool {
    #[inline(always)]
    fn from(variant: Niden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN` reader - 1:1\\]
Controls non-invasive debug enable."]
pub type NidenR = crate::BitReader<Niden>;
impl NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Niden {
        match self.bits {
            true => Niden::En,
            false => Niden::Dis,
        }
    }
    #[doc = "Non-invasive debug enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Niden::En
    }
    #[doc = "Non-invasive debug disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Niden::Dis
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls invasive debug enable."]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls non-invasive debug enable."]
    #[inline(always)]
    pub fn niden(&self) -> NidenR {
        NidenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {}
#[doc = "This register indicates the debug privileges of ARM Cortex CPU.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_auth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_auth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppAuthSpec;
impl crate::RegisterSpec for AppAuthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_auth::R`](R) reader structure"]
impl crate::Readable for AppAuthSpec {}
#[doc = "`write(|w| ..)` method takes [`app_auth::W`](W) writer structure"]
impl crate::Writable for AppAuthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_AUTH to value 0"]
impl crate::Resettable for AppAuthSpec {
    const RESET_VALUE: u32 = 0;
}
