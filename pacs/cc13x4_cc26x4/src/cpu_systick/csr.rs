#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Indicates the enabled status of the SysTick counter"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Indicates the enabled status of the SysTick counter"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICKINT` reader - 1:1\\]
Indicates whether counting to 0 causes the status of the SysTick exception to change to pending"]
pub type TickintR = crate::BitReader;
#[doc = "Field `TICKINT` writer - 1:1\\]
Indicates whether counting to 0 causes the status of the SysTick exception to change to pending"]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSOURCE` reader - 2:2\\]
Indicates the SysTick clock source"]
pub type ClksourceR = crate::BitReader;
#[doc = "Field `CLKSOURCE` writer - 2:2\\]
Indicates the SysTick clock source"]
pub type ClksourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED3` writer - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `COUNTFLAG` reader - 16:16\\]
Indicates whether the counter has counted to zero since the last read of this register"]
pub type CountflagR = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - 16:16\\]
Indicates whether the counter has counted to zero since the last read of this register"]
pub type CountflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the enabled status of the SysTick counter"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether counting to 0 causes the status of the SysTick exception to change to pending"]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates the SysTick clock source"]
    #[inline(always)]
    pub fn clksource(&self) -> ClksourceR {
        ClksourceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates whether the counter has counted to zero since the last read of this register"]
    #[inline(always)]
    pub fn countflag(&self) -> CountflagR {
        CountflagR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the enabled status of the SysTick counter"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates whether counting to 0 causes the status of the SysTick exception to change to pending"]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TickintW<CsrSpec> {
        TickintW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates the SysTick clock source"]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> ClksourceW<CsrSpec> {
        ClksourceW::new(self, 2)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CsrSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates whether the counter has counted to zero since the last read of this register"]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> CountflagW<CsrSpec> {
        CountflagW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<CsrSpec> {
        Reserved17W::new(self, 17)
    }
}
#[doc = "Controls the SysTick timer and provides status data `FTSSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}
