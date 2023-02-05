#[doc = "Register `CPU_LOCK_CFG` reader"]
pub struct R(crate::R<CPU_LOCK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_LOCK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_LOCK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_LOCK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_LOCK_CFG` writer"]
pub struct W(crate::W<CPU_LOCK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_LOCK_CFG_SPEC>;
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
impl From<crate::W<CPU_LOCK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_LOCK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKSMPU_N` reader - 0:0\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSMPU by ROM boot FW"]
pub type LOCKSMPU_N_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSMPU_N` writer - 0:0\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSMPU by ROM boot FW"]
pub type LOCKSMPU_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_LOCK_CFG_SPEC, bool, O>;
#[doc = "Field `LOCKNSMPU_N` reader - 1:1\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSMPU by ROM boot FW"]
pub type LOCKNSMPU_N_R = crate::BitReader<bool>;
#[doc = "Field `LOCKNSMPU_N` writer - 1:1\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSMPU by ROM boot FW"]
pub type LOCKNSMPU_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_LOCK_CFG_SPEC, bool, O>;
#[doc = "Field `LOCKSAU_N` reader - 2:2\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSAU by ROM boot FW"]
pub type LOCKSAU_N_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSAU_N` writer - 2:2\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSAU by ROM boot FW"]
pub type LOCKSAU_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_LOCK_CFG_SPEC, bool, O>;
#[doc = "Field `LOCKSVTAIRCR_N` reader - 3:3\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSVTAIRCR by ROM boot FW"]
pub type LOCKSVTAIRCR_N_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSVTAIRCR_N` writer - 3:3\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSVTAIRCR by ROM boot FW"]
pub type LOCKSVTAIRCR_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_LOCK_CFG_SPEC, bool, O>;
#[doc = "Field `LOCKNSVTOR_N` reader - 4:4\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSVTOR by ROM boot FW"]
pub type LOCKNSVTOR_N_R = crate::BitReader<bool>;
#[doc = "Field `LOCKNSVTOR_N` writer - 4:4\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSVTOR by ROM boot FW"]
pub type LOCKNSVTOR_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_LOCK_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSMPU by ROM boot FW"]
    #[inline(always)]
    pub fn locksmpu_n(&self) -> LOCKSMPU_N_R {
        LOCKSMPU_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSMPU by ROM boot FW"]
    #[inline(always)]
    pub fn locknsmpu_n(&self) -> LOCKNSMPU_N_R {
        LOCKNSMPU_N_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSAU by ROM boot FW"]
    #[inline(always)]
    pub fn locksau_n(&self) -> LOCKSAU_N_R {
        LOCKSAU_N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSVTAIRCR by ROM boot FW"]
    #[inline(always)]
    pub fn locksvtaircr_n(&self) -> LOCKSVTAIRCR_N_R {
        LOCKSVTAIRCR_N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSVTOR by ROM boot FW"]
    #[inline(always)]
    pub fn locknsvtor_n(&self) -> LOCKNSVTOR_N_R {
        LOCKNSVTOR_N_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSMPU by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu_n(&mut self) -> LOCKSMPU_N_W<0> {
        LOCKSMPU_N_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSMPU by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locknsmpu_n(&mut self) -> LOCKNSMPU_N_W<1> {
        LOCKNSMPU_N_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSAU by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locksau_n(&mut self) -> LOCKSAU_N_W<2> {
        LOCKSAU_N_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKSVTAIRCR by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr_n(&mut self) -> LOCKSVTAIRCR_N_W<3> {
        LOCKSVTAIRCR_N_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Value will be inverted and written to PRCM:CPULOCK.LOCKNSVTOR by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn locknsvtor_n(&mut self) -> LOCKNSVTOR_N_W<4> {
        LOCKNSVTOR_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for MCU CPU lock options\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_lock_cfg](index.html) module"]
pub struct CPU_LOCK_CFG_SPEC;
impl crate::RegisterSpec for CPU_LOCK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_lock_cfg::R](R) reader structure"]
impl crate::Readable for CPU_LOCK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_lock_cfg::W](W) writer structure"]
impl crate::Writable for CPU_LOCK_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_LOCK_CFG to value 0xffff_ffff"]
impl crate::Resettable for CPU_LOCK_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
