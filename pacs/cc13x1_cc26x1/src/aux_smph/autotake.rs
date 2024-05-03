#[doc = "Register `AUTOTAKE` reader"]
pub type R = crate::R<AutotakeSpec>;
#[doc = "Register `AUTOTAKE` writer"]
pub type W = crate::W<AutotakeSpec>;
#[doc = "Field `SMPH_ID` reader - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
pub type SmphIdR = crate::FieldReader;
#[doc = "Field `SMPH_ID` writer - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
pub type SmphIdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
    #[inline(always)]
    pub fn smph_id(&self) -> SmphIdR {
        SmphIdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT3.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
    #[inline(always)]
    #[must_use]
    pub fn smph_id(&mut self) -> SmphIdW<AutotakeSpec> {
        SmphIdW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<AutotakeSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Auto Take Sticky Request for Single Semaphore.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autotake::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autotake::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutotakeSpec;
impl crate::RegisterSpec for AutotakeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autotake::R`](R) reader structure"]
impl crate::Readable for AutotakeSpec {}
#[doc = "`write(|w| ..)` method takes [`autotake::W`](W) writer structure"]
impl crate::Writable for AutotakeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOTAKE to value 0"]
impl crate::Resettable for AutotakeSpec {
    const RESET_VALUE: u32 = 0;
}
