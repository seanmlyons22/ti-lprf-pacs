#[doc = "Register `ICSR` reader"]
pub struct R(crate::R<ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSR` writer"]
pub struct W(crate::W<ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTACTIVE` reader - 8:0\\]
Active ISR number field. Reset clears this field."]
pub type VECTACTIVE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VECTACTIVE` writer - 8:0\\]
Active ISR number field. Reset clears this field."]
pub type VECTACTIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u16, u16, 9, O>;
#[doc = "Field `RESERVED9` reader - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RETTOBASE` reader - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
pub type RETTOBASE_R = crate::BitReader<bool>;
#[doc = "Field `RETTOBASE` writer - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
pub type RETTOBASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `VECTPENDING` reader - 17:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
pub type VECTPENDING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VECTPENDING` writer - 17:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
pub type VECTPENDING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED18` reader - 21:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED18` writer - 21:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ISRPENDING` reader - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
pub type ISRPENDING_R = crate::BitReader<bool>;
#[doc = "Field `ISRPENDING` writer - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
pub type ISRPENDING_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `ISRPREEMPT` reader - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
pub type ISRPREEMPT_R = crate::BitReader<bool>;
#[doc = "Field `ISRPREEMPT` writer - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
pub type ISRPREEMPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `RESERVED24` reader - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED24` writer - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `PENDSTCLR` reader - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
pub type PENDSTCLR_R = crate::BitReader<bool>;
#[doc = "Field `PENDSTCLR` writer - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
pub type PENDSTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `PENDSTSET` reader - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
pub type PENDSTSET_R = crate::BitReader<bool>;
#[doc = "Field `PENDSTSET` writer - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
pub type PENDSTSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `PENDSVCLR` reader - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
pub type PENDSVCLR_R = crate::BitReader<bool>;
#[doc = "Field `PENDSVCLR` writer - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
pub type PENDSVCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `PENDSVSET` reader - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
pub type PENDSVSET_R = crate::BitReader<bool>;
#[doc = "Field `PENDSVSET` writer - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
pub type PENDSVSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
#[doc = "Field `RESERVED29` reader - 30:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED29` writer - 30:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED29_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NMIPENDSET` reader - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
pub type NMIPENDSET_R = crate::BitReader<bool>;
#[doc = "Field `NMIPENDSET` writer - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
pub type NMIPENDSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICSR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Active ISR number field. Reset clears this field."]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
    #[inline(always)]
    pub fn pendstclr(&self) -> PENDSTCLR_R {
        PENDSTCLR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PENDSVCLR_R {
        PENDSVCLR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Active ISR number field. Reset clears this field."]
    #[inline(always)]
    #[must_use]
    pub fn vectactive(&mut self) -> VECTACTIVE_W<0> {
        VECTACTIVE_W::new(self)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
    #[inline(always)]
    #[must_use]
    pub fn rettobase(&mut self) -> RETTOBASE_W<11> {
        RETTOBASE_W::new(self)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
    #[inline(always)]
    #[must_use]
    pub fn vectpending(&mut self) -> VECTPENDING_W<12> {
        VECTPENDING_W::new(self)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> RESERVED18_W<18> {
        RESERVED18_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn isrpending(&mut self) -> ISRPENDING_W<22> {
        ISRPENDING_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
    #[inline(always)]
    #[must_use]
    pub fn isrpreempt(&mut self) -> ISRPREEMPT_W<23> {
        ISRPREEMPT_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
    #[inline(always)]
    #[must_use]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W<25> {
        PENDSTCLR_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
    #[inline(always)]
    #[must_use]
    pub fn pendstset(&mut self) -> PENDSTSET_W<26> {
        PENDSTSET_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W<27> {
        PENDSVCLR_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvset(&mut self) -> PENDSVSET_W<28> {
        PENDSVSET_W::new(self)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> RESERVED29_W<29> {
        RESERVED29_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
    #[inline(always)]
    #[must_use]
    pub fn nmipendset(&mut self) -> NMIPENDSET_W<31> {
        NMIPENDSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icsr](index.html) module"]
pub struct ICSR_SPEC;
impl crate::RegisterSpec for ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icsr::R](R) reader structure"]
impl crate::Readable for ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icsr::W](W) writer structure"]
impl crate::Writable for ICSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for ICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
