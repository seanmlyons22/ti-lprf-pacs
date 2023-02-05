#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    soar: Soar,
    sstat_sctl: SstatSctl,
    sdr: Sdr,
    simr: Simr,
    sris: Sris,
    smis: Smis,
    sicr: Sicr,
    _reserved7: [u8; 0x07e4],
    msa: Msa,
    mstat_mctl: MstatMctl,
    mdr: Mdr,
    mtpr: Mtpr,
    mimr: Mimr,
    mris: Mris,
    mmis: Mmis,
    micr: Micr,
    mcr: Mcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
    #[inline(always)]
    pub const fn soar(&self) -> &Soar {
        &self.soar
    }
    #[doc = "0x04 - Slave Control and Slave Status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub const fn sstat_sctl(&self) -> &SstatSctl {
        &self.sstat_sctl
    }
    #[doc = "0x08 - Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
    #[inline(always)]
    pub const fn sdr(&self) -> &Sdr {
        &self.sdr
    }
    #[doc = "0x0c - Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    #[inline(always)]
    pub const fn simr(&self) -> &Simr {
        &self.simr
    }
    #[doc = "0x10 - Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
    #[inline(always)]
    pub const fn sris(&self) -> &Sris {
        &self.sris
    }
    #[doc = "0x14 - Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
    #[inline(always)]
    pub const fn smis(&self) -> &Smis {
        &self.smis
    }
    #[doc = "0x18 - Slave Interrupt Clear This register clears the raw interrupt SRIS."]
    #[inline(always)]
    pub const fn sicr(&self) -> &Sicr {
        &self.sicr
    }
    #[doc = "0x800 - Master Slave Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
    #[inline(always)]
    pub const fn msa(&self) -> &Msa {
        &self.msa
    }
    #[doc = "0x804 - Master Control and Status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub const fn mstat_mctl(&self) -> &MstatMctl {
        &self.mstat_mctl
    }
    #[doc = "0x808 - Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
    #[inline(always)]
    pub const fn mdr(&self) -> &Mdr {
        &self.mdr
    }
    #[doc = "0x80c - Master Timer Period This register specifies the period of the SCL clock."]
    #[inline(always)]
    pub const fn mtpr(&self) -> &Mtpr {
        &self.mtpr
    }
    #[doc = "0x810 - Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    #[inline(always)]
    pub const fn mimr(&self) -> &Mimr {
        &self.mimr
    }
    #[doc = "0x814 - Master Raw Interrupt Status This register show the unmasked interrupt status."]
    #[inline(always)]
    pub const fn mris(&self) -> &Mris {
        &self.mris
    }
    #[doc = "0x818 - Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
    #[inline(always)]
    pub const fn mmis(&self) -> &Mmis {
        &self.mmis
    }
    #[doc = "0x81c - Master Interrupt Clear This register clears the raw and masked interrupt."]
    #[inline(always)]
    pub const fn micr(&self) -> &Micr {
        &self.micr
    }
    #[doc = "0x820 - Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
}
#[doc = "SOAR (rw) register accessor: Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soar`]
module"]
#[doc(alias = "SOAR")]
pub type Soar = crate::Reg<soar::SoarSpec>;
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
pub mod soar;
#[doc = "SSTAT_SCTL (rw) register accessor: Slave Control and Slave Status This register functions as a control register when written, and a status register when read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstat_sctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstat_sctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstat_sctl`]
module"]
#[doc(alias = "SSTAT_SCTL")]
pub type SstatSctl = crate::Reg<sstat_sctl::SstatSctlSpec>;
#[doc = "Slave Control and Slave Status This register functions as a control register when written, and a status register when read."]
pub mod sstat_sctl;
#[doc = "SDR (rw) register accessor: Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdr`]
module"]
#[doc(alias = "SDR")]
pub type Sdr = crate::Reg<sdr::SdrSpec>;
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
pub mod sdr;
#[doc = "SIMR (rw) register accessor: Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simr`]
module"]
#[doc(alias = "SIMR")]
pub type Simr = crate::Reg<simr::SimrSpec>;
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod simr;
#[doc = "SRIS (rw) register accessor: Slave Raw Interrupt Status This register shows the unmasked interrupt status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sris`]
module"]
#[doc(alias = "SRIS")]
pub type Sris = crate::Reg<sris::SrisSpec>;
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
pub mod sris;
#[doc = "SMIS (rw) register accessor: Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smis`]
module"]
#[doc(alias = "SMIS")]
pub type Smis = crate::Reg<smis::SmisSpec>;
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
pub mod smis;
#[doc = "SICR (rw) register accessor: Slave Interrupt Clear This register clears the raw interrupt SRIS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sicr`]
module"]
#[doc(alias = "SICR")]
pub type Sicr = crate::Reg<sicr::SicrSpec>;
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS."]
pub mod sicr;
#[doc = "MSA (rw) register accessor: Master Slave Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msa`]
module"]
#[doc(alias = "MSA")]
pub type Msa = crate::Reg<msa::MsaSpec>;
#[doc = "Master Slave Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
pub mod msa;
#[doc = "MSTAT_MCTL (rw) register accessor: Master Control and Status This register functions as a control register when written, and a status register when read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstat_mctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstat_mctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstat_mctl`]
module"]
#[doc(alias = "MSTAT_MCTL")]
pub type MstatMctl = crate::Reg<mstat_mctl::MstatMctlSpec>;
#[doc = "Master Control and Status This register functions as a control register when written, and a status register when read."]
pub mod mstat_mctl;
#[doc = "MDR (rw) register accessor: Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdr`]
module"]
#[doc(alias = "MDR")]
pub type Mdr = crate::Reg<mdr::MdrSpec>;
#[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
pub mod mdr;
#[doc = "MTPR (rw) register accessor: Master Timer Period This register specifies the period of the SCL clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtpr`]
module"]
#[doc(alias = "MTPR")]
pub type Mtpr = crate::Reg<mtpr::MtprSpec>;
#[doc = "Master Timer Period This register specifies the period of the SCL clock."]
pub mod mtpr;
#[doc = "MIMR (rw) register accessor: Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mimr`]
module"]
#[doc(alias = "MIMR")]
pub type Mimr = crate::Reg<mimr::MimrSpec>;
#[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod mimr;
#[doc = "MRIS (rw) register accessor: Master Raw Interrupt Status This register show the unmasked interrupt status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mris`]
module"]
#[doc(alias = "MRIS")]
pub type Mris = crate::Reg<mris::MrisSpec>;
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status."]
pub mod mris;
#[doc = "MMIS (rw) register accessor: Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmis`]
module"]
#[doc(alias = "MMIS")]
pub type Mmis = crate::Reg<mmis::MmisSpec>;
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
pub mod mmis;
#[doc = "MICR (rw) register accessor: Master Interrupt Clear This register clears the raw and masked interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`micr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@micr`]
module"]
#[doc(alias = "MICR")]
pub type Micr = crate::Reg<micr::MicrSpec>;
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt."]
pub mod micr;
#[doc = "MCR (rw) register accessor: Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
pub mod mcr;
