#[doc = "Register `GBLINFO2` reader"]
pub type R = crate::R<Gblinfo2Spec>;
#[doc = "Register `GBLINFO2` writer"]
pub type W = crate::W<Gblinfo2Spec>;
#[doc = "3:0\\]
Number of data registers present.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dataregisters {
    #[doc = "8: Maximum value of DATAREGISTERS"]
    Maximum = 8,
    #[doc = "1: Minimum value of DATAREGISTERS"]
    Minimum = 1,
}
impl From<Dataregisters> for u8 {
    #[inline(always)]
    fn from(variant: Dataregisters) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dataregisters {
    type Ux = u8;
}
impl crate::IsEnum for Dataregisters {}
#[doc = "Field `DATAREGISTERS` reader - 3:0\\]
Number of data registers present."]
pub type DataregistersR = crate::FieldReader<Dataregisters>;
impl DataregistersR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dataregisters> {
        match self.bits {
            8 => Some(Dataregisters::Maximum),
            1 => Some(Dataregisters::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value of DATAREGISTERS"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Dataregisters::Maximum
    }
    #[doc = "Minimum value of DATAREGISTERS"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Dataregisters::Minimum
    }
}
#[doc = "Field `RESERVED_31_4` reader - 31:4\\]
Reserved"]
pub type Reserved31_4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of data registers present."]
    #[inline(always)]
    pub fn dataregisters(&self) -> DataregistersR {
        DataregistersR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_31_4(&self) -> Reserved31_4R {
        Reserved31_4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {}
#[doc = "Global Info 2 Register Read only register detailing information about the number of data registers present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gblinfo2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gblinfo2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gblinfo2Spec;
impl crate::RegisterSpec for Gblinfo2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gblinfo2::R`](R) reader structure"]
impl crate::Readable for Gblinfo2Spec {}
#[doc = "`write(|w| ..)` method takes [`gblinfo2::W`](W) writer structure"]
impl crate::Writable for Gblinfo2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GBLINFO2 to value 0x01"]
impl crate::Resettable for Gblinfo2Spec {
    const RESET_VALUE: u32 = 0x01;
}
