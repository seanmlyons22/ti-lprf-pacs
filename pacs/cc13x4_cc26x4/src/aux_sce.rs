#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub ctl: CTL,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub fetchstat: FETCHSTAT,
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    pub cpustat: CPUSTAT,
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    pub wustat: WUSTAT,
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    pub reg1_0: REG1_0,
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    pub reg3_2: REG3_2,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub reg5_4: REG5_4,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub reg7_6: REG7_6,
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    pub loopaddr: LOOPADDR,
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub loopcnt: LOOPCNT,
    #[doc = "0x28 - Non-Secure DDI Access 0 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
    pub nonsecddiacc0: NONSECDDIACC0,
    #[doc = "0x2c - Non-Secure DDI Access 1 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
    pub nonsecddiacc1: NONSECDDIACC1,
    #[doc = "0x30 - Non-Secure DDI Access 2 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
    pub nonsecddiacc2: NONSECDDIACC2,
    #[doc = "0x34 - Non-Secure DDI Access 3 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
    pub nonsecddiacc3: NONSECDDIACC3,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "FETCHSTAT (rw) register accessor: an alias for `Reg<FETCHSTAT_SPEC>`"]
pub type FETCHSTAT = crate::Reg<fetchstat::FETCHSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fetchstat;
#[doc = "CPUSTAT (rw) register accessor: an alias for `Reg<CPUSTAT_SPEC>`"]
pub type CPUSTAT = crate::Reg<cpustat::CPUSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpustat;
#[doc = "WUSTAT (rw) register accessor: an alias for `Reg<WUSTAT_SPEC>`"]
pub type WUSTAT = crate::Reg<wustat::WUSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod wustat;
#[doc = "REG1_0 (rw) register accessor: an alias for `Reg<REG1_0_SPEC>`"]
pub type REG1_0 = crate::Reg<reg1_0::REG1_0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg1_0;
#[doc = "REG3_2 (rw) register accessor: an alias for `Reg<REG3_2_SPEC>`"]
pub type REG3_2 = crate::Reg<reg3_2::REG3_2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg3_2;
#[doc = "REG5_4 (rw) register accessor: an alias for `Reg<REG5_4_SPEC>`"]
pub type REG5_4 = crate::Reg<reg5_4::REG5_4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg5_4;
#[doc = "REG7_6 (rw) register accessor: an alias for `Reg<REG7_6_SPEC>`"]
pub type REG7_6 = crate::Reg<reg7_6::REG7_6_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg7_6;
#[doc = "LOOPADDR (rw) register accessor: an alias for `Reg<LOOPADDR_SPEC>`"]
pub type LOOPADDR = crate::Reg<loopaddr::LOOPADDR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopaddr;
#[doc = "LOOPCNT (rw) register accessor: an alias for `Reg<LOOPCNT_SPEC>`"]
pub type LOOPCNT = crate::Reg<loopcnt::LOOPCNT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopcnt;
#[doc = "NONSECDDIACC0 (rw) register accessor: an alias for `Reg<NONSECDDIACC0_SPEC>`"]
pub type NONSECDDIACC0 = crate::Reg<nonsecddiacc0::NONSECDDIACC0_SPEC>;
#[doc = "Non-Secure DDI Access 0 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
pub mod nonsecddiacc0;
#[doc = "NONSECDDIACC1 (rw) register accessor: an alias for `Reg<NONSECDDIACC1_SPEC>`"]
pub type NONSECDDIACC1 = crate::Reg<nonsecddiacc1::NONSECDDIACC1_SPEC>;
#[doc = "Non-Secure DDI Access 1 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
pub mod nonsecddiacc1;
#[doc = "NONSECDDIACC2 (rw) register accessor: an alias for `Reg<NONSECDDIACC2_SPEC>`"]
pub type NONSECDDIACC2 = crate::Reg<nonsecddiacc2::NONSECDDIACC2_SPEC>;
#[doc = "Non-Secure DDI Access 2 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
pub mod nonsecddiacc2;
#[doc = "NONSECDDIACC3 (rw) register accessor: an alias for `Reg<NONSECDDIACC3_SPEC>`"]
pub type NONSECDDIACC3 = crate::Reg<nonsecddiacc3::NONSECDDIACC3_SPEC>;
#[doc = "Non-Secure DDI Access 3 When system is in secure state, AUX_SCE is allowed to update a predefined DDI half-word using SET or CLR access. Configuration will determine if AUX_SCE can read the same half-word. An access to a non-allowed register will suspend the AUX_SCE when system state is secure. If ADDR field in two or more NONSECDDIACC registers are equal, the MASK and RD_EN from the highest numbered register will be used. Examples: Half-word with address of 0 corresponds to DDI_0_OSC:CTL0 bit range \\[15:0\\]. Half-word with address of 1 corresponds to DDI_0_OSC:CTL0 bit range \\[31:16\\]. … Half-word with address of 34 corresponds to DDI_0_OSC:STAT2 bit range \\[15:0\\]. Half-word with address of 35 corresponds to DDI_0_OSC:STAT2 bit range \\[31:16\\]."]
pub mod nonsecddiacc3;
