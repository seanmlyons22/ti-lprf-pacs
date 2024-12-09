#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Enable SysTick counter 0x0:Counter disabled 0x1:Counter operates in a multi-shot way. that is, counter loads with the reload value and then begins counting down. on reaching 0, it sets the countflag to 1 and optionally pends the systick handler, based on tickint. it then loads the reload value again, and begins counting."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enable SysTick counter 0x0:Counter disabled 0x1:Counter operates in a multi-shot way. that is, counter loads with the reload value and then begins counting down. on reaching 0, it sets the countflag to 1 and optionally pends the systick handler, based on tickint. it then loads the reload value again, and begins counting."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICKINT` reader - 1:1\\]
0x0:Counting down to zero does not pend the systick handler. software can use countflag to determine if the systick handler has ever counted to zero. 0x1:Counting down to zero pends the systick handler."]
pub type TickintR = crate::BitReader;
#[doc = "Field `TICKINT` writer - 1:1\\]
0x0:Counting down to zero does not pend the systick handler. software can use countflag to determine if the systick handler has ever counted to zero. 0x1:Counting down to zero pends the systick handler."]
pub type TickintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSOURCE` reader - 2:2\\]
SysTick clock source. Always reads as one if STCALIB reports NOREF. 0x0:Systick driven by external reference clock. 0x1:Systick driven by processor clock, hclk."]
pub type ClksourceR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 15:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `COUNTFLAG` reader - 16:16\\]
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
pub type CountflagR = crate::BitReader;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable SysTick counter 0x0:Counter disabled 0x1:Counter operates in a multi-shot way. that is, counter loads with the reload value and then begins counting down. on reaching 0, it sets the countflag to 1 and optionally pends the systick handler, based on tickint. it then loads the reload value again, and begins counting."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0x0:Counting down to zero does not pend the systick handler. software can use countflag to determine if the systick handler has ever counted to zero. 0x1:Counting down to zero pends the systick handler."]
    #[inline(always)]
    pub fn tickint(&self) -> TickintR {
        TickintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SysTick clock source. Always reads as one if STCALIB reports NOREF. 0x0:Systick driven by external reference clock. 0x1:Systick driven by processor clock, hclk."]
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
Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
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
Enable SysTick counter 0x0:Counter disabled 0x1:Counter operates in a multi-shot way. that is, counter loads with the reload value and then begins counting down. on reaching 0, it sets the countflag to 1 and optionally pends the systick handler, based on tickint. it then loads the reload value again, and begins counting."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CsrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0x0:Counting down to zero does not pend the systick handler. software can use countflag to determine if the systick handler has ever counted to zero. 0x1:Counting down to zero pends the systick handler."]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TickintW<CsrSpec> {
        TickintW::new(self, 1)
    }
}
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
