#[doc = "Register `MIMR` reader"]
pub type R = crate::R<MimrSpec>;
#[doc = "Register `MIMR` writer"]
pub type W = crate::W<MimrSpec>;
#[doc = "0:0\\]
Interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Im {
    #[doc = "1: ENA Interrupt mask"]
    Ena = 1,
    #[doc = "0: Disable Interrupt mask"]
    Dis = 0,
}
impl From<Im> for bool {
    #[inline(always)]
    fn from(variant: Im) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IM` reader - 0:0\\]
Interrupt mask"]
pub type ImR = crate::BitReader<Im>;
impl ImR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Im {
        match self.bits {
            true => Im::Ena,
            false => Im::Dis,
        }
    }
    #[doc = "ENA Interrupt mask"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Im::Ena
    }
    #[doc = "Disable Interrupt mask"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Im::Dis
    }
}
#[doc = "Field `IM` writer - 0:0\\]
Interrupt mask"]
pub type ImW<'a, REG> = crate::BitWriter<'a, REG, Im>;
impl<'a, REG> ImW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ENA Interrupt mask"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Im::Ena)
    }
    #[doc = "Disable Interrupt mask"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Im::Dis)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Reads to this field return zero.Writes to this field are ignored."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt mask"]
    #[inline(always)]
    pub fn im(&self) -> ImR {
        ImR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reads to this field return zero.Writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> ImW<MimrSpec> {
        ImW::new(self, 0)
    }
}
#[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MimrSpec;
impl crate::RegisterSpec for MimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mimr::R`](R) reader structure"]
impl crate::Readable for MimrSpec {}
#[doc = "`write(|w| ..)` method takes [`mimr::W`](W) writer structure"]
impl crate::Writable for MimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIMR to value 0"]
impl crate::Resettable for MimrSpec {
    const RESET_VALUE: u32 = 0;
}
