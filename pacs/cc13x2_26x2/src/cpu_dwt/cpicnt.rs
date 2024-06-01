#[doc = "Register `CPICNT` reader"]
pub type R = crate::R<CpicntSpec>;
#[doc = "Register `CPICNT` writer"]
pub type W = crate::W<CpicntSpec>;
#[doc = "Field `CPICNT` reader - 7:0\\]
Current CPI counter value. Increments on the additional cycles (the first cycle is not counted) required to execute all instructions except those recorded by LSUCNT. This counter also increments on all instruction fetch stalls. If CTRL.CPIEVTENA is set, an event is emitted when the counter overflows. This counter initializes to 0 when it is enabled using CTRL.CPIEVTENA."]
pub type CpicntR = crate::FieldReader;
#[doc = "Field `CPICNT` writer - 7:0\\]
Current CPI counter value. Increments on the additional cycles (the first cycle is not counted) required to execute all instructions except those recorded by LSUCNT. This counter also increments on all instruction fetch stalls. If CTRL.CPIEVTENA is set, an event is emitted when the counter overflows. This counter initializes to 0 when it is enabled using CTRL.CPIEVTENA."]
pub type CpicntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Current CPI counter value. Increments on the additional cycles (the first cycle is not counted) required to execute all instructions except those recorded by LSUCNT. This counter also increments on all instruction fetch stalls. If CTRL.CPIEVTENA is set, an event is emitted when the counter overflows. This counter initializes to 0 when it is enabled using CTRL.CPIEVTENA."]
    #[inline(always)]
    pub fn cpicnt(&self) -> CpicntR {
        CpicntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Current CPI counter value. Increments on the additional cycles (the first cycle is not counted) required to execute all instructions except those recorded by LSUCNT. This counter also increments on all instruction fetch stalls. If CTRL.CPIEVTENA is set, an event is emitted when the counter overflows. This counter initializes to 0 when it is enabled using CTRL.CPIEVTENA."]
    #[inline(always)]
    #[must_use]
    pub fn cpicnt(&mut self) -> CpicntW<CpicntSpec> {
        CpicntW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<CpicntSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpicnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpicnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpicntSpec;
impl crate::RegisterSpec for CpicntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpicnt::R`](R) reader structure"]
impl crate::Readable for CpicntSpec {}
#[doc = "`write(|w| ..)` method takes [`cpicnt::W`](W) writer structure"]
impl crate::Writable for CpicntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPICNT to value 0"]
impl crate::Resettable for CpicntSpec {
    const RESET_VALUE: u32 = 0;
}
