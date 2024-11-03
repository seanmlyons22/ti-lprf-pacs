#[doc = "Register `DTB` reader"]
pub type R = crate::R<DtbSpec>;
#[doc = "Register `DTB` writer"]
pub type W = crate::W<DtbSpec>;
#[doc = "0:0\\]
Digital test bus selection mux control. Non-zero select values output a 16 bit selected group of signals per value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "0: All 16 observation signals are set to zero."]
    Dis = 0,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - 0:0\\]
Digital test bus selection mux control. Non-zero select values output a 16 bit selected group of signals per value."]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            false => Some(Sel::Dis),
            _ => None,
        }
    }
    #[doc = "All 16 observation signals are set to zero."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Sel::Dis
    }
}
#[doc = "Field `SEL` writer - 0:0\\]
Digital test bus selection mux control. Non-zero select values output a 16 bit selected group of signals per value."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All 16 observation signals are set to zero."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Dis)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Digital test bus selection mux control. Non-zero select values output a 16 bit selected group of signals per value."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Digital test bus selection mux control. Non-zero select values output a 16 bit selected group of signals per value."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<DtbSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DtbSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtbSpec;
impl crate::RegisterSpec for DtbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtb::R`](R) reader structure"]
impl crate::Readable for DtbSpec {}
#[doc = "`write(|w| ..)` method takes [`dtb::W`](W) writer structure"]
impl crate::Writable for DtbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTB to value 0"]
impl crate::Resettable for DtbSpec {
    const RESET_VALUE: u32 = 0;
}
