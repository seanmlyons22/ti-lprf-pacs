#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwmclken: Pwmclken,
}
impl RegisterBlock {
    #[doc = "0x00 - RF Core Power Management and Clock Enable"]
    #[inline(always)]
    pub const fn pwmclken(&self) -> &Pwmclken {
        &self.pwmclken
    }
}
#[doc = "PWMCLKEN (rw) register accessor: RF Core Power Management and Clock Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwmclken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwmclken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmclken`]
module"]
#[doc(alias = "PWMCLKEN")]
pub type Pwmclken = crate::Reg<pwmclken::PwmclkenSpec>;
#[doc = "RF Core Power Management and Clock Enable"]
pub mod pwmclken;
