#[doc = "Register `COMP3` reader"]
pub type R = crate::R<Comp3Spec>;
#[doc = "Register `COMP3` writer"]
pub type W = crate::W<Comp3Spec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - 28:2\\]
Comparison address."]
pub type CompR = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - 28:2\\]
Comparison address."]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `RESERVED29` reader - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::BitReader;
#[doc = "Field `RESERVED29` writer - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPLACE` reader - 31:30\\]
This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
pub type ReplaceR = crate::FieldReader;
#[doc = "Field `REPLACE` writer - 31:30\\]
This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
pub type ReplaceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Comparison address."]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new((self.bits >> 2) & 0x07ff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub fn replace(&self) -> ReplaceR {
        ReplaceR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Comp3Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Comp3Spec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Comparison address."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<Comp3Spec> {
        CompW::new(self, 2)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<Comp3Spec> {
        Reserved29W::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    #[must_use]
    pub fn replace(&mut self) -> ReplaceW<Comp3Spec> {
        ReplaceW::new(self, 30)
    }
}
#[doc = "Comparator 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp3Spec;
impl crate::RegisterSpec for Comp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp3::R`](R) reader structure"]
impl crate::Readable for Comp3Spec {}
#[doc = "`write(|w| ..)` method takes [`comp3::W`](W) writer structure"]
impl crate::Writable for Comp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP3 to value 0"]
impl crate::Resettable for Comp3Spec {
    const RESET_VALUE: u32 = 0;
}
