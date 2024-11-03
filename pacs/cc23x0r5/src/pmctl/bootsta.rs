#[doc = "Register `BOOTSTA` reader"]
pub type R = crate::R<BootstaSpec>;
#[doc = "Register `BOOTSTA` writer"]
pub type W = crate::W<BootstaSpec>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flag {
    #[doc = "255: Internal. Only to be used through TI provided API."]
    AppFaultHandler = 255,
    #[doc = "254: Internal. Only to be used through TI provided API."]
    AppFailApptransfer = 254,
    #[doc = "253: Internal. Only to be used through TI provided API."]
    AppFailNoapp = 253,
    #[doc = "193: Internal. Only to be used through TI provided API."]
    AppWaitloopDbgprobe = 193,
    #[doc = "192: Internal. Only to be used through TI provided API."]
    ModeApp = 192,
    #[doc = "191: Internal. Only to be used through TI provided API."]
    BldrFaultHandler = 191,
    #[doc = "190: Internal. Only to be used through TI provided API."]
    BldrFailApptransfer = 190,
    #[doc = "189: Internal. Only to be used through TI provided API."]
    BldrFailExecutionContext = 189,
    #[doc = "188: Internal. Only to be used through TI provided API."]
    BldrCmdProcessing = 188,
    #[doc = "187: Internal. Only to be used through TI provided API."]
    BldrCmdIdle = 187,
    #[doc = "186: Internal. Only to be used through TI provided API."]
    BldrStarted = 186,
    #[doc = "129: Internal. Only to be used through TI provided API."]
    BldrWaitloopDbgprobe = 129,
    #[doc = "128: Internal. Only to be used through TI provided API."]
    ModeBldr = 128,
    #[doc = "63: Internal. Only to be used through TI provided API."]
    BootFaultHandler = 63,
    #[doc = "62: Internal. Only to be used through TI provided API."]
    BootFailSramRepair = 62,
    #[doc = "56: Internal. Only to be used through TI provided API."]
    BootWaitloopDbgprobe = 56,
    #[doc = "55: Internal. Only to be used through TI provided API."]
    BootExitedSaci = 55,
    #[doc = "54: Internal. Only to be used through TI provided API."]
    BootWaitSwdDisconnect = 54,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    BootEnteredSaci = 32,
    #[doc = "3: Internal. Only to be used through TI provided API."]
    BootGeneralTrims = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    BootSramRepDone = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    BootColdBoot = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    BootReset = 0,
}
impl From<Flag> for u8 {
    #[inline(always)]
    fn from(variant: Flag) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flag {
    type Ux = u8;
}
impl crate::IsEnum for Flag {}
#[doc = "Field `FLAG` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type FlagR = crate::FieldReader<Flag>;
impl FlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flag> {
        match self.bits {
            255 => Some(Flag::AppFaultHandler),
            254 => Some(Flag::AppFailApptransfer),
            253 => Some(Flag::AppFailNoapp),
            193 => Some(Flag::AppWaitloopDbgprobe),
            192 => Some(Flag::ModeApp),
            191 => Some(Flag::BldrFaultHandler),
            190 => Some(Flag::BldrFailApptransfer),
            189 => Some(Flag::BldrFailExecutionContext),
            188 => Some(Flag::BldrCmdProcessing),
            187 => Some(Flag::BldrCmdIdle),
            186 => Some(Flag::BldrStarted),
            129 => Some(Flag::BldrWaitloopDbgprobe),
            128 => Some(Flag::ModeBldr),
            63 => Some(Flag::BootFaultHandler),
            62 => Some(Flag::BootFailSramRepair),
            56 => Some(Flag::BootWaitloopDbgprobe),
            55 => Some(Flag::BootExitedSaci),
            54 => Some(Flag::BootWaitSwdDisconnect),
            32 => Some(Flag::BootEnteredSaci),
            3 => Some(Flag::BootGeneralTrims),
            2 => Some(Flag::BootSramRepDone),
            1 => Some(Flag::BootColdBoot),
            0 => Some(Flag::BootReset),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_app_fault_handler(&self) -> bool {
        *self == Flag::AppFaultHandler
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_app_fail_apptransfer(&self) -> bool {
        *self == Flag::AppFailApptransfer
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_app_fail_noapp(&self) -> bool {
        *self == Flag::AppFailNoapp
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_app_waitloop_dbgprobe(&self) -> bool {
        *self == Flag::AppWaitloopDbgprobe
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_mode_app(&self) -> bool {
        *self == Flag::ModeApp
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_bldr_fault_handler(&self) -> bool {
        *self == Flag::BldrFaultHandler
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_bldr_fail_apptransfer(&self) -> bool {
        *self == Flag::BldrFailApptransfer
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_bldr_fail_execution_context(&self) -> bool {
        *self == Flag::BldrFailExecutionContext
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_bldr_cmd_processing(&self) -> bool {
        *self == Flag::BldrCmdProcessing
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_bldr_cmd_idle(&self) -> bool {
        *self == Flag::BldrCmdIdle
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_bldr_started(&self) -> bool {
        *self == Flag::BldrStarted
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_bldr_waitloop_dbgprobe(&self) -> bool {
        *self == Flag::BldrWaitloopDbgprobe
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_mode_bldr(&self) -> bool {
        *self == Flag::ModeBldr
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_fault_handler(&self) -> bool {
        *self == Flag::BootFaultHandler
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_fail_sram_repair(&self) -> bool {
        *self == Flag::BootFailSramRepair
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_waitloop_dbgprobe(&self) -> bool {
        *self == Flag::BootWaitloopDbgprobe
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_exited_saci(&self) -> bool {
        *self == Flag::BootExitedSaci
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_wait_swd_disconnect(&self) -> bool {
        *self == Flag::BootWaitSwdDisconnect
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_entered_saci(&self) -> bool {
        *self == Flag::BootEnteredSaci
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_general_trims(&self) -> bool {
        *self == Flag::BootGeneralTrims
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_sram_rep_done(&self) -> bool {
        *self == Flag::BootSramRepDone
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_cold_boot(&self) -> bool {
        *self == Flag::BootColdBoot
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_boot_reset(&self) -> bool {
        *self == Flag::BootReset
    }
}
#[doc = "Field `FLAG` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type FlagW<'a, REG> = crate::FieldWriter<'a, REG, 8, Flag>;
impl<'a, REG> FlagW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn app_fault_handler(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::AppFaultHandler)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn app_fail_apptransfer(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::AppFailApptransfer)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn app_fail_noapp(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::AppFailNoapp)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn app_waitloop_dbgprobe(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::AppWaitloopDbgprobe)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode_app(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::ModeApp)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bldr_fault_handler(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BldrFaultHandler)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bldr_fail_apptransfer(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BldrFailApptransfer)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bldr_fail_execution_context(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BldrFailExecutionContext)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bldr_cmd_processing(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BldrCmdProcessing)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bldr_cmd_idle(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BldrCmdIdle)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bldr_started(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BldrStarted)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bldr_waitloop_dbgprobe(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BldrWaitloopDbgprobe)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode_bldr(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::ModeBldr)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_fault_handler(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootFaultHandler)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_fail_sram_repair(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootFailSramRepair)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_waitloop_dbgprobe(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootWaitloopDbgprobe)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_exited_saci(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootExitedSaci)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_wait_swd_disconnect(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootWaitSwdDisconnect)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_entered_saci(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootEnteredSaci)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_general_trims(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootGeneralTrims)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_sram_rep_done(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootSramRepDone)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_cold_boot(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootColdBoot)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boot_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Flag::BootReset)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FlagW<BootstaSpec> {
        FlagW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<BootstaSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootsta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bootsta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootstaSpec;
impl crate::RegisterSpec for BootstaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootsta::R`](R) reader structure"]
impl crate::Readable for BootstaSpec {}
#[doc = "`write(|w| ..)` method takes [`bootsta::W`](W) writer structure"]
impl crate::Writable for BootstaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTSTA to value 0"]
impl crate::Resettable for BootstaSpec {
    const RESET_VALUE: u32 = 0;
}
