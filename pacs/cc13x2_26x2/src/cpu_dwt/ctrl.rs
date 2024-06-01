#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CYCCNTENA` reader - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
pub type CyccntenaR = crate::BitReader;
#[doc = "Field `CYCCNTENA` writer - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
pub type CyccntenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSTPRESET` reader - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
pub type PostpresetR = crate::FieldReader;
#[doc = "Field `POSTPRESET` writer - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
pub type PostpresetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `POSTCNT` reader - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
pub type PostcntR = crate::FieldReader;
#[doc = "Field `POSTCNT` writer - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
pub type PostcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cyctap {
    #[doc = "1: Selects bit \\[10\\]
to tap"]
    Bit10 = 1,
    #[doc = "0: Selects bit \\[6\\]
to tap"]
    Bit6 = 0,
}
impl From<Cyctap> for bool {
    #[inline(always)]
    fn from(variant: Cyctap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CYCTAP` reader - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
pub type CyctapR = crate::BitReader<Cyctap>;
impl CyctapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cyctap {
        match self.bits {
            true => Cyctap::Bit10,
            false => Cyctap::Bit6,
        }
    }
    #[doc = "Selects bit \\[10\\]
to tap"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == Cyctap::Bit10
    }
    #[doc = "Selects bit \\[6\\]
to tap"]
    #[inline(always)]
    pub fn is_bit6(&self) -> bool {
        *self == Cyctap::Bit6
    }
}
#[doc = "Field `CYCTAP` writer - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
pub type CyctapW<'a, REG> = crate::BitWriter<'a, REG, Cyctap>;
impl<'a, REG> CyctapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects bit \\[10\\]
to tap"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(Cyctap::Bit10)
    }
    #[doc = "Selects bit \\[6\\]
