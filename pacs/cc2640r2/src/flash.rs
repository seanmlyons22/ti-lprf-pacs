#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    stat: Stat,
    _reserved1: [u8; 0x04],
    cfg: Cfg,
    syscode_start: SyscodeStart,
    flash_size: FlashSize,
    _reserved4: [u8; 0x0c],
    fwlock: Fwlock,
    fwflag: Fwflag,
    _reserved6: [u8; 0x0fbc],
    efuse: Efuse,
    efuseaddr: Efuseaddr,
    dataupper: Dataupper,
    datalower: Datalower,
    efusecfg: Efusecfg,
    efusestat: Efusestat,
    acc: Acc,
    boundary: Boundary,
    efuseflag: Efuseflag,
    efusekey: Efusekey,
    efuserelease: Efuserelease,
    efusepins: Efusepins,
    efusecra: Efusecra,
    efuseread: Efuseread,
    efuseprogram: Efuseprogram,
    efuseerror: Efuseerror,
    singlebit: Singlebit,
    twobit: Twobit,
    selftestcyc: Selftestcyc,
    selftestsign: Selftestsign,
    _reserved26: [u8; 0x0fb0],
    frdctl: Frdctl,
    fsprd: Fsprd,
    fedacctl1: Fedacctl1,
    fedacctl2: Fedacctl2,
    fcor_err_cnt: FcorErrCnt,
    fcor_err_add: FcorErrAdd,
    fcor_err_pos: FcorErrPos,
    fedacstat: Fedacstat,
    func_err_add: FuncErrAdd,
    fedacsdis: Fedacsdis,
    fprim_add_tag: FprimAddTag,
    fredu_add_tag: FreduAddTag,
    fbprot: Fbprot,
    fbse: Fbse,
    fbbusy: Fbbusy,
    fbac: Fbac,
    fbfallback: Fbfallback,
    fbprdy: Fbprdy,
    fpac1: Fpac1,
    fpac2: Fpac2,
    fmac: Fmac,
    fmstat: Fmstat,
    femu_dmsw: FemuDmsw,
    femu_dlsw: FemuDlsw,
    femu_ecc: FemuEcc,
    flock: Flock,
    femu_addr: FemuAddr,
    fdiagctl: Fdiagctl,
    fraw_datah: FrawDatah,
    fraw_datal: FrawDatal,
    fraw_ecc: FrawEcc,
    fpar_ovr: FparOvr,
    fvreadct: Fvreadct,
    fvhvct1: Fvhvct1,
    fvhvct2: Fvhvct2,
    fvhvct3: Fvhvct3,
    fvnvct: Fvnvct,
    fvslp: Fvslp,
    fvwlct: Fvwlct,
    fefusectl: Fefusectl,
    fefusestat: Fefusestat,
    fefusedata: Fefusedata,
    fseqpmp: Fseqpmp,
    fclktrim: Fclktrim,
    rom_test: RomTest,
    _reserved71: [u8; 0x0c],
    fedacsdis2: Fedacsdis2,
    _reserved72: [u8; 0x3c],
    fbstrobes: Fbstrobes,
    fpstrobes: Fpstrobes,
    fbmode: Fbmode,
    ftcr: Ftcr,
    faddr: Faddr,
    fpmtctl: Fpmtctl,
    pbistctl: Pbistctl,
    ftctl: Ftctl,
    fwpwrite0: Fwpwrite0,
    fwpwrite1: Fwpwrite1,
    fwpwrite2: Fwpwrite2,
    fwpwrite3: Fwpwrite3,
    fwpwrite4: Fwpwrite4,
    fwpwrite5: Fwpwrite5,
    fwpwrite6: Fwpwrite6,
    fwpwrite7: Fwpwrite7,
    fwpwrite_ecc: FwpwriteEcc,
    fswstat: Fswstat,
    _reserved90: [u8; 0xb8],
    fsm_glbctl: FsmGlbctl,
    fsm_state: FsmState,
    fsm_stat: FsmStat,
    fsm_cmd: FsmCmd,
    fsm_pe_osu: FsmPeOsu,
    fsm_vstat: FsmVstat,
    fsm_pe_vsu: FsmPeVsu,
    fsm_cmp_vsu: FsmCmpVsu,
    fsm_ex_val: FsmExVal,
    fsm_rd_h: FsmRdH,
    fsm_p_oh: FsmPOh,
    fsm_era_oh: FsmEraOh,
    fsm_sav_ppul: FsmSavPpul,
    fsm_pe_vh: FsmPeVh,
    _reserved104: [u8; 0x08],
    fsm_prg_pw: FsmPrgPw,
    fsm_era_pw: FsmEraPw,
    _reserved106: [u8; 0x0c],
    fsm_sav_era_pul: FsmSavEraPul,
    fsm_timer: FsmTimer,
    fsm_mode: FsmMode,
    fsm_pgm: FsmPgm,
    fsm_era: FsmEra,
    fsm_prg_pul: FsmPrgPul,
    fsm_era_pul: FsmEraPul,
    fsm_step_size: FsmStepSize,
    fsm_pul_cntr: FsmPulCntr,
    fsm_ec_step_height: FsmEcStepHeight,
    fsm_st_machine: FsmStMachine,
    fsm_fles: FsmFles,
    _reserved118: [u8; 0x04],
    fsm_wr_ena: FsmWrEna,
    fsm_acc_pp: FsmAccPp,
    fsm_acc_ep: FsmAccEp,
    _reserved121: [u8; 0x0c],
    fsm_addr: FsmAddr,
    fsm_sector: FsmSector,
    fmc_rev_id: FmcRevId,
    fsm_err_addr: FsmErrAddr,
    fsm_pgm_maxpul: FsmPgmMaxpul,
    fsm_execute: FsmExecute,
    eeprom_cfg: EepromCfg,
    _reserved128: [u8; 0x04],
    fsm_sector1: FsmSector1,
    fsm_sector2: FsmSector2,
    _reserved130: [u8; 0x18],
    fsm_bsle0: FsmBsle0,
    fsm_bsle1: FsmBsle1,
    _reserved132: [u8; 0x08],
    fsm_bslp0: FsmBslp0,
    fsm_bslp1: FsmBslp1,
    _reserved134: [u8; 0x0108],
    fcfg_bank: FcfgBank,
    fcfg_wrapper: FcfgWrapper,
    fcfg_bnk_type: FcfgBnkType,
    _reserved137: [u8; 0x04],
    fcfg_b0_start: FcfgB0Start,
    fcfg_b1_start: FcfgB1Start,
    fcfg_b2_start: FcfgB2Start,
    fcfg_b3_start: FcfgB3Start,
    fcfg_b4_start: FcfgB4Start,
    fcfg_b5_start: FcfgB5Start,
    fcfg_b6_start: FcfgB6Start,
    fcfg_b7_start: FcfgB7Start,
    fcfg_b0_ssize0: FcfgB0Ssize0,
    fcfg_b0_ssize1: FcfgB0Ssize1,
    fcfg_b0_ssize2: FcfgB0Ssize2,
    fcfg_b0_ssize3: FcfgB0Ssize3,
    fcfg_b1_ssize0: FcfgB1Ssize0,
    fcfg_b1_ssize1: FcfgB1Ssize1,
    fcfg_b1_ssize2: FcfgB1Ssize2,
    fcfg_b1_ssize3: FcfgB1Ssize3,
    fcfg_b2_ssize0: FcfgB2Ssize0,
    fcfg_b2_ssize1: FcfgB2Ssize1,
    fcfg_b2_ssize2: FcfgB2Ssize2,
    fcfg_b2_ssize3: FcfgB2Ssize3,
    fcfg_b3_ssize0: FcfgB3Ssize0,
    fcfg_b3_ssize1: FcfgB3Ssize1,
    fcfg_b3_ssize2: FcfgB3Ssize2,
    fcfg_b3_ssize3: FcfgB3Ssize3,
    fcfg_b4_ssize0: FcfgB4Ssize0,
    fcfg_b4_ssize1: FcfgB4Ssize1,
    fcfg_b4_ssize2: FcfgB4Ssize2,
    fcfg_b4_ssize3: FcfgB4Ssize3,
    fcfg_b5_ssize0: FcfgB5Ssize0,
    fcfg_b5_ssize1: FcfgB5Ssize1,
    fcfg_b5_ssize2: FcfgB5Ssize2,
    fcfg_b5_ssize3: FcfgB5Ssize3,
    fcfg_b6_ssize0: FcfgB6Ssize0,
    fcfg_b6_ssize1: FcfgB6Ssize1,
    fcfg_b6_ssize2: FcfgB6Ssize2,
    fcfg_b6_ssize3: FcfgB6Ssize3,
    fcfg_b7_ssize0: FcfgB7Ssize0,
    fcfg_b7_ssize1: FcfgB7Ssize1,
    fcfg_b7_ssize2: FcfgB7Ssize2,
    fcfg_b7_ssize3: FcfgB7Ssize3,
}
impl RegisterBlock {
    #[doc = "0x1c - FMC and Efuse Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x28 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn syscode_start(&self) -> &SyscodeStart {
        &self.syscode_start
    }
    #[doc = "0x2c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flash_size(&self) -> &FlashSize {
        &self.flash_size
    }
    #[doc = "0x3c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwlock(&self) -> &Fwlock {
        &self.fwlock
    }
    #[doc = "0x40 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwflag(&self) -> &Fwflag {
        &self.fwflag
    }
    #[doc = "0x1000 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuse(&self) -> &Efuse {
        &self.efuse
    }
    #[doc = "0x1004 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseaddr(&self) -> &Efuseaddr {
        &self.efuseaddr
    }
    #[doc = "0x1008 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn dataupper(&self) -> &Dataupper {
        &self.dataupper
    }
    #[doc = "0x100c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn datalower(&self) -> &Datalower {
        &self.datalower
    }
    #[doc = "0x1010 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusecfg(&self) -> &Efusecfg {
        &self.efusecfg
    }
    #[doc = "0x1014 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusestat(&self) -> &Efusestat {
        &self.efusestat
    }
    #[doc = "0x1018 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn acc(&self) -> &Acc {
        &self.acc
    }
    #[doc = "0x101c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn boundary(&self) -> &Boundary {
        &self.boundary
    }
    #[doc = "0x1020 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseflag(&self) -> &Efuseflag {
        &self.efuseflag
    }
    #[doc = "0x1024 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusekey(&self) -> &Efusekey {
        &self.efusekey
    }
    #[doc = "0x1028 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuserelease(&self) -> &Efuserelease {
        &self.efuserelease
    }
    #[doc = "0x102c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusepins(&self) -> &Efusepins {
        &self.efusepins
    }
    #[doc = "0x1030 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efusecra(&self) -> &Efusecra {
        &self.efusecra
    }
    #[doc = "0x1034 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseread(&self) -> &Efuseread {
        &self.efuseread
    }
    #[doc = "0x1038 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseprogram(&self) -> &Efuseprogram {
        &self.efuseprogram
    }
    #[doc = "0x103c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn efuseerror(&self) -> &Efuseerror {
        &self.efuseerror
    }
    #[doc = "0x1040 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn singlebit(&self) -> &Singlebit {
        &self.singlebit
    }
    #[doc = "0x1044 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn twobit(&self) -> &Twobit {
        &self.twobit
    }
    #[doc = "0x1048 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn selftestcyc(&self) -> &Selftestcyc {
        &self.selftestcyc
    }
    #[doc = "0x104c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn selftestsign(&self) -> &Selftestsign {
        &self.selftestsign
    }
    #[doc = "0x2000 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn frdctl(&self) -> &Frdctl {
        &self.frdctl
    }
    #[doc = "0x2004 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsprd(&self) -> &Fsprd {
        &self.fsprd
    }
    #[doc = "0x2008 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fedacctl1(&self) -> &Fedacctl1 {
        &self.fedacctl1
    }
    #[doc = "0x200c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fedacctl2(&self) -> &Fedacctl2 {
        &self.fedacctl2
    }
    #[doc = "0x2010 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcor_err_cnt(&self) -> &FcorErrCnt {
        &self.fcor_err_cnt
    }
    #[doc = "0x2014 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcor_err_add(&self) -> &FcorErrAdd {
        &self.fcor_err_add
    }
    #[doc = "0x2018 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcor_err_pos(&self) -> &FcorErrPos {
        &self.fcor_err_pos
    }
    #[doc = "0x201c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fedacstat(&self) -> &Fedacstat {
        &self.fedacstat
    }
    #[doc = "0x2020 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn func_err_add(&self) -> &FuncErrAdd {
        &self.func_err_add
    }
    #[doc = "0x2024 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fedacsdis(&self) -> &Fedacsdis {
        &self.fedacsdis
    }
    #[doc = "0x2028 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fprim_add_tag(&self) -> &FprimAddTag {
        &self.fprim_add_tag
    }
    #[doc = "0x202c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fredu_add_tag(&self) -> &FreduAddTag {
        &self.fredu_add_tag
    }
    #[doc = "0x2030 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fbprot(&self) -> &Fbprot {
        &self.fbprot
    }
    #[doc = "0x2034 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fbse(&self) -> &Fbse {
        &self.fbse
    }
    #[doc = "0x2038 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fbbusy(&self) -> &Fbbusy {
        &self.fbbusy
    }
    #[doc = "0x203c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fbac(&self) -> &Fbac {
        &self.fbac
    }
    #[doc = "0x2040 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fbfallback(&self) -> &Fbfallback {
        &self.fbfallback
    }
    #[doc = "0x2044 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fbprdy(&self) -> &Fbprdy {
        &self.fbprdy
    }
    #[doc = "0x2048 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fpac1(&self) -> &Fpac1 {
        &self.fpac1
    }
    #[doc = "0x204c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fpac2(&self) -> &Fpac2 {
        &self.fpac2
    }
    #[doc = "0x2050 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fmac(&self) -> &Fmac {
        &self.fmac
    }
    #[doc = "0x2054 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fmstat(&self) -> &Fmstat {
        &self.fmstat
    }
    #[doc = "0x2058 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn femu_dmsw(&self) -> &FemuDmsw {
        &self.femu_dmsw
    }
    #[doc = "0x205c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn femu_dlsw(&self) -> &FemuDlsw {
        &self.femu_dlsw
    }
    #[doc = "0x2060 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn femu_ecc(&self) -> &FemuEcc {
        &self.femu_ecc
    }
    #[doc = "0x2064 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flock(&self) -> &Flock {
        &self.flock
    }
    #[doc = "0x2068 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn femu_addr(&self) -> &FemuAddr {
        &self.femu_addr
    }
    #[doc = "0x206c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fdiagctl(&self) -> &Fdiagctl {
        &self.fdiagctl
    }
    #[doc = "0x2070 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fraw_datah(&self) -> &FrawDatah {
        &self.fraw_datah
    }
    #[doc = "0x2074 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fraw_datal(&self) -> &FrawDatal {
        &self.fraw_datal
    }
    #[doc = "0x2078 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fraw_ecc(&self) -> &FrawEcc {
        &self.fraw_ecc
    }
    #[doc = "0x207c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fpar_ovr(&self) -> &FparOvr {
        &self.fpar_ovr
    }
    #[doc = "0x2080 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fvreadct(&self) -> &Fvreadct {
        &self.fvreadct
    }
    #[doc = "0x2084 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fvhvct1(&self) -> &Fvhvct1 {
        &self.fvhvct1
    }
    #[doc = "0x2088 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fvhvct2(&self) -> &Fvhvct2 {
        &self.fvhvct2
    }
    #[doc = "0x208c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fvhvct3(&self) -> &Fvhvct3 {
        &self.fvhvct3
    }
    #[doc = "0x2090 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fvnvct(&self) -> &Fvnvct {
        &self.fvnvct
    }
    #[doc = "0x2094 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fvslp(&self) -> &Fvslp {
        &self.fvslp
    }
    #[doc = "0x2098 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fvwlct(&self) -> &Fvwlct {
        &self.fvwlct
    }
    #[doc = "0x209c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fefusectl(&self) -> &Fefusectl {
        &self.fefusectl
    }
    #[doc = "0x20a0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fefusestat(&self) -> &Fefusestat {
        &self.fefusestat
    }
    #[doc = "0x20a4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fefusedata(&self) -> &Fefusedata {
        &self.fefusedata
    }
    #[doc = "0x20a8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fseqpmp(&self) -> &Fseqpmp {
        &self.fseqpmp
    }
    #[doc = "0x20ac - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fclktrim(&self) -> &Fclktrim {
        &self.fclktrim
    }
    #[doc = "0x20b0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn rom_test(&self) -> &RomTest {
        &self.rom_test
    }
    #[doc = "0x20c0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fedacsdis2(&self) -> &Fedacsdis2 {
        &self.fedacsdis2
    }
    #[doc = "0x2100 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fbstrobes(&self) -> &Fbstrobes {
        &self.fbstrobes
    }
    #[doc = "0x2104 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fpstrobes(&self) -> &Fpstrobes {
        &self.fpstrobes
    }
    #[doc = "0x2108 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fbmode(&self) -> &Fbmode {
        &self.fbmode
    }
    #[doc = "0x210c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ftcr(&self) -> &Ftcr {
        &self.ftcr
    }
    #[doc = "0x2110 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn faddr(&self) -> &Faddr {
        &self.faddr
    }
    #[doc = "0x2114 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fpmtctl(&self) -> &Fpmtctl {
        &self.fpmtctl
    }
    #[doc = "0x2118 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn pbistctl(&self) -> &Pbistctl {
        &self.pbistctl
    }
    #[doc = "0x211c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ftctl(&self) -> &Ftctl {
        &self.ftctl
    }
    #[doc = "0x2120 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite0(&self) -> &Fwpwrite0 {
        &self.fwpwrite0
    }
    #[doc = "0x2124 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite1(&self) -> &Fwpwrite1 {
        &self.fwpwrite1
    }
    #[doc = "0x2128 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite2(&self) -> &Fwpwrite2 {
        &self.fwpwrite2
    }
    #[doc = "0x212c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite3(&self) -> &Fwpwrite3 {
        &self.fwpwrite3
    }
    #[doc = "0x2130 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite4(&self) -> &Fwpwrite4 {
        &self.fwpwrite4
    }
    #[doc = "0x2134 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite5(&self) -> &Fwpwrite5 {
        &self.fwpwrite5
    }
    #[doc = "0x2138 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite6(&self) -> &Fwpwrite6 {
        &self.fwpwrite6
    }
    #[doc = "0x213c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite7(&self) -> &Fwpwrite7 {
        &self.fwpwrite7
    }
    #[doc = "0x2140 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fwpwrite_ecc(&self) -> &FwpwriteEcc {
        &self.fwpwrite_ecc
    }
    #[doc = "0x2144 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fswstat(&self) -> &Fswstat {
        &self.fswstat
    }
    #[doc = "0x2200 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_glbctl(&self) -> &FsmGlbctl {
        &self.fsm_glbctl
    }
    #[doc = "0x2204 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_state(&self) -> &FsmState {
        &self.fsm_state
    }
    #[doc = "0x2208 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_stat(&self) -> &FsmStat {
        &self.fsm_stat
    }
    #[doc = "0x220c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_cmd(&self) -> &FsmCmd {
        &self.fsm_cmd
    }
    #[doc = "0x2210 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_pe_osu(&self) -> &FsmPeOsu {
        &self.fsm_pe_osu
    }
    #[doc = "0x2214 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_vstat(&self) -> &FsmVstat {
        &self.fsm_vstat
    }
    #[doc = "0x2218 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_pe_vsu(&self) -> &FsmPeVsu {
        &self.fsm_pe_vsu
    }
    #[doc = "0x221c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_cmp_vsu(&self) -> &FsmCmpVsu {
        &self.fsm_cmp_vsu
    }
    #[doc = "0x2220 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_ex_val(&self) -> &FsmExVal {
        &self.fsm_ex_val
    }
    #[doc = "0x2224 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_rd_h(&self) -> &FsmRdH {
        &self.fsm_rd_h
    }
    #[doc = "0x2228 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_p_oh(&self) -> &FsmPOh {
        &self.fsm_p_oh
    }
    #[doc = "0x222c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_era_oh(&self) -> &FsmEraOh {
        &self.fsm_era_oh
    }
    #[doc = "0x2230 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_sav_ppul(&self) -> &FsmSavPpul {
        &self.fsm_sav_ppul
    }
    #[doc = "0x2234 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_pe_vh(&self) -> &FsmPeVh {
        &self.fsm_pe_vh
    }
    #[doc = "0x2240 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_prg_pw(&self) -> &FsmPrgPw {
        &self.fsm_prg_pw
    }
    #[doc = "0x2244 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_era_pw(&self) -> &FsmEraPw {
        &self.fsm_era_pw
    }
    #[doc = "0x2254 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_sav_era_pul(&self) -> &FsmSavEraPul {
        &self.fsm_sav_era_pul
    }
    #[doc = "0x2258 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_timer(&self) -> &FsmTimer {
        &self.fsm_timer
    }
    #[doc = "0x225c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_mode(&self) -> &FsmMode {
        &self.fsm_mode
    }
    #[doc = "0x2260 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_pgm(&self) -> &FsmPgm {
        &self.fsm_pgm
    }
    #[doc = "0x2264 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_era(&self) -> &FsmEra {
        &self.fsm_era
    }
    #[doc = "0x2268 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_prg_pul(&self) -> &FsmPrgPul {
        &self.fsm_prg_pul
    }
    #[doc = "0x226c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_era_pul(&self) -> &FsmEraPul {
        &self.fsm_era_pul
    }
    #[doc = "0x2270 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_step_size(&self) -> &FsmStepSize {
        &self.fsm_step_size
    }
    #[doc = "0x2274 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_pul_cntr(&self) -> &FsmPulCntr {
        &self.fsm_pul_cntr
    }
    #[doc = "0x2278 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_ec_step_height(&self) -> &FsmEcStepHeight {
        &self.fsm_ec_step_height
    }
    #[doc = "0x227c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_st_machine(&self) -> &FsmStMachine {
        &self.fsm_st_machine
    }
    #[doc = "0x2280 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_fles(&self) -> &FsmFles {
        &self.fsm_fles
    }
    #[doc = "0x2288 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_wr_ena(&self) -> &FsmWrEna {
        &self.fsm_wr_ena
    }
    #[doc = "0x228c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_acc_pp(&self) -> &FsmAccPp {
        &self.fsm_acc_pp
    }
    #[doc = "0x2290 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_acc_ep(&self) -> &FsmAccEp {
        &self.fsm_acc_ep
    }
    #[doc = "0x22a0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_addr(&self) -> &FsmAddr {
        &self.fsm_addr
    }
    #[doc = "0x22a4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_sector(&self) -> &FsmSector {
        &self.fsm_sector
    }
    #[doc = "0x22a8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fmc_rev_id(&self) -> &FmcRevId {
        &self.fmc_rev_id
    }
    #[doc = "0x22ac - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_err_addr(&self) -> &FsmErrAddr {
        &self.fsm_err_addr
    }
    #[doc = "0x22b0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_pgm_maxpul(&self) -> &FsmPgmMaxpul {
        &self.fsm_pgm_maxpul
    }
    #[doc = "0x22b4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_execute(&self) -> &FsmExecute {
        &self.fsm_execute
    }
    #[doc = "0x22b8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn eeprom_cfg(&self) -> &EepromCfg {
        &self.eeprom_cfg
    }
    #[doc = "0x22c0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_sector1(&self) -> &FsmSector1 {
        &self.fsm_sector1
    }
    #[doc = "0x22c4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_sector2(&self) -> &FsmSector2 {
        &self.fsm_sector2
    }
    #[doc = "0x22e0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_bsle0(&self) -> &FsmBsle0 {
        &self.fsm_bsle0
    }
    #[doc = "0x22e4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_bsle1(&self) -> &FsmBsle1 {
        &self.fsm_bsle1
    }
    #[doc = "0x22f0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_bslp0(&self) -> &FsmBslp0 {
        &self.fsm_bslp0
    }
    #[doc = "0x22f4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fsm_bslp1(&self) -> &FsmBslp1 {
        &self.fsm_bslp1
    }
    #[doc = "0x2400 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_bank(&self) -> &FcfgBank {
        &self.fcfg_bank
    }
    #[doc = "0x2404 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_wrapper(&self) -> &FcfgWrapper {
        &self.fcfg_wrapper
    }
    #[doc = "0x2408 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_bnk_type(&self) -> &FcfgBnkType {
        &self.fcfg_bnk_type
    }
    #[doc = "0x2410 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b0_start(&self) -> &FcfgB0Start {
        &self.fcfg_b0_start
    }
    #[doc = "0x2414 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b1_start(&self) -> &FcfgB1Start {
        &self.fcfg_b1_start
    }
    #[doc = "0x2418 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b2_start(&self) -> &FcfgB2Start {
        &self.fcfg_b2_start
    }
    #[doc = "0x241c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b3_start(&self) -> &FcfgB3Start {
        &self.fcfg_b3_start
    }
    #[doc = "0x2420 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b4_start(&self) -> &FcfgB4Start {
        &self.fcfg_b4_start
    }
    #[doc = "0x2424 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b5_start(&self) -> &FcfgB5Start {
        &self.fcfg_b5_start
    }
    #[doc = "0x2428 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b6_start(&self) -> &FcfgB6Start {
        &self.fcfg_b6_start
    }
    #[doc = "0x242c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b7_start(&self) -> &FcfgB7Start {
        &self.fcfg_b7_start
    }
    #[doc = "0x2430 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b0_ssize0(&self) -> &FcfgB0Ssize0 {
        &self.fcfg_b0_ssize0
    }
    #[doc = "0x2434 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b0_ssize1(&self) -> &FcfgB0Ssize1 {
        &self.fcfg_b0_ssize1
    }
    #[doc = "0x2438 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b0_ssize2(&self) -> &FcfgB0Ssize2 {
        &self.fcfg_b0_ssize2
    }
    #[doc = "0x243c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b0_ssize3(&self) -> &FcfgB0Ssize3 {
        &self.fcfg_b0_ssize3
    }
    #[doc = "0x2440 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b1_ssize0(&self) -> &FcfgB1Ssize0 {
        &self.fcfg_b1_ssize0
    }
    #[doc = "0x2444 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b1_ssize1(&self) -> &FcfgB1Ssize1 {
        &self.fcfg_b1_ssize1
    }
    #[doc = "0x2448 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b1_ssize2(&self) -> &FcfgB1Ssize2 {
        &self.fcfg_b1_ssize2
    }
    #[doc = "0x244c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b1_ssize3(&self) -> &FcfgB1Ssize3 {
        &self.fcfg_b1_ssize3
    }
    #[doc = "0x2450 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b2_ssize0(&self) -> &FcfgB2Ssize0 {
        &self.fcfg_b2_ssize0
    }
    #[doc = "0x2454 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b2_ssize1(&self) -> &FcfgB2Ssize1 {
        &self.fcfg_b2_ssize1
    }
    #[doc = "0x2458 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b2_ssize2(&self) -> &FcfgB2Ssize2 {
        &self.fcfg_b2_ssize2
    }
    #[doc = "0x245c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b2_ssize3(&self) -> &FcfgB2Ssize3 {
        &self.fcfg_b2_ssize3
    }
    #[doc = "0x2460 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b3_ssize0(&self) -> &FcfgB3Ssize0 {
        &self.fcfg_b3_ssize0
    }
    #[doc = "0x2464 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b3_ssize1(&self) -> &FcfgB3Ssize1 {
        &self.fcfg_b3_ssize1
    }
    #[doc = "0x2468 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b3_ssize2(&self) -> &FcfgB3Ssize2 {
        &self.fcfg_b3_ssize2
    }
    #[doc = "0x246c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b3_ssize3(&self) -> &FcfgB3Ssize3 {
        &self.fcfg_b3_ssize3
    }
    #[doc = "0x2470 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b4_ssize0(&self) -> &FcfgB4Ssize0 {
        &self.fcfg_b4_ssize0
    }
    #[doc = "0x2474 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b4_ssize1(&self) -> &FcfgB4Ssize1 {
        &self.fcfg_b4_ssize1
    }
    #[doc = "0x2478 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b4_ssize2(&self) -> &FcfgB4Ssize2 {
        &self.fcfg_b4_ssize2
    }
    #[doc = "0x247c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b4_ssize3(&self) -> &FcfgB4Ssize3 {
        &self.fcfg_b4_ssize3
    }
    #[doc = "0x2480 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b5_ssize0(&self) -> &FcfgB5Ssize0 {
        &self.fcfg_b5_ssize0
    }
    #[doc = "0x2484 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b5_ssize1(&self) -> &FcfgB5Ssize1 {
        &self.fcfg_b5_ssize1
    }
    #[doc = "0x2488 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b5_ssize2(&self) -> &FcfgB5Ssize2 {
        &self.fcfg_b5_ssize2
    }
    #[doc = "0x248c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b5_ssize3(&self) -> &FcfgB5Ssize3 {
        &self.fcfg_b5_ssize3
    }
    #[doc = "0x2490 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b6_ssize0(&self) -> &FcfgB6Ssize0 {
        &self.fcfg_b6_ssize0
    }
    #[doc = "0x2494 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b6_ssize1(&self) -> &FcfgB6Ssize1 {
        &self.fcfg_b6_ssize1
    }
    #[doc = "0x2498 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b6_ssize2(&self) -> &FcfgB6Ssize2 {
        &self.fcfg_b6_ssize2
    }
    #[doc = "0x249c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b6_ssize3(&self) -> &FcfgB6Ssize3 {
        &self.fcfg_b6_ssize3
    }
    #[doc = "0x24a0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b7_ssize0(&self) -> &FcfgB7Ssize0 {
        &self.fcfg_b7_ssize0
    }
    #[doc = "0x24a4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b7_ssize1(&self) -> &FcfgB7Ssize1 {
        &self.fcfg_b7_ssize1
    }
    #[doc = "0x24a8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b7_ssize2(&self) -> &FcfgB7Ssize2 {
        &self.fcfg_b7_ssize2
    }
    #[doc = "0x24ac - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn fcfg_b7_ssize3(&self) -> &FcfgB7Ssize3 {
        &self.fcfg_b7_ssize3
    }
}
#[doc = "STAT (rw) register accessor: FMC and Efuse Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "FMC and Efuse Status"]
pub mod stat;
#[doc = "CFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cfg;
#[doc = "SYSCODE_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscode_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscode_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscode_start`]
module"]
#[doc(alias = "SYSCODE_START")]
pub type SyscodeStart = crate::Reg<syscode_start::SyscodeStartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod syscode_start;
#[doc = "FLASH_SIZE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_size`]
module"]
#[doc(alias = "FLASH_SIZE")]
pub type FlashSize = crate::Reg<flash_size::FlashSizeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_size;
#[doc = "FWLOCK (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwlock`]
module"]
#[doc(alias = "FWLOCK")]
pub type Fwlock = crate::Reg<fwlock::FwlockSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwlock;
#[doc = "FWFLAG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwflag`]
module"]
#[doc(alias = "FWFLAG")]
pub type Fwflag = crate::Reg<fwflag::FwflagSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwflag;
#[doc = "EFUSE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuse`]
module"]
#[doc(alias = "EFUSE")]
pub type Efuse = crate::Reg<efuse::EfuseSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuse;
#[doc = "EFUSEADDR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseaddr`]
module"]
#[doc(alias = "EFUSEADDR")]
pub type Efuseaddr = crate::Reg<efuseaddr::EfuseaddrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseaddr;
#[doc = "DATAUPPER (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataupper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataupper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataupper`]
module"]
#[doc(alias = "DATAUPPER")]
pub type Dataupper = crate::Reg<dataupper::DataupperSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dataupper;
#[doc = "DATALOWER (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datalower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datalower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datalower`]
module"]
#[doc(alias = "DATALOWER")]
pub type Datalower = crate::Reg<datalower::DatalowerSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod datalower;
#[doc = "EFUSECFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusecfg`]
module"]
#[doc(alias = "EFUSECFG")]
pub type Efusecfg = crate::Reg<efusecfg::EfusecfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecfg;
#[doc = "EFUSESTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusestat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusestat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusestat`]
module"]
#[doc(alias = "EFUSESTAT")]
pub type Efusestat = crate::Reg<efusestat::EfusestatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusestat;
#[doc = "ACC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acc`]
module"]
#[doc(alias = "ACC")]
pub type Acc = crate::Reg<acc::AccSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod acc;
#[doc = "BOUNDARY (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boundary::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boundary::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boundary`]
module"]
#[doc(alias = "BOUNDARY")]
pub type Boundary = crate::Reg<boundary::BoundarySpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod boundary;
#[doc = "EFUSEFLAG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseflag`]
module"]
#[doc(alias = "EFUSEFLAG")]
pub type Efuseflag = crate::Reg<efuseflag::EfuseflagSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseflag;
#[doc = "EFUSEKEY (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusekey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusekey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusekey`]
module"]
#[doc(alias = "EFUSEKEY")]
pub type Efusekey = crate::Reg<efusekey::EfusekeySpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusekey;
#[doc = "EFUSERELEASE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuserelease::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuserelease::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuserelease`]
module"]
#[doc(alias = "EFUSERELEASE")]
pub type Efuserelease = crate::Reg<efuserelease::EfusereleaseSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuserelease;
#[doc = "EFUSEPINS (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusepins::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusepins::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusepins`]
module"]
#[doc(alias = "EFUSEPINS")]
pub type Efusepins = crate::Reg<efusepins::EfusepinsSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusepins;
#[doc = "EFUSECRA (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusecra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusecra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efusecra`]
module"]
#[doc(alias = "EFUSECRA")]
pub type Efusecra = crate::Reg<efusecra::EfusecraSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecra;
#[doc = "EFUSEREAD (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseread::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseread::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseread`]
module"]
#[doc(alias = "EFUSEREAD")]
pub type Efuseread = crate::Reg<efuseread::EfusereadSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseread;
#[doc = "EFUSEPROGRAM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseprogram::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseprogram::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseprogram`]
module"]
#[doc(alias = "EFUSEPROGRAM")]
pub type Efuseprogram = crate::Reg<efuseprogram::EfuseprogramSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseprogram;
#[doc = "EFUSEERROR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseerror::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseerror::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efuseerror`]
module"]
#[doc(alias = "EFUSEERROR")]
pub type Efuseerror = crate::Reg<efuseerror::EfuseerrorSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseerror;
#[doc = "SINGLEBIT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlebit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlebit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlebit`]
module"]
#[doc(alias = "SINGLEBIT")]
pub type Singlebit = crate::Reg<singlebit::SinglebitSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod singlebit;
#[doc = "TWOBIT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twobit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twobit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@twobit`]
module"]
#[doc(alias = "TWOBIT")]
pub type Twobit = crate::Reg<twobit::TwobitSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod twobit;
#[doc = "SELFTESTCYC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selftestcyc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selftestcyc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selftestcyc`]
module"]
#[doc(alias = "SELFTESTCYC")]
pub type Selftestcyc = crate::Reg<selftestcyc::SelftestcycSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestcyc;
#[doc = "SELFTESTSIGN (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selftestsign::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selftestsign::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@selftestsign`]
module"]
#[doc(alias = "SELFTESTSIGN")]
pub type Selftestsign = crate::Reg<selftestsign::SelftestsignSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestsign;
#[doc = "FRDCTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frdctl`]
module"]
#[doc(alias = "FRDCTL")]
pub type Frdctl = crate::Reg<frdctl::FrdctlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod frdctl;
#[doc = "FSPRD (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsprd`]
module"]
#[doc(alias = "FSPRD")]
pub type Fsprd = crate::Reg<fsprd::FsprdSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsprd;
#[doc = "FEDACCTL1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fedacctl1`]
module"]
#[doc(alias = "FEDACCTL1")]
pub type Fedacctl1 = crate::Reg<fedacctl1::Fedacctl1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacctl1;
#[doc = "FEDACCTL2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fedacctl2`]
module"]
#[doc(alias = "FEDACCTL2")]
pub type Fedacctl2 = crate::Reg<fedacctl2::Fedacctl2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacctl2;
#[doc = "FCOR_ERR_CNT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcor_err_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcor_err_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcor_err_cnt`]
module"]
#[doc(alias = "FCOR_ERR_CNT")]
pub type FcorErrCnt = crate::Reg<fcor_err_cnt::FcorErrCntSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_cnt;
#[doc = "FCOR_ERR_ADD (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcor_err_add::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcor_err_add::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcor_err_add`]
module"]
#[doc(alias = "FCOR_ERR_ADD")]
pub type FcorErrAdd = crate::Reg<fcor_err_add::FcorErrAddSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_add;
#[doc = "FCOR_ERR_POS (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcor_err_pos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcor_err_pos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcor_err_pos`]
module"]
#[doc(alias = "FCOR_ERR_POS")]
pub type FcorErrPos = crate::Reg<fcor_err_pos::FcorErrPosSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_pos;
#[doc = "FEDACSTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fedacstat`]
module"]
#[doc(alias = "FEDACSTAT")]
pub type Fedacstat = crate::Reg<fedacstat::FedacstatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacstat;
#[doc = "FUNC_ERR_ADD (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_err_add::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_err_add::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_err_add`]
module"]
#[doc(alias = "FUNC_ERR_ADD")]
pub type FuncErrAdd = crate::Reg<func_err_add::FuncErrAddSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod func_err_add;
#[doc = "FEDACSDIS (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacsdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacsdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fedacsdis`]
module"]
#[doc(alias = "FEDACSDIS")]
pub type Fedacsdis = crate::Reg<fedacsdis::FedacsdisSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacsdis;
#[doc = "FPRIM_ADD_TAG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fprim_add_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fprim_add_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fprim_add_tag`]
module"]
#[doc(alias = "FPRIM_ADD_TAG")]
pub type FprimAddTag = crate::Reg<fprim_add_tag::FprimAddTagSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fprim_add_tag;
#[doc = "FREDU_ADD_TAG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fredu_add_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fredu_add_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fredu_add_tag`]
module"]
#[doc(alias = "FREDU_ADD_TAG")]
pub type FreduAddTag = crate::Reg<fredu_add_tag::FreduAddTagSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fredu_add_tag;
#[doc = "FBPROT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbprot::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbprot::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbprot`]
module"]
#[doc(alias = "FBPROT")]
pub type Fbprot = crate::Reg<fbprot::FbprotSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbprot;
#[doc = "FBSE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbse`]
module"]
#[doc(alias = "FBSE")]
pub type Fbse = crate::Reg<fbse::FbseSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbse;
#[doc = "FBBUSY (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbbusy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbbusy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbbusy`]
module"]
#[doc(alias = "FBBUSY")]
pub type Fbbusy = crate::Reg<fbbusy::FbbusySpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbbusy;
#[doc = "FBAC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbac`]
module"]
#[doc(alias = "FBAC")]
pub type Fbac = crate::Reg<fbac::FbacSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbac;
#[doc = "FBFALLBACK (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbfallback::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbfallback::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbfallback`]
module"]
#[doc(alias = "FBFALLBACK")]
pub type Fbfallback = crate::Reg<fbfallback::FbfallbackSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbfallback;
#[doc = "FBPRDY (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbprdy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbprdy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbprdy`]
module"]
#[doc(alias = "FBPRDY")]
pub type Fbprdy = crate::Reg<fbprdy::FbprdySpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbprdy;
#[doc = "FPAC1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpac1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpac1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpac1`]
module"]
#[doc(alias = "FPAC1")]
pub type Fpac1 = crate::Reg<fpac1::Fpac1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpac1;
#[doc = "FPAC2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpac2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpac2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpac2`]
module"]
#[doc(alias = "FPAC2")]
pub type Fpac2 = crate::Reg<fpac2::Fpac2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpac2;
#[doc = "FMAC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmac`]
module"]
#[doc(alias = "FMAC")]
pub type Fmac = crate::Reg<fmac::FmacSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmac;
#[doc = "FMSTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmstat`]
module"]
#[doc(alias = "FMSTAT")]
pub type Fmstat = crate::Reg<fmstat::FmstatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmstat;
#[doc = "FEMU_DMSW (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`femu_dmsw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`femu_dmsw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@femu_dmsw`]
module"]
#[doc(alias = "FEMU_DMSW")]
pub type FemuDmsw = crate::Reg<femu_dmsw::FemuDmswSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_dmsw;
#[doc = "FEMU_DLSW (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`femu_dlsw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`femu_dlsw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@femu_dlsw`]
module"]
#[doc(alias = "FEMU_DLSW")]
pub type FemuDlsw = crate::Reg<femu_dlsw::FemuDlswSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_dlsw;
#[doc = "FEMU_ECC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`femu_ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`femu_ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@femu_ecc`]
module"]
#[doc(alias = "FEMU_ECC")]
pub type FemuEcc = crate::Reg<femu_ecc::FemuEccSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_ecc;
#[doc = "FLOCK (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flock`]
module"]
#[doc(alias = "FLOCK")]
pub type Flock = crate::Reg<flock::FlockSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flock;
#[doc = "FEMU_ADDR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`femu_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`femu_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@femu_addr`]
module"]
#[doc(alias = "FEMU_ADDR")]
pub type FemuAddr = crate::Reg<femu_addr::FemuAddrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_addr;
#[doc = "FDIAGCTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdiagctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdiagctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdiagctl`]
module"]
#[doc(alias = "FDIAGCTL")]
pub type Fdiagctl = crate::Reg<fdiagctl::FdiagctlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fdiagctl;
#[doc = "FRAW_DATAH (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fraw_datah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fraw_datah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fraw_datah`]
module"]
#[doc(alias = "FRAW_DATAH")]
pub type FrawDatah = crate::Reg<fraw_datah::FrawDatahSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_datah;
#[doc = "FRAW_DATAL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fraw_datal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fraw_datal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fraw_datal`]
module"]
#[doc(alias = "FRAW_DATAL")]
pub type FrawDatal = crate::Reg<fraw_datal::FrawDatalSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_datal;
#[doc = "FRAW_ECC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fraw_ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fraw_ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fraw_ecc`]
module"]
#[doc(alias = "FRAW_ECC")]
pub type FrawEcc = crate::Reg<fraw_ecc::FrawEccSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_ecc;
#[doc = "FPAR_OVR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpar_ovr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpar_ovr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpar_ovr`]
module"]
#[doc(alias = "FPAR_OVR")]
pub type FparOvr = crate::Reg<fpar_ovr::FparOvrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpar_ovr;
#[doc = "FVREADCT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvreadct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvreadct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fvreadct`]
module"]
#[doc(alias = "FVREADCT")]
pub type Fvreadct = crate::Reg<fvreadct::FvreadctSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvreadct;
#[doc = "FVHVCT1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvhvct1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvhvct1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fvhvct1`]
module"]
#[doc(alias = "FVHVCT1")]
pub type Fvhvct1 = crate::Reg<fvhvct1::Fvhvct1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct1;
#[doc = "FVHVCT2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvhvct2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvhvct2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fvhvct2`]
module"]
#[doc(alias = "FVHVCT2")]
pub type Fvhvct2 = crate::Reg<fvhvct2::Fvhvct2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct2;
#[doc = "FVHVCT3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvhvct3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvhvct3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fvhvct3`]
module"]
#[doc(alias = "FVHVCT3")]
pub type Fvhvct3 = crate::Reg<fvhvct3::Fvhvct3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct3;
#[doc = "FVNVCT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvnvct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvnvct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fvnvct`]
module"]
#[doc(alias = "FVNVCT")]
pub type Fvnvct = crate::Reg<fvnvct::FvnvctSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvnvct;
#[doc = "FVSLP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvslp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvslp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fvslp`]
module"]
#[doc(alias = "FVSLP")]
pub type Fvslp = crate::Reg<fvslp::FvslpSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvslp;
#[doc = "FVWLCT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvwlct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvwlct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fvwlct`]
module"]
#[doc(alias = "FVWLCT")]
pub type Fvwlct = crate::Reg<fvwlct::FvwlctSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvwlct;
#[doc = "FEFUSECTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fefusectl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fefusectl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fefusectl`]
module"]
#[doc(alias = "FEFUSECTL")]
pub type Fefusectl = crate::Reg<fefusectl::FefusectlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusectl;
#[doc = "FEFUSESTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fefusestat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fefusestat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fefusestat`]
module"]
#[doc(alias = "FEFUSESTAT")]
pub type Fefusestat = crate::Reg<fefusestat::FefusestatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusestat;
#[doc = "FEFUSEDATA (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fefusedata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fefusedata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fefusedata`]
module"]
#[doc(alias = "FEFUSEDATA")]
pub type Fefusedata = crate::Reg<fefusedata::FefusedataSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusedata;
#[doc = "FSEQPMP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fseqpmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fseqpmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fseqpmp`]
module"]
#[doc(alias = "FSEQPMP")]
pub type Fseqpmp = crate::Reg<fseqpmp::FseqpmpSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fseqpmp;
#[doc = "FCLKTRIM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fclktrim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fclktrim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fclktrim`]
module"]
#[doc(alias = "FCLKTRIM")]
pub type Fclktrim = crate::Reg<fclktrim::FclktrimSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fclktrim;
#[doc = "ROM_TEST (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_test`]
module"]
#[doc(alias = "ROM_TEST")]
pub type RomTest = crate::Reg<rom_test::RomTestSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rom_test;
#[doc = "FEDACSDIS2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacsdis2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacsdis2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fedacsdis2`]
module"]
#[doc(alias = "FEDACSDIS2")]
pub type Fedacsdis2 = crate::Reg<fedacsdis2::Fedacsdis2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacsdis2;
#[doc = "FBSTROBES (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbstrobes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbstrobes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbstrobes`]
module"]
#[doc(alias = "FBSTROBES")]
pub type Fbstrobes = crate::Reg<fbstrobes::FbstrobesSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbstrobes;
#[doc = "FPSTROBES (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpstrobes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpstrobes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpstrobes`]
module"]
#[doc(alias = "FPSTROBES")]
pub type Fpstrobes = crate::Reg<fpstrobes::FpstrobesSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpstrobes;
#[doc = "FBMODE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbmode`]
module"]
#[doc(alias = "FBMODE")]
pub type Fbmode = crate::Reg<fbmode::FbmodeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbmode;
#[doc = "FTCR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftcr`]
module"]
#[doc(alias = "FTCR")]
pub type Ftcr = crate::Reg<ftcr::FtcrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ftcr;
#[doc = "FADDR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`faddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faddr`]
module"]
#[doc(alias = "FADDR")]
pub type Faddr = crate::Reg<faddr::FaddrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod faddr;
#[doc = "FPMTCTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpmtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpmtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpmtctl`]
module"]
#[doc(alias = "FPMTCTL")]
pub type Fpmtctl = crate::Reg<fpmtctl::FpmtctlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpmtctl;
#[doc = "PBISTCTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbistctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbistctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbistctl`]
module"]
#[doc(alias = "PBISTCTL")]
pub type Pbistctl = crate::Reg<pbistctl::PbistctlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pbistctl;
#[doc = "FTCTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftctl`]
module"]
#[doc(alias = "FTCTL")]
pub type Ftctl = crate::Reg<ftctl::FtctlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ftctl;
#[doc = "FWPWRITE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite0`]
module"]
#[doc(alias = "FWPWRITE0")]
pub type Fwpwrite0 = crate::Reg<fwpwrite0::Fwpwrite0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite0;
#[doc = "FWPWRITE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite1`]
module"]
#[doc(alias = "FWPWRITE1")]
pub type Fwpwrite1 = crate::Reg<fwpwrite1::Fwpwrite1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite1;
#[doc = "FWPWRITE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite2`]
module"]
#[doc(alias = "FWPWRITE2")]
pub type Fwpwrite2 = crate::Reg<fwpwrite2::Fwpwrite2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite2;
#[doc = "FWPWRITE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite3`]
module"]
#[doc(alias = "FWPWRITE3")]
pub type Fwpwrite3 = crate::Reg<fwpwrite3::Fwpwrite3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite3;
#[doc = "FWPWRITE4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite4`]
module"]
#[doc(alias = "FWPWRITE4")]
pub type Fwpwrite4 = crate::Reg<fwpwrite4::Fwpwrite4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite4;
#[doc = "FWPWRITE5 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite5`]
module"]
#[doc(alias = "FWPWRITE5")]
pub type Fwpwrite5 = crate::Reg<fwpwrite5::Fwpwrite5Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite5;
#[doc = "FWPWRITE6 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite6`]
module"]
#[doc(alias = "FWPWRITE6")]
pub type Fwpwrite6 = crate::Reg<fwpwrite6::Fwpwrite6Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite6;
#[doc = "FWPWRITE7 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite7`]
module"]
#[doc(alias = "FWPWRITE7")]
pub type Fwpwrite7 = crate::Reg<fwpwrite7::Fwpwrite7Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite7;
#[doc = "FWPWRITE_ECC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite_ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite_ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwpwrite_ecc`]
module"]
#[doc(alias = "FWPWRITE_ECC")]
pub type FwpwriteEcc = crate::Reg<fwpwrite_ecc::FwpwriteEccSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite_ecc;
#[doc = "FSWSTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fswstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fswstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fswstat`]
module"]
#[doc(alias = "FSWSTAT")]
pub type Fswstat = crate::Reg<fswstat::FswstatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fswstat;
#[doc = "FSM_GLBCTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_glbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_glbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_glbctl`]
module"]
#[doc(alias = "FSM_GLBCTL")]
pub type FsmGlbctl = crate::Reg<fsm_glbctl::FsmGlbctlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_glbctl;
#[doc = "FSM_STATE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_state`]
module"]
#[doc(alias = "FSM_STATE")]
pub type FsmState = crate::Reg<fsm_state::FsmStateSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_state;
#[doc = "FSM_STAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_stat`]
module"]
#[doc(alias = "FSM_STAT")]
pub type FsmStat = crate::Reg<fsm_stat::FsmStatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_stat;
#[doc = "FSM_CMD (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_cmd`]
module"]
#[doc(alias = "FSM_CMD")]
pub type FsmCmd = crate::Reg<fsm_cmd::FsmCmdSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_cmd;
#[doc = "FSM_PE_OSU (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pe_osu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pe_osu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_pe_osu`]
module"]
#[doc(alias = "FSM_PE_OSU")]
pub type FsmPeOsu = crate::Reg<fsm_pe_osu::FsmPeOsuSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_osu;
#[doc = "FSM_VSTAT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_vstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_vstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_vstat`]
module"]
#[doc(alias = "FSM_VSTAT")]
pub type FsmVstat = crate::Reg<fsm_vstat::FsmVstatSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_vstat;
#[doc = "FSM_PE_VSU (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pe_vsu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pe_vsu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_pe_vsu`]
module"]
#[doc(alias = "FSM_PE_VSU")]
pub type FsmPeVsu = crate::Reg<fsm_pe_vsu::FsmPeVsuSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_vsu;
#[doc = "FSM_CMP_VSU (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_cmp_vsu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_cmp_vsu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_cmp_vsu`]
module"]
#[doc(alias = "FSM_CMP_VSU")]
pub type FsmCmpVsu = crate::Reg<fsm_cmp_vsu::FsmCmpVsuSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_cmp_vsu;
#[doc = "FSM_EX_VAL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_ex_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_ex_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_ex_val`]
module"]
#[doc(alias = "FSM_EX_VAL")]
pub type FsmExVal = crate::Reg<fsm_ex_val::FsmExValSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_ex_val;
#[doc = "FSM_RD_H (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_rd_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_rd_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_rd_h`]
module"]
#[doc(alias = "FSM_RD_H")]
pub type FsmRdH = crate::Reg<fsm_rd_h::FsmRdHSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_rd_h;
#[doc = "FSM_P_OH (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_p_oh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_p_oh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_p_oh`]
module"]
#[doc(alias = "FSM_P_OH")]
pub type FsmPOh = crate::Reg<fsm_p_oh::FsmPOhSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_p_oh;
#[doc = "FSM_ERA_OH (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_era_oh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_era_oh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_era_oh`]
module"]
#[doc(alias = "FSM_ERA_OH")]
pub type FsmEraOh = crate::Reg<fsm_era_oh::FsmEraOhSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_oh;
#[doc = "FSM_SAV_PPUL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sav_ppul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sav_ppul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_sav_ppul`]
module"]
#[doc(alias = "FSM_SAV_PPUL")]
pub type FsmSavPpul = crate::Reg<fsm_sav_ppul::FsmSavPpulSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sav_ppul;
#[doc = "FSM_PE_VH (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pe_vh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pe_vh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_pe_vh`]
module"]
#[doc(alias = "FSM_PE_VH")]
pub type FsmPeVh = crate::Reg<fsm_pe_vh::FsmPeVhSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_vh;
#[doc = "FSM_PRG_PW (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_prg_pw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_prg_pw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_prg_pw`]
module"]
#[doc(alias = "FSM_PRG_PW")]
pub type FsmPrgPw = crate::Reg<fsm_prg_pw::FsmPrgPwSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_prg_pw;
#[doc = "FSM_ERA_PW (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_era_pw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_era_pw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_era_pw`]
module"]
#[doc(alias = "FSM_ERA_PW")]
pub type FsmEraPw = crate::Reg<fsm_era_pw::FsmEraPwSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_pw;
#[doc = "FSM_SAV_ERA_PUL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sav_era_pul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sav_era_pul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_sav_era_pul`]
module"]
#[doc(alias = "FSM_SAV_ERA_PUL")]
pub type FsmSavEraPul = crate::Reg<fsm_sav_era_pul::FsmSavEraPulSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sav_era_pul;
#[doc = "FSM_TIMER (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_timer`]
module"]
#[doc(alias = "FSM_TIMER")]
pub type FsmTimer = crate::Reg<fsm_timer::FsmTimerSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_timer;
#[doc = "FSM_MODE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_mode`]
module"]
#[doc(alias = "FSM_MODE")]
pub type FsmMode = crate::Reg<fsm_mode::FsmModeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_mode;
#[doc = "FSM_PGM (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pgm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pgm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_pgm`]
module"]
#[doc(alias = "FSM_PGM")]
pub type FsmPgm = crate::Reg<fsm_pgm::FsmPgmSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pgm;
#[doc = "FSM_ERA (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_era::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_era::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_era`]
module"]
#[doc(alias = "FSM_ERA")]
pub type FsmEra = crate::Reg<fsm_era::FsmEraSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era;
#[doc = "FSM_PRG_PUL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_prg_pul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_prg_pul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_prg_pul`]
module"]
#[doc(alias = "FSM_PRG_PUL")]
pub type FsmPrgPul = crate::Reg<fsm_prg_pul::FsmPrgPulSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_prg_pul;
#[doc = "FSM_ERA_PUL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_era_pul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_era_pul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_era_pul`]
module"]
#[doc(alias = "FSM_ERA_PUL")]
pub type FsmEraPul = crate::Reg<fsm_era_pul::FsmEraPulSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_pul;
#[doc = "FSM_STEP_SIZE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_step_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_step_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_step_size`]
module"]
#[doc(alias = "FSM_STEP_SIZE")]
pub type FsmStepSize = crate::Reg<fsm_step_size::FsmStepSizeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_step_size;
#[doc = "FSM_PUL_CNTR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pul_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pul_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_pul_cntr`]
module"]
#[doc(alias = "FSM_PUL_CNTR")]
pub type FsmPulCntr = crate::Reg<fsm_pul_cntr::FsmPulCntrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pul_cntr;
#[doc = "FSM_EC_STEP_HEIGHT (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_ec_step_height::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_ec_step_height::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_ec_step_height`]
module"]
#[doc(alias = "FSM_EC_STEP_HEIGHT")]
pub type FsmEcStepHeight = crate::Reg<fsm_ec_step_height::FsmEcStepHeightSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_ec_step_height;
#[doc = "FSM_ST_MACHINE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_st_machine::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_st_machine::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_st_machine`]
module"]
#[doc(alias = "FSM_ST_MACHINE")]
pub type FsmStMachine = crate::Reg<fsm_st_machine::FsmStMachineSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_st_machine;
#[doc = "FSM_FLES (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_fles::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_fles::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_fles`]
module"]
#[doc(alias = "FSM_FLES")]
pub type FsmFles = crate::Reg<fsm_fles::FsmFlesSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_fles;
#[doc = "FSM_WR_ENA (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_wr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_wr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_wr_ena`]
module"]
#[doc(alias = "FSM_WR_ENA")]
pub type FsmWrEna = crate::Reg<fsm_wr_ena::FsmWrEnaSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_wr_ena;
#[doc = "FSM_ACC_PP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_acc_pp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_acc_pp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_acc_pp`]
module"]
#[doc(alias = "FSM_ACC_PP")]
pub type FsmAccPp = crate::Reg<fsm_acc_pp::FsmAccPpSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_acc_pp;
#[doc = "FSM_ACC_EP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_acc_ep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_acc_ep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_acc_ep`]
module"]
#[doc(alias = "FSM_ACC_EP")]
pub type FsmAccEp = crate::Reg<fsm_acc_ep::FsmAccEpSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_acc_ep;
#[doc = "FSM_ADDR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_addr`]
module"]
#[doc(alias = "FSM_ADDR")]
pub type FsmAddr = crate::Reg<fsm_addr::FsmAddrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_addr;
#[doc = "FSM_SECTOR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sector::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sector::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_sector`]
module"]
#[doc(alias = "FSM_SECTOR")]
pub type FsmSector = crate::Reg<fsm_sector::FsmSectorSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector;
#[doc = "FMC_REV_ID (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_rev_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_rev_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc_rev_id`]
module"]
#[doc(alias = "FMC_REV_ID")]
pub type FmcRevId = crate::Reg<fmc_rev_id::FmcRevIdSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmc_rev_id;
#[doc = "FSM_ERR_ADDR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_err_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_err_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_err_addr`]
module"]
#[doc(alias = "FSM_ERR_ADDR")]
pub type FsmErrAddr = crate::Reg<fsm_err_addr::FsmErrAddrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_err_addr;
#[doc = "FSM_PGM_MAXPUL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pgm_maxpul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pgm_maxpul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_pgm_maxpul`]
module"]
#[doc(alias = "FSM_PGM_MAXPUL")]
pub type FsmPgmMaxpul = crate::Reg<fsm_pgm_maxpul::FsmPgmMaxpulSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pgm_maxpul;
#[doc = "FSM_EXECUTE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_execute::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_execute::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_execute`]
module"]
#[doc(alias = "FSM_EXECUTE")]
pub type FsmExecute = crate::Reg<fsm_execute::FsmExecuteSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_execute;
#[doc = "EEPROM_CFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eeprom_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eeprom_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeprom_cfg`]
module"]
#[doc(alias = "EEPROM_CFG")]
pub type EepromCfg = crate::Reg<eeprom_cfg::EepromCfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod eeprom_cfg;
#[doc = "FSM_SECTOR1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sector1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sector1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_sector1`]
module"]
#[doc(alias = "FSM_SECTOR1")]
pub type FsmSector1 = crate::Reg<fsm_sector1::FsmSector1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector1;
#[doc = "FSM_SECTOR2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_sector2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_sector2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_sector2`]
module"]
#[doc(alias = "FSM_SECTOR2")]
pub type FsmSector2 = crate::Reg<fsm_sector2::FsmSector2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector2;
#[doc = "FSM_BSLE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_bsle0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_bsle0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_bsle0`]
module"]
#[doc(alias = "FSM_BSLE0")]
pub type FsmBsle0 = crate::Reg<fsm_bsle0::FsmBsle0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bsle0;
#[doc = "FSM_BSLE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_bsle1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_bsle1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_bsle1`]
module"]
#[doc(alias = "FSM_BSLE1")]
pub type FsmBsle1 = crate::Reg<fsm_bsle1::FsmBsle1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bsle1;
#[doc = "FSM_BSLP0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_bslp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_bslp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_bslp0`]
module"]
#[doc(alias = "FSM_BSLP0")]
pub type FsmBslp0 = crate::Reg<fsm_bslp0::FsmBslp0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bslp0;
#[doc = "FSM_BSLP1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_bslp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_bslp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsm_bslp1`]
module"]
#[doc(alias = "FSM_BSLP1")]
pub type FsmBslp1 = crate::Reg<fsm_bslp1::FsmBslp1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bslp1;
#[doc = "FCFG_BANK (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_bank::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_bank::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_bank`]
module"]
#[doc(alias = "FCFG_BANK")]
pub type FcfgBank = crate::Reg<fcfg_bank::FcfgBankSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_bank;
#[doc = "FCFG_WRAPPER (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_wrapper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_wrapper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_wrapper`]
module"]
#[doc(alias = "FCFG_WRAPPER")]
pub type FcfgWrapper = crate::Reg<fcfg_wrapper::FcfgWrapperSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_wrapper;
#[doc = "FCFG_BNK_TYPE (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_bnk_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_bnk_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_bnk_type`]
module"]
#[doc(alias = "FCFG_BNK_TYPE")]
pub type FcfgBnkType = crate::Reg<fcfg_bnk_type::FcfgBnkTypeSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_bnk_type;
#[doc = "FCFG_B0_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b0_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b0_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b0_start`]
module"]
#[doc(alias = "FCFG_B0_START")]
pub type FcfgB0Start = crate::Reg<fcfg_b0_start::FcfgB0StartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_start;
#[doc = "FCFG_B1_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b1_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b1_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b1_start`]
module"]
#[doc(alias = "FCFG_B1_START")]
pub type FcfgB1Start = crate::Reg<fcfg_b1_start::FcfgB1StartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_start;
#[doc = "FCFG_B2_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b2_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b2_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b2_start`]
module"]
#[doc(alias = "FCFG_B2_START")]
pub type FcfgB2Start = crate::Reg<fcfg_b2_start::FcfgB2StartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_start;
#[doc = "FCFG_B3_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b3_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b3_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b3_start`]
module"]
#[doc(alias = "FCFG_B3_START")]
pub type FcfgB3Start = crate::Reg<fcfg_b3_start::FcfgB3StartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_start;
#[doc = "FCFG_B4_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b4_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b4_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b4_start`]
module"]
#[doc(alias = "FCFG_B4_START")]
pub type FcfgB4Start = crate::Reg<fcfg_b4_start::FcfgB4StartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_start;
#[doc = "FCFG_B5_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b5_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b5_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b5_start`]
module"]
#[doc(alias = "FCFG_B5_START")]
pub type FcfgB5Start = crate::Reg<fcfg_b5_start::FcfgB5StartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_start;
#[doc = "FCFG_B6_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b6_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b6_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b6_start`]
module"]
#[doc(alias = "FCFG_B6_START")]
pub type FcfgB6Start = crate::Reg<fcfg_b6_start::FcfgB6StartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_start;
#[doc = "FCFG_B7_START (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b7_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b7_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b7_start`]
module"]
#[doc(alias = "FCFG_B7_START")]
pub type FcfgB7Start = crate::Reg<fcfg_b7_start::FcfgB7StartSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_start;
#[doc = "FCFG_B0_SSIZE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b0_ssize0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b0_ssize0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b0_ssize0`]
module"]
#[doc(alias = "FCFG_B0_SSIZE0")]
pub type FcfgB0Ssize0 = crate::Reg<fcfg_b0_ssize0::FcfgB0Ssize0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize0;
#[doc = "FCFG_B0_SSIZE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b0_ssize1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b0_ssize1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b0_ssize1`]
module"]
#[doc(alias = "FCFG_B0_SSIZE1")]
pub type FcfgB0Ssize1 = crate::Reg<fcfg_b0_ssize1::FcfgB0Ssize1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize1;
#[doc = "FCFG_B0_SSIZE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b0_ssize2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b0_ssize2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b0_ssize2`]
module"]
#[doc(alias = "FCFG_B0_SSIZE2")]
pub type FcfgB0Ssize2 = crate::Reg<fcfg_b0_ssize2::FcfgB0Ssize2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize2;
#[doc = "FCFG_B0_SSIZE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b0_ssize3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b0_ssize3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b0_ssize3`]
module"]
#[doc(alias = "FCFG_B0_SSIZE3")]
pub type FcfgB0Ssize3 = crate::Reg<fcfg_b0_ssize3::FcfgB0Ssize3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize3;
#[doc = "FCFG_B1_SSIZE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b1_ssize0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b1_ssize0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b1_ssize0`]
module"]
#[doc(alias = "FCFG_B1_SSIZE0")]
pub type FcfgB1Ssize0 = crate::Reg<fcfg_b1_ssize0::FcfgB1Ssize0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize0;
#[doc = "FCFG_B1_SSIZE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b1_ssize1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b1_ssize1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b1_ssize1`]
module"]
#[doc(alias = "FCFG_B1_SSIZE1")]
pub type FcfgB1Ssize1 = crate::Reg<fcfg_b1_ssize1::FcfgB1Ssize1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize1;
#[doc = "FCFG_B1_SSIZE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b1_ssize2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b1_ssize2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b1_ssize2`]
module"]
#[doc(alias = "FCFG_B1_SSIZE2")]
pub type FcfgB1Ssize2 = crate::Reg<fcfg_b1_ssize2::FcfgB1Ssize2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize2;
#[doc = "FCFG_B1_SSIZE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b1_ssize3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b1_ssize3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b1_ssize3`]
module"]
#[doc(alias = "FCFG_B1_SSIZE3")]
pub type FcfgB1Ssize3 = crate::Reg<fcfg_b1_ssize3::FcfgB1Ssize3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize3;
#[doc = "FCFG_B2_SSIZE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b2_ssize0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b2_ssize0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b2_ssize0`]
module"]
#[doc(alias = "FCFG_B2_SSIZE0")]
pub type FcfgB2Ssize0 = crate::Reg<fcfg_b2_ssize0::FcfgB2Ssize0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize0;
#[doc = "FCFG_B2_SSIZE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b2_ssize1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b2_ssize1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b2_ssize1`]
module"]
#[doc(alias = "FCFG_B2_SSIZE1")]
pub type FcfgB2Ssize1 = crate::Reg<fcfg_b2_ssize1::FcfgB2Ssize1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize1;
#[doc = "FCFG_B2_SSIZE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b2_ssize2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b2_ssize2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b2_ssize2`]
module"]
#[doc(alias = "FCFG_B2_SSIZE2")]
pub type FcfgB2Ssize2 = crate::Reg<fcfg_b2_ssize2::FcfgB2Ssize2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize2;
#[doc = "FCFG_B2_SSIZE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b2_ssize3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b2_ssize3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b2_ssize3`]
module"]
#[doc(alias = "FCFG_B2_SSIZE3")]
pub type FcfgB2Ssize3 = crate::Reg<fcfg_b2_ssize3::FcfgB2Ssize3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize3;
#[doc = "FCFG_B3_SSIZE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b3_ssize0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b3_ssize0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b3_ssize0`]
module"]
#[doc(alias = "FCFG_B3_SSIZE0")]
pub type FcfgB3Ssize0 = crate::Reg<fcfg_b3_ssize0::FcfgB3Ssize0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize0;
#[doc = "FCFG_B3_SSIZE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b3_ssize1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b3_ssize1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b3_ssize1`]
module"]
#[doc(alias = "FCFG_B3_SSIZE1")]
pub type FcfgB3Ssize1 = crate::Reg<fcfg_b3_ssize1::FcfgB3Ssize1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize1;
#[doc = "FCFG_B3_SSIZE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b3_ssize2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b3_ssize2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b3_ssize2`]
module"]
#[doc(alias = "FCFG_B3_SSIZE2")]
pub type FcfgB3Ssize2 = crate::Reg<fcfg_b3_ssize2::FcfgB3Ssize2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize2;
#[doc = "FCFG_B3_SSIZE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b3_ssize3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b3_ssize3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b3_ssize3`]
module"]
#[doc(alias = "FCFG_B3_SSIZE3")]
pub type FcfgB3Ssize3 = crate::Reg<fcfg_b3_ssize3::FcfgB3Ssize3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize3;
#[doc = "FCFG_B4_SSIZE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b4_ssize0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b4_ssize0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b4_ssize0`]
module"]
#[doc(alias = "FCFG_B4_SSIZE0")]
pub type FcfgB4Ssize0 = crate::Reg<fcfg_b4_ssize0::FcfgB4Ssize0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize0;
#[doc = "FCFG_B4_SSIZE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b4_ssize1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b4_ssize1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b4_ssize1`]
module"]
#[doc(alias = "FCFG_B4_SSIZE1")]
pub type FcfgB4Ssize1 = crate::Reg<fcfg_b4_ssize1::FcfgB4Ssize1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize1;
#[doc = "FCFG_B4_SSIZE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b4_ssize2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b4_ssize2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b4_ssize2`]
module"]
#[doc(alias = "FCFG_B4_SSIZE2")]
pub type FcfgB4Ssize2 = crate::Reg<fcfg_b4_ssize2::FcfgB4Ssize2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize2;
#[doc = "FCFG_B4_SSIZE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b4_ssize3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b4_ssize3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b4_ssize3`]
module"]
#[doc(alias = "FCFG_B4_SSIZE3")]
pub type FcfgB4Ssize3 = crate::Reg<fcfg_b4_ssize3::FcfgB4Ssize3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize3;
#[doc = "FCFG_B5_SSIZE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b5_ssize0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b5_ssize0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b5_ssize0`]
module"]
#[doc(alias = "FCFG_B5_SSIZE0")]
pub type FcfgB5Ssize0 = crate::Reg<fcfg_b5_ssize0::FcfgB5Ssize0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize0;
#[doc = "FCFG_B5_SSIZE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b5_ssize1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b5_ssize1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b5_ssize1`]
module"]
#[doc(alias = "FCFG_B5_SSIZE1")]
pub type FcfgB5Ssize1 = crate::Reg<fcfg_b5_ssize1::FcfgB5Ssize1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize1;
#[doc = "FCFG_B5_SSIZE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b5_ssize2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b5_ssize2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b5_ssize2`]
module"]
#[doc(alias = "FCFG_B5_SSIZE2")]
pub type FcfgB5Ssize2 = crate::Reg<fcfg_b5_ssize2::FcfgB5Ssize2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize2;
#[doc = "FCFG_B5_SSIZE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b5_ssize3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b5_ssize3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b5_ssize3`]
module"]
#[doc(alias = "FCFG_B5_SSIZE3")]
pub type FcfgB5Ssize3 = crate::Reg<fcfg_b5_ssize3::FcfgB5Ssize3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize3;
#[doc = "FCFG_B6_SSIZE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b6_ssize0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b6_ssize0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b6_ssize0`]
module"]
#[doc(alias = "FCFG_B6_SSIZE0")]
pub type FcfgB6Ssize0 = crate::Reg<fcfg_b6_ssize0::FcfgB6Ssize0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize0;
#[doc = "FCFG_B6_SSIZE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b6_ssize1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b6_ssize1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b6_ssize1`]
module"]
#[doc(alias = "FCFG_B6_SSIZE1")]
pub type FcfgB6Ssize1 = crate::Reg<fcfg_b6_ssize1::FcfgB6Ssize1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize1;
#[doc = "FCFG_B6_SSIZE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b6_ssize2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b6_ssize2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b6_ssize2`]
module"]
#[doc(alias = "FCFG_B6_SSIZE2")]
pub type FcfgB6Ssize2 = crate::Reg<fcfg_b6_ssize2::FcfgB6Ssize2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize2;
#[doc = "FCFG_B6_SSIZE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b6_ssize3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b6_ssize3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b6_ssize3`]
module"]
#[doc(alias = "FCFG_B6_SSIZE3")]
pub type FcfgB6Ssize3 = crate::Reg<fcfg_b6_ssize3::FcfgB6Ssize3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize3;
#[doc = "FCFG_B7_SSIZE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b7_ssize0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b7_ssize0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b7_ssize0`]
module"]
#[doc(alias = "FCFG_B7_SSIZE0")]
pub type FcfgB7Ssize0 = crate::Reg<fcfg_b7_ssize0::FcfgB7Ssize0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize0;
#[doc = "FCFG_B7_SSIZE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b7_ssize1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b7_ssize1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b7_ssize1`]
module"]
#[doc(alias = "FCFG_B7_SSIZE1")]
pub type FcfgB7Ssize1 = crate::Reg<fcfg_b7_ssize1::FcfgB7Ssize1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize1;
#[doc = "FCFG_B7_SSIZE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b7_ssize2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b7_ssize2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b7_ssize2`]
module"]
#[doc(alias = "FCFG_B7_SSIZE2")]
pub type FcfgB7Ssize2 = crate::Reg<fcfg_b7_ssize2::FcfgB7Ssize2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize2;
#[doc = "FCFG_B7_SSIZE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b7_ssize3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b7_ssize3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfg_b7_ssize3`]
module"]
#[doc(alias = "FCFG_B7_SSIZE3")]
pub type FcfgB7Ssize3 = crate::Reg<fcfg_b7_ssize3::FcfgB7Ssize3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize3;
