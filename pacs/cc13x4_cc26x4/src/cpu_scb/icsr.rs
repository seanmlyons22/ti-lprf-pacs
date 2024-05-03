#[doc = "Register `ICSR` reader"]
pub type R = crate::R<IcsrSpec>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<IcsrSpec>;
#[doc = "Field `VECTACTIVE` reader - 8:0\\]
Active ISR number field. Reset clears this field."]
pub type VectactiveR = crate::FieldReader<u16>;
#[doc = "Field `VECTACTIVE` writer - 8:0\\]
Active ISR number field. Reset clears this field."]
pub type VectactiveW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RESERVED9` reader - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RETTOBASE` reader - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
pub type RettobaseR = crate::BitReader;
#[doc = "Field `RETTOBASE` writer - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
pub type RettobaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VECTPENDING` reader - 20:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
pub type VectpendingR = crate::FieldReader<u16>;
#[doc = "Field `VECTPENDING` writer - 20:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
pub type VectpendingW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RESERVED21` reader - 21:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21R = crate::BitReader;
#[doc = "Field `RESERVED21` writer - 21:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISRPENDING` reader - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
pub type IsrpendingR = crate::BitReader;
#[doc = "Field `ISRPENDING` writer - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
pub type IsrpendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISRPREEMPT` reader - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
pub type IsrpreemptR = crate::BitReader;
#[doc = "Field `ISRPREEMPT` writer - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
pub type IsrpreemptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTNS` reader - 24:24\\]
SysTick Targets Non-secure. Controls whether in a single SysTick implementation, the SysTick is Secure or Non-secure. 0x0 SysTick is Secure. 0x1 SysTick is Non-secure."]
pub type SttnsR = crate::BitReader;
#[doc = "Field `STTNS` writer - 24:24\\]
SysTick Targets Non-secure. Controls whether in a single SysTick implementation, the SysTick is Secure or Non-secure. 0x0 SysTick is Secure. 0x1 SysTick is Non-secure."]
pub type SttnsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSTCLR` reader - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
pub type PendstclrR = crate::BitReader;
#[doc = "Field `PENDSTCLR` writer - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
pub type PendstclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSTSET` reader - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
pub type PendstsetR = crate::BitReader;
#[doc = "Field `PENDSTSET` writer - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
pub type PendstsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSVCLR` reader - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
pub type PendsvclrR = crate::BitReader;
#[doc = "Field `PENDSVCLR` writer - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
pub type PendsvclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDSVSET` reader - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
pub type PendsvsetR = crate::BitReader;
#[doc = "Field `PENDSVSET` writer - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
pub type PendsvsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED29` reader - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::BitReader;
#[doc = "Field `RESERVED29` writer - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENDNMICLR` reader - 30:30\\]
Pend NMI clear. Allows the NMI exception pending state to be cleared. 0x0 No effect. 0x1 Clear pending status."]
pub type PendnmiclrR = crate::BitReader;
#[doc = "Field `PENDNMICLR` writer - 30:30\\]
Pend NMI clear. Allows the NMI exception pending state to be cleared. 0x0 No effect. 0x1 Clear pending status."]
pub type PendnmiclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIPENDSET` reader - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
pub type NmipendsetR = crate::BitReader;
#[doc = "Field `NMIPENDSET` writer - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
pub type NmipendsetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Active ISR number field. Reset clears this field."]
    #[inline(always)]
    pub fn vectactive(&self) -> VectactiveR {
        VectactiveR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
    #[inline(always)]
    pub fn rettobase(&self) -> RettobaseR {
        RettobaseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:20 - 20:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
    #[inline(always)]
    pub fn vectpending(&self) -> VectpendingR {
        VectpendingR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 21 - 21:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
    #[inline(always)]
    pub fn isrpending(&self) -> IsrpendingR {
        IsrpendingR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> IsrpreemptR {
        IsrpreemptR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
SysTick Targets Non-secure. Controls whether in a single SysTick implementation, the SysTick is Secure or Non-secure. 0x0 SysTick is Secure. 0x1 SysTick is Non-secure."]
    #[inline(always)]
    pub fn sttns(&self) -> SttnsR {
        SttnsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
    #[inline(always)]
    pub fn pendstclr(&self) -> PendstclrR {
        PendstclrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
    #[inline(always)]
    pub fn pendstset(&self) -> PendstsetR {
        PendstsetR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
    #[inline(always)]
    pub fn pendsvclr(&self) -> PendsvclrR {
        PendsvclrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PendsvsetR {
        PendsvsetR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Pend NMI clear. Allows the NMI exception pending state to be cleared. 0x0 No effect. 0x1 Clear pending status."]
    #[inline(always)]
    pub fn pendnmiclr(&self) -> PendnmiclrR {
        PendnmiclrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NmipendsetR {
        NmipendsetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Active ISR number field. Reset clears this field."]
    #[inline(always)]
    #[must_use]
    pub fn vectactive(&mut self) -> VectactiveW<IcsrSpec> {
        VectactiveW::new(self, 0)
    }
    #[doc = "Bits 9:10 - 10:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<IcsrSpec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 11 - 11:11\\]
Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
    #[inline(always)]
    #[must_use]
    pub fn rettobase(&mut self) -> RettobaseW<IcsrSpec> {
        RettobaseW::new(self, 11)
    }
    #[doc = "Bits 12:20 - 20:12\\]
Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
    #[inline(always)]
    #[must_use]
    pub fn vectpending(&mut self) -> VectpendingW<IcsrSpec> {
        VectpendingW::new(self, 12)
    }
    #[doc = "Bit 21 - 21:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<IcsrSpec> {
        Reserved21W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn isrpending(&mut self) -> IsrpendingW<IcsrSpec> {
        IsrpendingW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state"]
    #[inline(always)]
    #[must_use]
    pub fn isrpreempt(&mut self) -> IsrpreemptW<IcsrSpec> {
        IsrpreemptW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
SysTick Targets Non-secure. Controls whether in a single SysTick implementation, the SysTick is Secure or Non-secure. 0x0 SysTick is Secure. 0x1 SysTick is Non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn sttns(&mut self) -> SttnsW<IcsrSpec> {
        SttnsW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear pending SysTick bit 0: No action 1: Clear pending SysTick"]
    #[inline(always)]
    #[must_use]
    pub fn pendstclr(&mut self) -> PendstclrW<IcsrSpec> {
        PendstclrW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Set a pending SysTick bit. 0: No action 1: Set pending SysTick"]
    #[inline(always)]
    #[must_use]
    pub fn pendstset(&mut self) -> PendstsetW<IcsrSpec> {
        PendstsetW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear pending pendSV bit 0: No action 1: Clear pending pendSV"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvclr(&mut self) -> PendsvclrW<IcsrSpec> {
        PendsvclrW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Set pending pendSV bit. 0: No action 1: Set pending PendSV"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvset(&mut self) -> PendsvsetW<IcsrSpec> {
        PendsvsetW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<IcsrSpec> {
        Reserved29W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Pend NMI clear. Allows the NMI exception pending state to be cleared. 0x0 No effect. 0x1 Clear pending status."]
    #[inline(always)]
    #[must_use]
    pub fn pendnmiclr(&mut self) -> PendnmiclrW<IcsrSpec> {
        PendnmiclrW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI"]
    #[inline(always)]
    #[must_use]
    pub fn nmipendset(&mut self) -> NmipendsetW<IcsrSpec> {
        NmipendsetW::new(self, 31)
    }
}
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsrSpec;
impl crate::RegisterSpec for IcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for IcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for IcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for IcsrSpec {
    const RESET_VALUE: u32 = 0;
}