to tap"]
    #[inline(always)]
    pub fn bit6(self) -> &'a mut crate::W<REG> {
        self.variant(Cyctap::Bit6)
    }
}
#[doc = "11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Synctap {
    #[doc = "3: Tap at bit 28 of CYCCNT"]
    Bit28 = 3,
    #[doc = "2: Tap at bit 26 of CYCCNT"]
    Bit26 = 2,
    #[doc = "1: Tap at bit 24 of CYCCNT"]
    Bit24 = 1,
    #[doc = "0: Disabled. No synchronization packets"]
    Dis = 0,
}
impl From<Synctap> for u8 {
    #[inline(always)]
    fn from(variant: Synctap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Synctap {
    type Ux = u8;
}
impl crate::IsEnum for Synctap {}
#[doc = "Field `SYNCTAP` reader - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
pub type SynctapR = crate::FieldReader<Synctap>;
impl SynctapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Synctap {
        match self.bits {
            3 => Synctap::Bit28,
            2 => Synctap::Bit26,
            1 => Synctap::Bit24,
            0 => Synctap::Dis,
            _ => unreachable!(),
        }
    }
    #[doc = "Tap at bit 28 of CYCCNT"]
    #[inline(always)]
    pub fn is_bit28(&self) -> bool {
        *self == Synctap::Bit28
    }
    #[doc = "Tap at bit 26 of CYCCNT"]
    #[inline(always)]
    pub fn is_bit26(&self) -> bool {
        *self == Synctap::Bit26
    }
    #[doc = "Tap at bit 24 of CYCCNT"]
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == Synctap::Bit24
    }
    #[doc = "Disabled. No synchronization packets"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Synctap::Dis
    }
}
#[doc = "Field `SYNCTAP` writer - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
pub type SynctapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Synctap, crate::Safe>;
impl<'a, REG> SynctapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Tap at bit 28 of CYCCNT"]
    #[inline(always)]
    pub fn bit28(self) -> &'a mut crate::W<REG> {
        self.variant(Synctap::Bit28)
    }
    #[doc = "Tap at bit 26 of CYCCNT"]
    #[inline(always)]
    pub fn bit26(self) -> &'a mut crate::W<REG> {
        self.variant(Synctap::Bit26)
    }
    #[doc = "Tap at bit 24 of CYCCNT"]
    #[inline(always)]
    pub fn bit24(self) -> &'a mut crate::W<REG> {
        self.variant(Synctap::Bit24)
    }
    #[doc = "Disabled. No synchronization packets"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Synctap::Dis)
    }
}
#[doc = "Field `PCSAMPLEENA` reader - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
pub type PcsampleenaR = crate::BitReader;
#[doc = "Field `PCSAMPLEENA` writer - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
pub type PcsampleenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED13` reader - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13R = crate::FieldReader;
#[doc = "Field `RESERVED13` writer - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXCTRCENA` reader - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
pub type ExctrcenaR = crate::BitReader;
#[doc = "Field `EXCTRCENA` writer - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
pub type ExctrcenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPIEVTENA` reader - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
pub type CpievtenaR = crate::BitReader;
#[doc = "Field `CPIEVTENA` writer - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
pub type CpievtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCEVTENA` reader - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
pub type ExcevtenaR = crate::BitReader;
#[doc = "Field `EXCEVTENA` writer - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
pub type ExcevtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEPEVTENA` reader - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
pub type SleepevtenaR = crate::BitReader;
#[doc = "Field `SLEEPEVTENA` writer - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
pub type SleepevtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSUEVTENA` reader - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
pub type LsuevtenaR = crate::BitReader;
#[doc = "Field `LSUEVTENA` writer - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
pub type LsuevtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOLDEVTENA` reader - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
pub type FoldevtenaR = crate::BitReader;
#[doc = "Field `FOLDEVTENA` writer - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
pub type FoldevtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYCEVTENA` reader - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
pub type CycevtenaR = crate::BitReader;
#[doc = "Field `CYCEVTENA` writer - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
pub type CycevtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED23` reader - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23R = crate::BitReader;
#[doc = "Field `RESERVED23` writer - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOPRFCNT` reader - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
pub type NoprfcntR = crate::BitReader;
#[doc = "Field `NOPRFCNT` writer - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
pub type NoprfcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOCYCCNT` reader - 25:25\\]
When set, CYCCNT is not supported."]
pub type NocyccntR = crate::BitReader;
#[doc = "Field `NOCYCCNT` writer - 25:25\\]
When set, CYCCNT is not supported."]
pub type NocyccntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26R = crate::FieldReader;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline(always)]
    pub fn cyccntena(&self) -> CyccntenaR {
        CyccntenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline(always)]
    pub fn postpreset(&self) -> PostpresetR {
        PostpresetR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline(always)]
    pub fn postcnt(&self) -> PostcntR {
        PostcntR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline(always)]
    pub fn cyctap(&self) -> CyctapR {
        CyctapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline(always)]
    pub fn synctap(&self) -> SynctapR {
        SynctapR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline(always)]
    pub fn pcsampleena(&self) -> PcsampleenaR {
        PcsampleenaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline(always)]
    pub fn exctrcena(&self) -> ExctrcenaR {
        ExctrcenaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline(always)]
    pub fn cpievtena(&self) -> CpievtenaR {
        CpievtenaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline(always)]
    pub fn excevtena(&self) -> ExcevtenaR {
        ExcevtenaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline(always)]
    pub fn sleepevtena(&self) -> SleepevtenaR {
        SleepevtenaR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline(always)]
    pub fn lsuevtena(&self) -> LsuevtenaR {
        LsuevtenaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline(always)]
    pub fn foldevtena(&self) -> FoldevtenaR {
        FoldevtenaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
    #[inline(always)]
    pub fn cycevtena(&self) -> CycevtenaR {
        CycevtenaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline(always)]
    pub fn noprfcnt(&self) -> NoprfcntR {
        NoprfcntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
When set, CYCCNT is not supported."]
    #[inline(always)]
    pub fn nocyccnt(&self) -> NocyccntR {
        NocyccntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline(always)]
    #[must_use]
    pub fn cyccntena(&mut self) -> CyccntenaW<CtrlSpec> {
        CyccntenaW::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline(always)]
    #[must_use]
    pub fn postpreset(&mut self) -> PostpresetW<CtrlSpec> {
        PostpresetW::new(self, 1)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline(always)]
    #[must_use]
    pub fn postcnt(&mut self) -> PostcntW<CtrlSpec> {
        PostcntW::new(self, 5)
    }
    #[doc = "Bit 9 - 9:9\\]
Selects a tap on CYCCNT. These are spaced at bits \\[6\\]
and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline(always)]
    #[must_use]
    pub fn cyctap(&mut self) -> CyctapW<CtrlSpec> {
        CyctapW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline(always)]
    #[must_use]
    pub fn synctap(&mut self) -> SynctapW<CtrlSpec> {
        SynctapW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pcsampleena(&mut self) -> PcsampleenaW<CtrlSpec> {
        PcsampleenaW::new(self, 12)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<CtrlSpec> {
        Reserved13W::new(self, 13)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline(always)]
    #[must_use]
    pub fn exctrcena(&mut self) -> ExctrcenaW<CtrlSpec> {
        ExctrcenaW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cpievtena(&mut self) -> CpievtenaW<CtrlSpec> {
        CpievtenaW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline(always)]
    #[must_use]
    pub fn excevtena(&mut self) -> ExcevtenaW<CtrlSpec> {
        ExcevtenaW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sleepevtena(&mut self) -> SleepevtenaW<CtrlSpec> {
        SleepevtenaW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lsuevtena(&mut self) -> LsuevtenaW<CtrlSpec> {
        LsuevtenaW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline(always)]
    #[must_use]
    pub fn foldevtena(&mut self) -> FoldevtenaW<CtrlSpec> {
        FoldevtenaW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cycevtena(&mut self) -> CycevtenaW<CtrlSpec> {
        CycevtenaW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> Reserved23W<CtrlSpec> {
        Reserved23W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline(always)]
    #[must_use]
    pub fn noprfcnt(&mut self) -> NoprfcntW<CtrlSpec> {
        NoprfcntW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
When set, CYCCNT is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn nocyccnt(&mut self) -> NocyccntW<CtrlSpec> {
        NocyccntW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> Reserved26W<CtrlSpec> {
        Reserved26W::new(self, 26)
    }
}
#[doc = "Control Use the DWT Control Register to enable the DWT unit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x4000_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
