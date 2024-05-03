#[doc = "Register `RLAR` reader"]
pub type R = crate::R<RlarSpec>;
#[doc = "Register `RLAR` writer"]
pub type W = crate::W<RlarSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
SAU region enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
SAU region enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSC` reader - 1:1\\]
Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
pub type NscR = crate::BitReader;
#[doc = "Field `NSC` writer - 1:1\\]
Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
pub type NscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 4:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 4:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LADDR` reader - 31:5\\]
Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
pub type LaddrR = crate::FieldReader<u32>;
#[doc = "Field `LADDR` writer - 31:5\\]
Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
pub type LaddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SAU region enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
    #[inline(always)]
    pub fn nsc(&self) -> NscR {
        NscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
    #[inline(always)]
    pub fn laddr(&self) -> LaddrR {
        LaddrR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SAU region enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<RlarSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
    #[inline(always)]
    #[must_use]
    pub fn nsc(&mut self) -> NscW<RlarSpec> {
        NscW::new(self, 1)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<RlarSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
    #[inline(always)]
    #[must_use]
    pub fn laddr(&mut self) -> LaddrW<RlarSpec> {
        LaddrW::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the limit address of the currently selected SAU region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RlarSpec;
impl crate::RegisterSpec for RlarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlar::R`](R) reader structure"]
impl crate::Readable for RlarSpec {}
#[doc = "`write(|w| ..)` method takes [`rlar::W`](W) writer structure"]
impl crate::Writable for RlarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLAR to value 0"]
impl crate::Resettable for RlarSpec {
    const RESET_VALUE: u32 = 0;
}
