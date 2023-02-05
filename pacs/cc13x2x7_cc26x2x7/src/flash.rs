#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    #[doc = "0x1c - FMC and Efuse Status"]
    pub stat: STAT,
    _reserved1: [u8; 0x04],
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub cfg: CFG,
    _reserved2: [u8; 0x04],
    #[doc = "0x2c - Internal. Only to be used through TI provided API."]
    pub flash_size: FLASH_SIZE,
    _reserved3: [u8; 0x0c],
    #[doc = "0x3c - Internal. Only to be used through TI provided API."]
    pub fwlock: FWLOCK,
    #[doc = "0x40 - Internal. Only to be used through TI provided API."]
    pub fwflag: FWFLAG,
    _reserved5: [u8; 0x0fbc],
    #[doc = "0x1000 - Internal. Only to be used through TI provided API."]
    pub efuse: EFUSE,
    #[doc = "0x1004 - Internal. Only to be used through TI provided API."]
    pub efuseaddr: EFUSEADDR,
    #[doc = "0x1008 - Internal. Only to be used through TI provided API."]
    pub dataupper: DATAUPPER,
    #[doc = "0x100c - Internal. Only to be used through TI provided API."]
    pub datalower: DATALOWER,
    #[doc = "0x1010 - Internal. Only to be used through TI provided API."]
    pub efusecfg: EFUSECFG,
    #[doc = "0x1014 - Internal. Only to be used through TI provided API."]
    pub efusestat: EFUSESTAT,
    #[doc = "0x1018 - Internal. Only to be used through TI provided API."]
    pub acc: ACC,
    #[doc = "0x101c - Internal. Only to be used through TI provided API."]
    pub boundary: BOUNDARY,
    #[doc = "0x1020 - Internal. Only to be used through TI provided API."]
    pub efuseflag: EFUSEFLAG,
    #[doc = "0x1024 - Internal. Only to be used through TI provided API."]
    pub efusekey: EFUSEKEY,
    #[doc = "0x1028 - Internal. Only to be used through TI provided API."]
    pub efuserelease: EFUSERELEASE,
    #[doc = "0x102c - Internal. Only to be used through TI provided API."]
    pub efusepins: EFUSEPINS,
    #[doc = "0x1030 - Internal. Only to be used through TI provided API."]
    pub efusecra: EFUSECRA,
    #[doc = "0x1034 - Internal. Only to be used through TI provided API."]
    pub efuseread: EFUSEREAD,
    #[doc = "0x1038 - Internal. Only to be used through TI provided API."]
    pub efuseprogram: EFUSEPROGRAM,
    #[doc = "0x103c - Internal. Only to be used through TI provided API."]
    pub efuseerror: EFUSEERROR,
    #[doc = "0x1040 - Internal. Only to be used through TI provided API."]
    pub singlebit: SINGLEBIT,
    #[doc = "0x1044 - Internal. Only to be used through TI provided API."]
    pub twobit: TWOBIT,
    #[doc = "0x1048 - Internal. Only to be used through TI provided API."]
    pub selftestcyc: SELFTESTCYC,
    #[doc = "0x104c - Internal. Only to be used through TI provided API."]
    pub selftestsign: SELFTESTSIGN,
    _reserved25: [u8; 0x0fb0],
    #[doc = "0x2000 - Internal. Only to be used through TI provided API."]
    pub frdctl: FRDCTL,
    #[doc = "0x2004 - Internal. Only to be used through TI provided API."]
    pub fsprd: FSPRD,
    #[doc = "0x2008 - Internal. Only to be used through TI provided API."]
    pub fedacctl1: FEDACCTL1,
    #[doc = "0x200c - Internal. Only to be used through TI provided API."]
    pub fedacctl2: FEDACCTL2,
    #[doc = "0x2010 - Internal. Only to be used through TI provided API."]
    pub fcor_err_cnt: FCOR_ERR_CNT,
    #[doc = "0x2014 - Internal. Only to be used through TI provided API."]
    pub fcor_err_add: FCOR_ERR_ADD,
    #[doc = "0x2018 - Internal. Only to be used through TI provided API."]
    pub fcor_err_pos: FCOR_ERR_POS,
    #[doc = "0x201c - Internal. Only to be used through TI provided API."]
    pub fedacstat: FEDACSTAT,
    #[doc = "0x2020 - Internal. Only to be used through TI provided API."]
    pub func_err_add: FUNC_ERR_ADD,
    #[doc = "0x2024 - Internal. Only to be used through TI provided API."]
    pub fedacsdis: FEDACSDIS,
    #[doc = "0x2028 - Internal. Only to be used through TI provided API."]
    pub fprim_add_tag: FPRIM_ADD_TAG,
    #[doc = "0x202c - Internal. Only to be used through TI provided API."]
    pub fredu_add_tag: FREDU_ADD_TAG,
    #[doc = "0x2030 - Internal. Only to be used through TI provided API."]
    pub fbprot: FBPROT,
    #[doc = "0x2034 - Internal. Only to be used through TI provided API."]
    pub fbse: FBSE,
    #[doc = "0x2038 - Internal. Only to be used through TI provided API."]
    pub fbbusy: FBBUSY,
    #[doc = "0x203c - Internal. Only to be used through TI provided API."]
    pub fbac: FBAC,
    #[doc = "0x2040 - Internal. Only to be used through TI provided API."]
    pub fbfallback: FBFALLBACK,
    #[doc = "0x2044 - Internal. Only to be used through TI provided API."]
    pub fbprdy: FBPRDY,
    #[doc = "0x2048 - Internal. Only to be used through TI provided API."]
    pub fpac1: FPAC1,
    #[doc = "0x204c - Internal. Only to be used through TI provided API."]
    pub fpac2: FPAC2,
    #[doc = "0x2050 - Internal. Only to be used through TI provided API."]
    pub fmac: FMAC,
    #[doc = "0x2054 - Internal. Only to be used through TI provided API."]
    pub fmstat: FMSTAT,
    #[doc = "0x2058 - Internal. Only to be used through TI provided API."]
    pub femu_dmsw: FEMU_DMSW,
    #[doc = "0x205c - Internal. Only to be used through TI provided API."]
    pub femu_dlsw: FEMU_DLSW,
    #[doc = "0x2060 - Internal. Only to be used through TI provided API."]
    pub femu_ecc: FEMU_ECC,
    #[doc = "0x2064 - Internal. Only to be used through TI provided API."]
    pub flock: FLOCK,
    #[doc = "0x2068 - Internal. Only to be used through TI provided API."]
    pub femu_addr: FEMU_ADDR,
    #[doc = "0x206c - Internal. Only to be used through TI provided API."]
    pub fdiagctl: FDIAGCTL,
    #[doc = "0x2070 - Internal. Only to be used through TI provided API."]
    pub fraw_datah: FRAW_DATAH,
    #[doc = "0x2074 - Internal. Only to be used through TI provided API."]
    pub fraw_datal: FRAW_DATAL,
    #[doc = "0x2078 - Internal. Only to be used through TI provided API."]
    pub fraw_ecc: FRAW_ECC,
    #[doc = "0x207c - Internal. Only to be used through TI provided API."]
    pub fpar_ovr: FPAR_OVR,
    #[doc = "0x2080 - Internal. Only to be used through TI provided API."]
    pub fvreadct: FVREADCT,
    #[doc = "0x2084 - Internal. Only to be used through TI provided API."]
    pub fvhvct1: FVHVCT1,
    #[doc = "0x2088 - Internal. Only to be used through TI provided API."]
    pub fvhvct2: FVHVCT2,
    #[doc = "0x208c - Internal. Only to be used through TI provided API."]
    pub fvhvct3: FVHVCT3,
    #[doc = "0x2090 - Internal. Only to be used through TI provided API."]
    pub fvnvct: FVNVCT,
    #[doc = "0x2094 - Internal. Only to be used through TI provided API."]
    pub fvslp: FVSLP,
    #[doc = "0x2098 - Internal. Only to be used through TI provided API."]
    pub fvwlct: FVWLCT,
    #[doc = "0x209c - Internal. Only to be used through TI provided API."]
    pub fefusectl: FEFUSECTL,
    #[doc = "0x20a0 - Internal. Only to be used through TI provided API."]
    pub fefusestat: FEFUSESTAT,
    #[doc = "0x20a4 - Internal. Only to be used through TI provided API."]
    pub fefusedata: FEFUSEDATA,
    #[doc = "0x20a8 - Internal. Only to be used through TI provided API."]
    pub fseqpmp: FSEQPMP,
    #[doc = "0x20ac - Internal. Only to be used through TI provided API."]
    pub fclktrim: FCLKTRIM,
    #[doc = "0x20b0 - Internal. Only to be used through TI provided API."]
    pub rom_test: ROM_TEST,
    _reserved70: [u8; 0x0c],
    #[doc = "0x20c0 - Internal. Only to be used through TI provided API."]
    pub fedacsdis2: FEDACSDIS2,
    _reserved71: [u8; 0x3c],
    #[doc = "0x2100 - Internal. Only to be used through TI provided API."]
    pub fbstrobes: FBSTROBES,
    #[doc = "0x2104 - Internal. Only to be used through TI provided API."]
    pub fpstrobes: FPSTROBES,
    #[doc = "0x2108 - Internal. Only to be used through TI provided API."]
    pub fbmode: FBMODE,
    #[doc = "0x210c - Internal. Only to be used through TI provided API."]
    pub ftcr: FTCR,
    #[doc = "0x2110 - Internal. Only to be used through TI provided API."]
    pub faddr: FADDR,
    #[doc = "0x2114 - Internal. Only to be used through TI provided API."]
    pub fpmtctl: FPMTCTL,
    #[doc = "0x2118 - Internal. Only to be used through TI provided API."]
    pub pbistctl: PBISTCTL,
    #[doc = "0x211c - Internal. Only to be used through TI provided API."]
    pub ftctl: FTCTL,
    #[doc = "0x2120 - Internal. Only to be used through TI provided API."]
    pub fwpwrite0: FWPWRITE0,
    #[doc = "0x2124 - Internal. Only to be used through TI provided API."]
    pub fwpwrite1: FWPWRITE1,
    #[doc = "0x2128 - Internal. Only to be used through TI provided API."]
    pub fwpwrite2: FWPWRITE2,
    #[doc = "0x212c - Internal. Only to be used through TI provided API."]
    pub fwpwrite3: FWPWRITE3,
    #[doc = "0x2130 - Internal. Only to be used through TI provided API."]
    pub fwpwrite4: FWPWRITE4,
    #[doc = "0x2134 - Internal. Only to be used through TI provided API."]
    pub fwpwrite5: FWPWRITE5,
    #[doc = "0x2138 - Internal. Only to be used through TI provided API."]
    pub fwpwrite6: FWPWRITE6,
    #[doc = "0x213c - Internal. Only to be used through TI provided API."]
    pub fwpwrite7: FWPWRITE7,
    #[doc = "0x2140 - Internal. Only to be used through TI provided API."]
    pub fwpwrite_ecc: FWPWRITE_ECC,
    #[doc = "0x2144 - Internal. Only to be used through TI provided API."]
    pub fswstat: FSWSTAT,
    _reserved89: [u8; 0xb8],
    #[doc = "0x2200 - Internal. Only to be used through TI provided API."]
    pub fsm_glbctl: FSM_GLBCTL,
    #[doc = "0x2204 - Internal. Only to be used through TI provided API."]
    pub fsm_state: FSM_STATE,
    #[doc = "0x2208 - Internal. Only to be used through TI provided API."]
    pub fsm_stat: FSM_STAT,
    #[doc = "0x220c - Internal. Only to be used through TI provided API."]
    pub fsm_cmd: FSM_CMD,
    #[doc = "0x2210 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_osu: FSM_PE_OSU,
    #[doc = "0x2214 - Internal. Only to be used through TI provided API."]
    pub fsm_vstat: FSM_VSTAT,
    #[doc = "0x2218 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_vsu: FSM_PE_VSU,
    #[doc = "0x221c - Internal. Only to be used through TI provided API."]
    pub fsm_cmp_vsu: FSM_CMP_VSU,
    #[doc = "0x2220 - Internal. Only to be used through TI provided API."]
    pub fsm_ex_val: FSM_EX_VAL,
    #[doc = "0x2224 - Internal. Only to be used through TI provided API."]
    pub fsm_rd_h: FSM_RD_H,
    #[doc = "0x2228 - Internal. Only to be used through TI provided API."]
    pub fsm_p_oh: FSM_P_OH,
    #[doc = "0x222c - Internal. Only to be used through TI provided API."]
    pub fsm_era_oh: FSM_ERA_OH,
    #[doc = "0x2230 - Internal. Only to be used through TI provided API."]
    pub fsm_sav_ppul: FSM_SAV_PPUL,
    #[doc = "0x2234 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_vh: FSM_PE_VH,
    _reserved103: [u8; 0x08],
    #[doc = "0x2240 - Internal. Only to be used through TI provided API."]
    pub fsm_prg_pw: FSM_PRG_PW,
    #[doc = "0x2244 - Internal. Only to be used through TI provided API."]
    pub fsm_era_pw: FSM_ERA_PW,
    _reserved105: [u8; 0x0c],
    #[doc = "0x2254 - Internal. Only to be used through TI provided API."]
    pub fsm_sav_era_pul: FSM_SAV_ERA_PUL,
    #[doc = "0x2258 - Internal. Only to be used through TI provided API."]
    pub fsm_timer: FSM_TIMER,
    #[doc = "0x225c - Internal. Only to be used through TI provided API."]
    pub fsm_mode: FSM_MODE,
    #[doc = "0x2260 - Internal. Only to be used through TI provided API."]
    pub fsm_pgm: FSM_PGM,
    #[doc = "0x2264 - Internal. Only to be used through TI provided API."]
    pub fsm_era: FSM_ERA,
    #[doc = "0x2268 - Internal. Only to be used through TI provided API."]
    pub fsm_prg_pul: FSM_PRG_PUL,
    #[doc = "0x226c - Internal. Only to be used through TI provided API."]
    pub fsm_era_pul: FSM_ERA_PUL,
    #[doc = "0x2270 - Internal. Only to be used through TI provided API."]
    pub fsm_step_size: FSM_STEP_SIZE,
    #[doc = "0x2274 - Internal. Only to be used through TI provided API."]
    pub fsm_pul_cntr: FSM_PUL_CNTR,
    #[doc = "0x2278 - Internal. Only to be used through TI provided API."]
    pub fsm_ec_step_height: FSM_EC_STEP_HEIGHT,
    #[doc = "0x227c - Internal. Only to be used through TI provided API."]
    pub fsm_st_machine: FSM_ST_MACHINE,
    #[doc = "0x2280 - Internal. Only to be used through TI provided API."]
    pub fsm_fles: FSM_FLES,
    _reserved117: [u8; 0x04],
    #[doc = "0x2288 - Internal. Only to be used through TI provided API."]
    pub fsm_wr_ena: FSM_WR_ENA,
    #[doc = "0x228c - Internal. Only to be used through TI provided API."]
    pub fsm_acc_pp: FSM_ACC_PP,
    #[doc = "0x2290 - Internal. Only to be used through TI provided API."]
    pub fsm_acc_ep: FSM_ACC_EP,
    _reserved120: [u8; 0x0c],
    #[doc = "0x22a0 - Internal. Only to be used through TI provided API."]
    pub fsm_addr: FSM_ADDR,
    #[doc = "0x22a4 - Internal. Only to be used through TI provided API."]
    pub fsm_sector: FSM_SECTOR,
    #[doc = "0x22a8 - Internal. Only to be used through TI provided API."]
    pub fmc_rev_id: FMC_REV_ID,
    #[doc = "0x22ac - Internal. Only to be used through TI provided API."]
    pub fsm_err_addr: FSM_ERR_ADDR,
    #[doc = "0x22b0 - Internal. Only to be used through TI provided API."]
    pub fsm_pgm_maxpul: FSM_PGM_MAXPUL,
    #[doc = "0x22b4 - Internal. Only to be used through TI provided API."]
    pub fsm_execute: FSM_EXECUTE,
    #[doc = "0x22b8 - Internal. Only to be used through TI provided API."]
    pub eeprom_cfg: EEPROM_CFG,
    _reserved127: [u8; 0x04],
    #[doc = "0x22c0 - Internal. Only to be used through TI provided API."]
    pub fsm_sector1: FSM_SECTOR1,
    #[doc = "0x22c4 - Internal. Only to be used through TI provided API."]
    pub fsm_sector2: FSM_SECTOR2,
    _reserved129: [u8; 0x18],
    #[doc = "0x22e0 - Internal. Only to be used through TI provided API."]
    pub fsm_bsle0: FSM_BSLE0,
    #[doc = "0x22e4 - Internal. Only to be used through TI provided API."]
    pub fsm_bsle1: FSM_BSLE1,
    _reserved131: [u8; 0x08],
    #[doc = "0x22f0 - Internal. Only to be used through TI provided API."]
    pub fsm_bslp0: FSM_BSLP0,
    #[doc = "0x22f4 - Internal. Only to be used through TI provided API."]
    pub fsm_bslp1: FSM_BSLP1,
    #[doc = "0x22f8 - FMC FSM Enable 128-bit Wide Programming"]
    pub fsm_pgm128: FSM_PGM128,
    #[doc = "0x22fc - FMC FSM Enable Parallell Reads for Multibanks"]
    pub fsm_en_prl_bnk_rd: FSM_EN_PRL_BNK_RD,
    _reserved135: [u8; 0x0100],
    #[doc = "0x2400 - Internal. Only to be used through TI provided API."]
    pub fcfg_bank: FCFG_BANK,
    #[doc = "0x2404 - Internal. Only to be used through TI provided API."]
    pub fcfg_wrapper: FCFG_WRAPPER,
    #[doc = "0x2408 - Internal. Only to be used through TI provided API."]
    pub fcfg_bnk_type: FCFG_BNK_TYPE,
    _reserved138: [u8; 0x04],
    #[doc = "0x2410 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_start: FCFG_B0_START,
    #[doc = "0x2414 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_start: FCFG_B1_START,
    #[doc = "0x2418 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_start: FCFG_B2_START,
    #[doc = "0x241c - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_start: FCFG_B3_START,
    #[doc = "0x2420 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_start: FCFG_B4_START,
    #[doc = "0x2424 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_start: FCFG_B5_START,
    #[doc = "0x2428 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_start: FCFG_B6_START,
    #[doc = "0x242c - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_start: FCFG_B7_START,
    #[doc = "0x2430 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize0: FCFG_B0_SSIZE0,
    #[doc = "0x2434 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize1: FCFG_B0_SSIZE1,
    #[doc = "0x2438 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize2: FCFG_B0_SSIZE2,
    #[doc = "0x243c - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize3: FCFG_B0_SSIZE3,
    #[doc = "0x2440 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_ssize0: FCFG_B1_SSIZE0,
    #[doc = "0x2444 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_ssize1: FCFG_B1_SSIZE1,
    #[doc = "0x2448 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_ssize2: FCFG_B1_SSIZE2,
    #[doc = "0x244c - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_ssize3: FCFG_B1_SSIZE3,
    #[doc = "0x2450 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_ssize0: FCFG_B2_SSIZE0,
    #[doc = "0x2454 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_ssize1: FCFG_B2_SSIZE1,
    #[doc = "0x2458 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_ssize2: FCFG_B2_SSIZE2,
    #[doc = "0x245c - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_ssize3: FCFG_B2_SSIZE3,
    #[doc = "0x2460 - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_ssize0: FCFG_B3_SSIZE0,
    #[doc = "0x2464 - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_ssize1: FCFG_B3_SSIZE1,
    #[doc = "0x2468 - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_ssize2: FCFG_B3_SSIZE2,
    #[doc = "0x246c - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_ssize3: FCFG_B3_SSIZE3,
    #[doc = "0x2470 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_ssize0: FCFG_B4_SSIZE0,
    #[doc = "0x2474 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_ssize1: FCFG_B4_SSIZE1,
    #[doc = "0x2478 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_ssize2: FCFG_B4_SSIZE2,
    #[doc = "0x247c - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_ssize3: FCFG_B4_SSIZE3,
    #[doc = "0x2480 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_ssize0: FCFG_B5_SSIZE0,
    #[doc = "0x2484 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_ssize1: FCFG_B5_SSIZE1,
    #[doc = "0x2488 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_ssize2: FCFG_B5_SSIZE2,
    #[doc = "0x248c - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_ssize3: FCFG_B5_SSIZE3,
    #[doc = "0x2490 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_ssize0: FCFG_B6_SSIZE0,
    #[doc = "0x2494 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_ssize1: FCFG_B6_SSIZE1,
    #[doc = "0x2498 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_ssize2: FCFG_B6_SSIZE2,
    #[doc = "0x249c - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_ssize3: FCFG_B6_SSIZE3,
    #[doc = "0x24a0 - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_ssize0: FCFG_B7_SSIZE0,
    #[doc = "0x24a4 - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_ssize1: FCFG_B7_SSIZE1,
    #[doc = "0x24a8 - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_ssize2: FCFG_B7_SSIZE2,
    #[doc = "0x24ac - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_ssize3: FCFG_B7_SSIZE3,
}
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "FMC and Efuse Status"]
pub mod stat;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cfg;
#[doc = "FLASH_SIZE (rw) register accessor: an alias for `Reg<FLASH_SIZE_SPEC>`"]
pub type FLASH_SIZE = crate::Reg<flash_size::FLASH_SIZE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_size;
#[doc = "FWLOCK (rw) register accessor: an alias for `Reg<FWLOCK_SPEC>`"]
pub type FWLOCK = crate::Reg<fwlock::FWLOCK_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwlock;
#[doc = "FWFLAG (rw) register accessor: an alias for `Reg<FWFLAG_SPEC>`"]
pub type FWFLAG = crate::Reg<fwflag::FWFLAG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwflag;
#[doc = "EFUSE (rw) register accessor: an alias for `Reg<EFUSE_SPEC>`"]
pub type EFUSE = crate::Reg<efuse::EFUSE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuse;
#[doc = "EFUSEADDR (rw) register accessor: an alias for `Reg<EFUSEADDR_SPEC>`"]
pub type EFUSEADDR = crate::Reg<efuseaddr::EFUSEADDR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseaddr;
#[doc = "DATAUPPER (rw) register accessor: an alias for `Reg<DATAUPPER_SPEC>`"]
pub type DATAUPPER = crate::Reg<dataupper::DATAUPPER_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dataupper;
#[doc = "DATALOWER (rw) register accessor: an alias for `Reg<DATALOWER_SPEC>`"]
pub type DATALOWER = crate::Reg<datalower::DATALOWER_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod datalower;
#[doc = "EFUSECFG (rw) register accessor: an alias for `Reg<EFUSECFG_SPEC>`"]
pub type EFUSECFG = crate::Reg<efusecfg::EFUSECFG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecfg;
#[doc = "EFUSESTAT (rw) register accessor: an alias for `Reg<EFUSESTAT_SPEC>`"]
pub type EFUSESTAT = crate::Reg<efusestat::EFUSESTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusestat;
#[doc = "ACC (rw) register accessor: an alias for `Reg<ACC_SPEC>`"]
pub type ACC = crate::Reg<acc::ACC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod acc;
#[doc = "BOUNDARY (rw) register accessor: an alias for `Reg<BOUNDARY_SPEC>`"]
pub type BOUNDARY = crate::Reg<boundary::BOUNDARY_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod boundary;
#[doc = "EFUSEFLAG (rw) register accessor: an alias for `Reg<EFUSEFLAG_SPEC>`"]
pub type EFUSEFLAG = crate::Reg<efuseflag::EFUSEFLAG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseflag;
#[doc = "EFUSEKEY (rw) register accessor: an alias for `Reg<EFUSEKEY_SPEC>`"]
pub type EFUSEKEY = crate::Reg<efusekey::EFUSEKEY_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusekey;
#[doc = "EFUSERELEASE (rw) register accessor: an alias for `Reg<EFUSERELEASE_SPEC>`"]
pub type EFUSERELEASE = crate::Reg<efuserelease::EFUSERELEASE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuserelease;
#[doc = "EFUSEPINS (rw) register accessor: an alias for `Reg<EFUSEPINS_SPEC>`"]
pub type EFUSEPINS = crate::Reg<efusepins::EFUSEPINS_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusepins;
#[doc = "EFUSECRA (rw) register accessor: an alias for `Reg<EFUSECRA_SPEC>`"]
pub type EFUSECRA = crate::Reg<efusecra::EFUSECRA_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecra;
#[doc = "EFUSEREAD (rw) register accessor: an alias for `Reg<EFUSEREAD_SPEC>`"]
pub type EFUSEREAD = crate::Reg<efuseread::EFUSEREAD_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseread;
#[doc = "EFUSEPROGRAM (rw) register accessor: an alias for `Reg<EFUSEPROGRAM_SPEC>`"]
pub type EFUSEPROGRAM = crate::Reg<efuseprogram::EFUSEPROGRAM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseprogram;
#[doc = "EFUSEERROR (rw) register accessor: an alias for `Reg<EFUSEERROR_SPEC>`"]
pub type EFUSEERROR = crate::Reg<efuseerror::EFUSEERROR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseerror;
#[doc = "SINGLEBIT (rw) register accessor: an alias for `Reg<SINGLEBIT_SPEC>`"]
pub type SINGLEBIT = crate::Reg<singlebit::SINGLEBIT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod singlebit;
#[doc = "TWOBIT (rw) register accessor: an alias for `Reg<TWOBIT_SPEC>`"]
pub type TWOBIT = crate::Reg<twobit::TWOBIT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod twobit;
#[doc = "SELFTESTCYC (rw) register accessor: an alias for `Reg<SELFTESTCYC_SPEC>`"]
pub type SELFTESTCYC = crate::Reg<selftestcyc::SELFTESTCYC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestcyc;
#[doc = "SELFTESTSIGN (rw) register accessor: an alias for `Reg<SELFTESTSIGN_SPEC>`"]
pub type SELFTESTSIGN = crate::Reg<selftestsign::SELFTESTSIGN_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestsign;
#[doc = "FRDCTL (rw) register accessor: an alias for `Reg<FRDCTL_SPEC>`"]
pub type FRDCTL = crate::Reg<frdctl::FRDCTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod frdctl;
#[doc = "FSPRD (rw) register accessor: an alias for `Reg<FSPRD_SPEC>`"]
pub type FSPRD = crate::Reg<fsprd::FSPRD_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsprd;
#[doc = "FEDACCTL1 (rw) register accessor: an alias for `Reg<FEDACCTL1_SPEC>`"]
pub type FEDACCTL1 = crate::Reg<fedacctl1::FEDACCTL1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacctl1;
#[doc = "FEDACCTL2 (rw) register accessor: an alias for `Reg<FEDACCTL2_SPEC>`"]
pub type FEDACCTL2 = crate::Reg<fedacctl2::FEDACCTL2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacctl2;
#[doc = "FCOR_ERR_CNT (rw) register accessor: an alias for `Reg<FCOR_ERR_CNT_SPEC>`"]
pub type FCOR_ERR_CNT = crate::Reg<fcor_err_cnt::FCOR_ERR_CNT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_cnt;
#[doc = "FCOR_ERR_ADD (rw) register accessor: an alias for `Reg<FCOR_ERR_ADD_SPEC>`"]
pub type FCOR_ERR_ADD = crate::Reg<fcor_err_add::FCOR_ERR_ADD_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_add;
#[doc = "FCOR_ERR_POS (rw) register accessor: an alias for `Reg<FCOR_ERR_POS_SPEC>`"]
pub type FCOR_ERR_POS = crate::Reg<fcor_err_pos::FCOR_ERR_POS_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_pos;
#[doc = "FEDACSTAT (rw) register accessor: an alias for `Reg<FEDACSTAT_SPEC>`"]
pub type FEDACSTAT = crate::Reg<fedacstat::FEDACSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacstat;
#[doc = "FUNC_ERR_ADD (rw) register accessor: an alias for `Reg<FUNC_ERR_ADD_SPEC>`"]
pub type FUNC_ERR_ADD = crate::Reg<func_err_add::FUNC_ERR_ADD_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod func_err_add;
#[doc = "FEDACSDIS (rw) register accessor: an alias for `Reg<FEDACSDIS_SPEC>`"]
pub type FEDACSDIS = crate::Reg<fedacsdis::FEDACSDIS_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacsdis;
#[doc = "FPRIM_ADD_TAG (rw) register accessor: an alias for `Reg<FPRIM_ADD_TAG_SPEC>`"]
pub type FPRIM_ADD_TAG = crate::Reg<fprim_add_tag::FPRIM_ADD_TAG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fprim_add_tag;
#[doc = "FREDU_ADD_TAG (rw) register accessor: an alias for `Reg<FREDU_ADD_TAG_SPEC>`"]
pub type FREDU_ADD_TAG = crate::Reg<fredu_add_tag::FREDU_ADD_TAG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fredu_add_tag;
#[doc = "FBPROT (rw) register accessor: an alias for `Reg<FBPROT_SPEC>`"]
pub type FBPROT = crate::Reg<fbprot::FBPROT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbprot;
#[doc = "FBSE (rw) register accessor: an alias for `Reg<FBSE_SPEC>`"]
pub type FBSE = crate::Reg<fbse::FBSE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbse;
#[doc = "FBBUSY (rw) register accessor: an alias for `Reg<FBBUSY_SPEC>`"]
pub type FBBUSY = crate::Reg<fbbusy::FBBUSY_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbbusy;
#[doc = "FBAC (rw) register accessor: an alias for `Reg<FBAC_SPEC>`"]
pub type FBAC = crate::Reg<fbac::FBAC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbac;
#[doc = "FBFALLBACK (rw) register accessor: an alias for `Reg<FBFALLBACK_SPEC>`"]
pub type FBFALLBACK = crate::Reg<fbfallback::FBFALLBACK_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbfallback;
#[doc = "FBPRDY (rw) register accessor: an alias for `Reg<FBPRDY_SPEC>`"]
pub type FBPRDY = crate::Reg<fbprdy::FBPRDY_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbprdy;
#[doc = "FPAC1 (rw) register accessor: an alias for `Reg<FPAC1_SPEC>`"]
pub type FPAC1 = crate::Reg<fpac1::FPAC1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpac1;
#[doc = "FPAC2 (rw) register accessor: an alias for `Reg<FPAC2_SPEC>`"]
pub type FPAC2 = crate::Reg<fpac2::FPAC2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpac2;
#[doc = "FMAC (rw) register accessor: an alias for `Reg<FMAC_SPEC>`"]
pub type FMAC = crate::Reg<fmac::FMAC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmac;
#[doc = "FMSTAT (rw) register accessor: an alias for `Reg<FMSTAT_SPEC>`"]
pub type FMSTAT = crate::Reg<fmstat::FMSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmstat;
#[doc = "FEMU_DMSW (rw) register accessor: an alias for `Reg<FEMU_DMSW_SPEC>`"]
pub type FEMU_DMSW = crate::Reg<femu_dmsw::FEMU_DMSW_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_dmsw;
#[doc = "FEMU_DLSW (rw) register accessor: an alias for `Reg<FEMU_DLSW_SPEC>`"]
pub type FEMU_DLSW = crate::Reg<femu_dlsw::FEMU_DLSW_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_dlsw;
#[doc = "FEMU_ECC (rw) register accessor: an alias for `Reg<FEMU_ECC_SPEC>`"]
pub type FEMU_ECC = crate::Reg<femu_ecc::FEMU_ECC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_ecc;
#[doc = "FLOCK (rw) register accessor: an alias for `Reg<FLOCK_SPEC>`"]
pub type FLOCK = crate::Reg<flock::FLOCK_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flock;
#[doc = "FEMU_ADDR (rw) register accessor: an alias for `Reg<FEMU_ADDR_SPEC>`"]
pub type FEMU_ADDR = crate::Reg<femu_addr::FEMU_ADDR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_addr;
#[doc = "FDIAGCTL (rw) register accessor: an alias for `Reg<FDIAGCTL_SPEC>`"]
pub type FDIAGCTL = crate::Reg<fdiagctl::FDIAGCTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fdiagctl;
#[doc = "FRAW_DATAH (rw) register accessor: an alias for `Reg<FRAW_DATAH_SPEC>`"]
pub type FRAW_DATAH = crate::Reg<fraw_datah::FRAW_DATAH_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_datah;
#[doc = "FRAW_DATAL (rw) register accessor: an alias for `Reg<FRAW_DATAL_SPEC>`"]
pub type FRAW_DATAL = crate::Reg<fraw_datal::FRAW_DATAL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_datal;
#[doc = "FRAW_ECC (rw) register accessor: an alias for `Reg<FRAW_ECC_SPEC>`"]
pub type FRAW_ECC = crate::Reg<fraw_ecc::FRAW_ECC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_ecc;
#[doc = "FPAR_OVR (rw) register accessor: an alias for `Reg<FPAR_OVR_SPEC>`"]
pub type FPAR_OVR = crate::Reg<fpar_ovr::FPAR_OVR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpar_ovr;
#[doc = "FVREADCT (rw) register accessor: an alias for `Reg<FVREADCT_SPEC>`"]
pub type FVREADCT = crate::Reg<fvreadct::FVREADCT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvreadct;
#[doc = "FVHVCT1 (rw) register accessor: an alias for `Reg<FVHVCT1_SPEC>`"]
pub type FVHVCT1 = crate::Reg<fvhvct1::FVHVCT1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct1;
#[doc = "FVHVCT2 (rw) register accessor: an alias for `Reg<FVHVCT2_SPEC>`"]
pub type FVHVCT2 = crate::Reg<fvhvct2::FVHVCT2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct2;
#[doc = "FVHVCT3 (rw) register accessor: an alias for `Reg<FVHVCT3_SPEC>`"]
pub type FVHVCT3 = crate::Reg<fvhvct3::FVHVCT3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct3;
#[doc = "FVNVCT (rw) register accessor: an alias for `Reg<FVNVCT_SPEC>`"]
pub type FVNVCT = crate::Reg<fvnvct::FVNVCT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvnvct;
#[doc = "FVSLP (rw) register accessor: an alias for `Reg<FVSLP_SPEC>`"]
pub type FVSLP = crate::Reg<fvslp::FVSLP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvslp;
#[doc = "FVWLCT (rw) register accessor: an alias for `Reg<FVWLCT_SPEC>`"]
pub type FVWLCT = crate::Reg<fvwlct::FVWLCT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvwlct;
#[doc = "FEFUSECTL (rw) register accessor: an alias for `Reg<FEFUSECTL_SPEC>`"]
pub type FEFUSECTL = crate::Reg<fefusectl::FEFUSECTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusectl;
#[doc = "FEFUSESTAT (rw) register accessor: an alias for `Reg<FEFUSESTAT_SPEC>`"]
pub type FEFUSESTAT = crate::Reg<fefusestat::FEFUSESTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusestat;
#[doc = "FEFUSEDATA (rw) register accessor: an alias for `Reg<FEFUSEDATA_SPEC>`"]
pub type FEFUSEDATA = crate::Reg<fefusedata::FEFUSEDATA_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusedata;
#[doc = "FSEQPMP (rw) register accessor: an alias for `Reg<FSEQPMP_SPEC>`"]
pub type FSEQPMP = crate::Reg<fseqpmp::FSEQPMP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fseqpmp;
#[doc = "FCLKTRIM (rw) register accessor: an alias for `Reg<FCLKTRIM_SPEC>`"]
pub type FCLKTRIM = crate::Reg<fclktrim::FCLKTRIM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fclktrim;
#[doc = "ROM_TEST (rw) register accessor: an alias for `Reg<ROM_TEST_SPEC>`"]
pub type ROM_TEST = crate::Reg<rom_test::ROM_TEST_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rom_test;
#[doc = "FEDACSDIS2 (rw) register accessor: an alias for `Reg<FEDACSDIS2_SPEC>`"]
pub type FEDACSDIS2 = crate::Reg<fedacsdis2::FEDACSDIS2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacsdis2;
#[doc = "FBSTROBES (rw) register accessor: an alias for `Reg<FBSTROBES_SPEC>`"]
pub type FBSTROBES = crate::Reg<fbstrobes::FBSTROBES_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbstrobes;
#[doc = "FPSTROBES (rw) register accessor: an alias for `Reg<FPSTROBES_SPEC>`"]
pub type FPSTROBES = crate::Reg<fpstrobes::FPSTROBES_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpstrobes;
#[doc = "FBMODE (rw) register accessor: an alias for `Reg<FBMODE_SPEC>`"]
pub type FBMODE = crate::Reg<fbmode::FBMODE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbmode;
#[doc = "FTCR (rw) register accessor: an alias for `Reg<FTCR_SPEC>`"]
pub type FTCR = crate::Reg<ftcr::FTCR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ftcr;
#[doc = "FADDR (rw) register accessor: an alias for `Reg<FADDR_SPEC>`"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod faddr;
#[doc = "FPMTCTL (rw) register accessor: an alias for `Reg<FPMTCTL_SPEC>`"]
pub type FPMTCTL = crate::Reg<fpmtctl::FPMTCTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpmtctl;
#[doc = "PBISTCTL (rw) register accessor: an alias for `Reg<PBISTCTL_SPEC>`"]
pub type PBISTCTL = crate::Reg<pbistctl::PBISTCTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pbistctl;
#[doc = "FTCTL (rw) register accessor: an alias for `Reg<FTCTL_SPEC>`"]
pub type FTCTL = crate::Reg<ftctl::FTCTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ftctl;
#[doc = "FWPWRITE0 (rw) register accessor: an alias for `Reg<FWPWRITE0_SPEC>`"]
pub type FWPWRITE0 = crate::Reg<fwpwrite0::FWPWRITE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite0;
#[doc = "FWPWRITE1 (rw) register accessor: an alias for `Reg<FWPWRITE1_SPEC>`"]
pub type FWPWRITE1 = crate::Reg<fwpwrite1::FWPWRITE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite1;
#[doc = "FWPWRITE2 (rw) register accessor: an alias for `Reg<FWPWRITE2_SPEC>`"]
pub type FWPWRITE2 = crate::Reg<fwpwrite2::FWPWRITE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite2;
#[doc = "FWPWRITE3 (rw) register accessor: an alias for `Reg<FWPWRITE3_SPEC>`"]
pub type FWPWRITE3 = crate::Reg<fwpwrite3::FWPWRITE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite3;
#[doc = "FWPWRITE4 (rw) register accessor: an alias for `Reg<FWPWRITE4_SPEC>`"]
pub type FWPWRITE4 = crate::Reg<fwpwrite4::FWPWRITE4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite4;
#[doc = "FWPWRITE5 (rw) register accessor: an alias for `Reg<FWPWRITE5_SPEC>`"]
pub type FWPWRITE5 = crate::Reg<fwpwrite5::FWPWRITE5_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite5;
#[doc = "FWPWRITE6 (rw) register accessor: an alias for `Reg<FWPWRITE6_SPEC>`"]
pub type FWPWRITE6 = crate::Reg<fwpwrite6::FWPWRITE6_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite6;
#[doc = "FWPWRITE7 (rw) register accessor: an alias for `Reg<FWPWRITE7_SPEC>`"]
pub type FWPWRITE7 = crate::Reg<fwpwrite7::FWPWRITE7_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite7;
#[doc = "FWPWRITE_ECC (rw) register accessor: an alias for `Reg<FWPWRITE_ECC_SPEC>`"]
pub type FWPWRITE_ECC = crate::Reg<fwpwrite_ecc::FWPWRITE_ECC_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite_ecc;
#[doc = "FSWSTAT (rw) register accessor: an alias for `Reg<FSWSTAT_SPEC>`"]
pub type FSWSTAT = crate::Reg<fswstat::FSWSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fswstat;
#[doc = "FSM_GLBCTL (rw) register accessor: an alias for `Reg<FSM_GLBCTL_SPEC>`"]
pub type FSM_GLBCTL = crate::Reg<fsm_glbctl::FSM_GLBCTL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_glbctl;
#[doc = "FSM_STATE (rw) register accessor: an alias for `Reg<FSM_STATE_SPEC>`"]
pub type FSM_STATE = crate::Reg<fsm_state::FSM_STATE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_state;
#[doc = "FSM_STAT (rw) register accessor: an alias for `Reg<FSM_STAT_SPEC>`"]
pub type FSM_STAT = crate::Reg<fsm_stat::FSM_STAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_stat;
#[doc = "FSM_CMD (rw) register accessor: an alias for `Reg<FSM_CMD_SPEC>`"]
pub type FSM_CMD = crate::Reg<fsm_cmd::FSM_CMD_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_cmd;
#[doc = "FSM_PE_OSU (rw) register accessor: an alias for `Reg<FSM_PE_OSU_SPEC>`"]
pub type FSM_PE_OSU = crate::Reg<fsm_pe_osu::FSM_PE_OSU_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_osu;
#[doc = "FSM_VSTAT (rw) register accessor: an alias for `Reg<FSM_VSTAT_SPEC>`"]
pub type FSM_VSTAT = crate::Reg<fsm_vstat::FSM_VSTAT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_vstat;
#[doc = "FSM_PE_VSU (rw) register accessor: an alias for `Reg<FSM_PE_VSU_SPEC>`"]
pub type FSM_PE_VSU = crate::Reg<fsm_pe_vsu::FSM_PE_VSU_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_vsu;
#[doc = "FSM_CMP_VSU (rw) register accessor: an alias for `Reg<FSM_CMP_VSU_SPEC>`"]
pub type FSM_CMP_VSU = crate::Reg<fsm_cmp_vsu::FSM_CMP_VSU_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_cmp_vsu;
#[doc = "FSM_EX_VAL (rw) register accessor: an alias for `Reg<FSM_EX_VAL_SPEC>`"]
pub type FSM_EX_VAL = crate::Reg<fsm_ex_val::FSM_EX_VAL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_ex_val;
#[doc = "FSM_RD_H (rw) register accessor: an alias for `Reg<FSM_RD_H_SPEC>`"]
pub type FSM_RD_H = crate::Reg<fsm_rd_h::FSM_RD_H_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_rd_h;
#[doc = "FSM_P_OH (rw) register accessor: an alias for `Reg<FSM_P_OH_SPEC>`"]
pub type FSM_P_OH = crate::Reg<fsm_p_oh::FSM_P_OH_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_p_oh;
#[doc = "FSM_ERA_OH (rw) register accessor: an alias for `Reg<FSM_ERA_OH_SPEC>`"]
pub type FSM_ERA_OH = crate::Reg<fsm_era_oh::FSM_ERA_OH_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_oh;
#[doc = "FSM_SAV_PPUL (rw) register accessor: an alias for `Reg<FSM_SAV_PPUL_SPEC>`"]
pub type FSM_SAV_PPUL = crate::Reg<fsm_sav_ppul::FSM_SAV_PPUL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sav_ppul;
#[doc = "FSM_PE_VH (rw) register accessor: an alias for `Reg<FSM_PE_VH_SPEC>`"]
pub type FSM_PE_VH = crate::Reg<fsm_pe_vh::FSM_PE_VH_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_vh;
#[doc = "FSM_PRG_PW (rw) register accessor: an alias for `Reg<FSM_PRG_PW_SPEC>`"]
pub type FSM_PRG_PW = crate::Reg<fsm_prg_pw::FSM_PRG_PW_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_prg_pw;
#[doc = "FSM_ERA_PW (rw) register accessor: an alias for `Reg<FSM_ERA_PW_SPEC>`"]
pub type FSM_ERA_PW = crate::Reg<fsm_era_pw::FSM_ERA_PW_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_pw;
#[doc = "FSM_SAV_ERA_PUL (rw) register accessor: an alias for `Reg<FSM_SAV_ERA_PUL_SPEC>`"]
pub type FSM_SAV_ERA_PUL = crate::Reg<fsm_sav_era_pul::FSM_SAV_ERA_PUL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sav_era_pul;
#[doc = "FSM_TIMER (rw) register accessor: an alias for `Reg<FSM_TIMER_SPEC>`"]
pub type FSM_TIMER = crate::Reg<fsm_timer::FSM_TIMER_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_timer;
#[doc = "FSM_MODE (rw) register accessor: an alias for `Reg<FSM_MODE_SPEC>`"]
pub type FSM_MODE = crate::Reg<fsm_mode::FSM_MODE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_mode;
#[doc = "FSM_PGM (rw) register accessor: an alias for `Reg<FSM_PGM_SPEC>`"]
pub type FSM_PGM = crate::Reg<fsm_pgm::FSM_PGM_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pgm;
#[doc = "FSM_ERA (rw) register accessor: an alias for `Reg<FSM_ERA_SPEC>`"]
pub type FSM_ERA = crate::Reg<fsm_era::FSM_ERA_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era;
#[doc = "FSM_PRG_PUL (rw) register accessor: an alias for `Reg<FSM_PRG_PUL_SPEC>`"]
pub type FSM_PRG_PUL = crate::Reg<fsm_prg_pul::FSM_PRG_PUL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_prg_pul;
#[doc = "FSM_ERA_PUL (rw) register accessor: an alias for `Reg<FSM_ERA_PUL_SPEC>`"]
pub type FSM_ERA_PUL = crate::Reg<fsm_era_pul::FSM_ERA_PUL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_pul;
#[doc = "FSM_STEP_SIZE (rw) register accessor: an alias for `Reg<FSM_STEP_SIZE_SPEC>`"]
pub type FSM_STEP_SIZE = crate::Reg<fsm_step_size::FSM_STEP_SIZE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_step_size;
#[doc = "FSM_PUL_CNTR (rw) register accessor: an alias for `Reg<FSM_PUL_CNTR_SPEC>`"]
pub type FSM_PUL_CNTR = crate::Reg<fsm_pul_cntr::FSM_PUL_CNTR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pul_cntr;
#[doc = "FSM_EC_STEP_HEIGHT (rw) register accessor: an alias for `Reg<FSM_EC_STEP_HEIGHT_SPEC>`"]
pub type FSM_EC_STEP_HEIGHT = crate::Reg<fsm_ec_step_height::FSM_EC_STEP_HEIGHT_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_ec_step_height;
#[doc = "FSM_ST_MACHINE (rw) register accessor: an alias for `Reg<FSM_ST_MACHINE_SPEC>`"]
pub type FSM_ST_MACHINE = crate::Reg<fsm_st_machine::FSM_ST_MACHINE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_st_machine;
#[doc = "FSM_FLES (rw) register accessor: an alias for `Reg<FSM_FLES_SPEC>`"]
pub type FSM_FLES = crate::Reg<fsm_fles::FSM_FLES_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_fles;
#[doc = "FSM_WR_ENA (rw) register accessor: an alias for `Reg<FSM_WR_ENA_SPEC>`"]
pub type FSM_WR_ENA = crate::Reg<fsm_wr_ena::FSM_WR_ENA_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_wr_ena;
#[doc = "FSM_ACC_PP (rw) register accessor: an alias for `Reg<FSM_ACC_PP_SPEC>`"]
pub type FSM_ACC_PP = crate::Reg<fsm_acc_pp::FSM_ACC_PP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_acc_pp;
#[doc = "FSM_ACC_EP (rw) register accessor: an alias for `Reg<FSM_ACC_EP_SPEC>`"]
pub type FSM_ACC_EP = crate::Reg<fsm_acc_ep::FSM_ACC_EP_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_acc_ep;
#[doc = "FSM_ADDR (rw) register accessor: an alias for `Reg<FSM_ADDR_SPEC>`"]
pub type FSM_ADDR = crate::Reg<fsm_addr::FSM_ADDR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_addr;
#[doc = "FSM_SECTOR (rw) register accessor: an alias for `Reg<FSM_SECTOR_SPEC>`"]
pub type FSM_SECTOR = crate::Reg<fsm_sector::FSM_SECTOR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector;
#[doc = "FMC_REV_ID (rw) register accessor: an alias for `Reg<FMC_REV_ID_SPEC>`"]
pub type FMC_REV_ID = crate::Reg<fmc_rev_id::FMC_REV_ID_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmc_rev_id;
#[doc = "FSM_ERR_ADDR (rw) register accessor: an alias for `Reg<FSM_ERR_ADDR_SPEC>`"]
pub type FSM_ERR_ADDR = crate::Reg<fsm_err_addr::FSM_ERR_ADDR_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_err_addr;
#[doc = "FSM_PGM_MAXPUL (rw) register accessor: an alias for `Reg<FSM_PGM_MAXPUL_SPEC>`"]
pub type FSM_PGM_MAXPUL = crate::Reg<fsm_pgm_maxpul::FSM_PGM_MAXPUL_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pgm_maxpul;
#[doc = "FSM_EXECUTE (rw) register accessor: an alias for `Reg<FSM_EXECUTE_SPEC>`"]
pub type FSM_EXECUTE = crate::Reg<fsm_execute::FSM_EXECUTE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_execute;
#[doc = "EEPROM_CFG (rw) register accessor: an alias for `Reg<EEPROM_CFG_SPEC>`"]
pub type EEPROM_CFG = crate::Reg<eeprom_cfg::EEPROM_CFG_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod eeprom_cfg;
#[doc = "FSM_SECTOR1 (rw) register accessor: an alias for `Reg<FSM_SECTOR1_SPEC>`"]
pub type FSM_SECTOR1 = crate::Reg<fsm_sector1::FSM_SECTOR1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector1;
#[doc = "FSM_SECTOR2 (rw) register accessor: an alias for `Reg<FSM_SECTOR2_SPEC>`"]
pub type FSM_SECTOR2 = crate::Reg<fsm_sector2::FSM_SECTOR2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector2;
#[doc = "FSM_BSLE0 (rw) register accessor: an alias for `Reg<FSM_BSLE0_SPEC>`"]
pub type FSM_BSLE0 = crate::Reg<fsm_bsle0::FSM_BSLE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bsle0;
#[doc = "FSM_BSLE1 (rw) register accessor: an alias for `Reg<FSM_BSLE1_SPEC>`"]
pub type FSM_BSLE1 = crate::Reg<fsm_bsle1::FSM_BSLE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bsle1;
#[doc = "FSM_BSLP0 (rw) register accessor: an alias for `Reg<FSM_BSLP0_SPEC>`"]
pub type FSM_BSLP0 = crate::Reg<fsm_bslp0::FSM_BSLP0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bslp0;
#[doc = "FSM_BSLP1 (rw) register accessor: an alias for `Reg<FSM_BSLP1_SPEC>`"]
pub type FSM_BSLP1 = crate::Reg<fsm_bslp1::FSM_BSLP1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bslp1;
#[doc = "FSM_PGM128 (rw) register accessor: an alias for `Reg<FSM_PGM128_SPEC>`"]
pub type FSM_PGM128 = crate::Reg<fsm_pgm128::FSM_PGM128_SPEC>;
#[doc = "FMC FSM Enable 128-bit Wide Programming"]
pub mod fsm_pgm128;
#[doc = "FSM_EN_PRL_BNK_RD (rw) register accessor: an alias for `Reg<FSM_EN_PRL_BNK_RD_SPEC>`"]
pub type FSM_EN_PRL_BNK_RD = crate::Reg<fsm_en_prl_bnk_rd::FSM_EN_PRL_BNK_RD_SPEC>;
#[doc = "FMC FSM Enable Parallell Reads for Multibanks"]
pub mod fsm_en_prl_bnk_rd;
#[doc = "FCFG_BANK (rw) register accessor: an alias for `Reg<FCFG_BANK_SPEC>`"]
pub type FCFG_BANK = crate::Reg<fcfg_bank::FCFG_BANK_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_bank;
#[doc = "FCFG_WRAPPER (rw) register accessor: an alias for `Reg<FCFG_WRAPPER_SPEC>`"]
pub type FCFG_WRAPPER = crate::Reg<fcfg_wrapper::FCFG_WRAPPER_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_wrapper;
#[doc = "FCFG_BNK_TYPE (rw) register accessor: an alias for `Reg<FCFG_BNK_TYPE_SPEC>`"]
pub type FCFG_BNK_TYPE = crate::Reg<fcfg_bnk_type::FCFG_BNK_TYPE_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_bnk_type;
#[doc = "FCFG_B0_START (rw) register accessor: an alias for `Reg<FCFG_B0_START_SPEC>`"]
pub type FCFG_B0_START = crate::Reg<fcfg_b0_start::FCFG_B0_START_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_start;
#[doc = "FCFG_B1_START (rw) register accessor: an alias for `Reg<FCFG_B1_START_SPEC>`"]
pub type FCFG_B1_START = crate::Reg<fcfg_b1_start::FCFG_B1_START_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_start;
#[doc = "FCFG_B2_START (rw) register accessor: an alias for `Reg<FCFG_B2_START_SPEC>`"]
pub type FCFG_B2_START = crate::Reg<fcfg_b2_start::FCFG_B2_START_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_start;
#[doc = "FCFG_B3_START (rw) register accessor: an alias for `Reg<FCFG_B3_START_SPEC>`"]
pub type FCFG_B3_START = crate::Reg<fcfg_b3_start::FCFG_B3_START_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_start;
#[doc = "FCFG_B4_START (rw) register accessor: an alias for `Reg<FCFG_B4_START_SPEC>`"]
pub type FCFG_B4_START = crate::Reg<fcfg_b4_start::FCFG_B4_START_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_start;
#[doc = "FCFG_B5_START (rw) register accessor: an alias for `Reg<FCFG_B5_START_SPEC>`"]
pub type FCFG_B5_START = crate::Reg<fcfg_b5_start::FCFG_B5_START_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_start;
#[doc = "FCFG_B6_START (rw) register accessor: an alias for `Reg<FCFG_B6_START_SPEC>`"]
pub type FCFG_B6_START = crate::Reg<fcfg_b6_start::FCFG_B6_START_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_start;
#[doc = "FCFG_B7_START (rw) register accessor: an alias for `Reg<FCFG_B7_START_SPEC>`"]
pub type FCFG_B7_START = crate::Reg<fcfg_b7_start::FCFG_B7_START_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_start;
#[doc = "FCFG_B0_SSIZE0 (rw) register accessor: an alias for `Reg<FCFG_B0_SSIZE0_SPEC>`"]
pub type FCFG_B0_SSIZE0 = crate::Reg<fcfg_b0_ssize0::FCFG_B0_SSIZE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize0;
#[doc = "FCFG_B0_SSIZE1 (rw) register accessor: an alias for `Reg<FCFG_B0_SSIZE1_SPEC>`"]
pub type FCFG_B0_SSIZE1 = crate::Reg<fcfg_b0_ssize1::FCFG_B0_SSIZE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize1;
#[doc = "FCFG_B0_SSIZE2 (rw) register accessor: an alias for `Reg<FCFG_B0_SSIZE2_SPEC>`"]
pub type FCFG_B0_SSIZE2 = crate::Reg<fcfg_b0_ssize2::FCFG_B0_SSIZE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize2;
#[doc = "FCFG_B0_SSIZE3 (rw) register accessor: an alias for `Reg<FCFG_B0_SSIZE3_SPEC>`"]
pub type FCFG_B0_SSIZE3 = crate::Reg<fcfg_b0_ssize3::FCFG_B0_SSIZE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize3;
#[doc = "FCFG_B1_SSIZE0 (rw) register accessor: an alias for `Reg<FCFG_B1_SSIZE0_SPEC>`"]
pub type FCFG_B1_SSIZE0 = crate::Reg<fcfg_b1_ssize0::FCFG_B1_SSIZE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize0;
#[doc = "FCFG_B1_SSIZE1 (rw) register accessor: an alias for `Reg<FCFG_B1_SSIZE1_SPEC>`"]
pub type FCFG_B1_SSIZE1 = crate::Reg<fcfg_b1_ssize1::FCFG_B1_SSIZE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize1;
#[doc = "FCFG_B1_SSIZE2 (rw) register accessor: an alias for `Reg<FCFG_B1_SSIZE2_SPEC>`"]
pub type FCFG_B1_SSIZE2 = crate::Reg<fcfg_b1_ssize2::FCFG_B1_SSIZE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize2;
#[doc = "FCFG_B1_SSIZE3 (rw) register accessor: an alias for `Reg<FCFG_B1_SSIZE3_SPEC>`"]
pub type FCFG_B1_SSIZE3 = crate::Reg<fcfg_b1_ssize3::FCFG_B1_SSIZE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize3;
#[doc = "FCFG_B2_SSIZE0 (rw) register accessor: an alias for `Reg<FCFG_B2_SSIZE0_SPEC>`"]
pub type FCFG_B2_SSIZE0 = crate::Reg<fcfg_b2_ssize0::FCFG_B2_SSIZE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize0;
#[doc = "FCFG_B2_SSIZE1 (rw) register accessor: an alias for `Reg<FCFG_B2_SSIZE1_SPEC>`"]
pub type FCFG_B2_SSIZE1 = crate::Reg<fcfg_b2_ssize1::FCFG_B2_SSIZE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize1;
#[doc = "FCFG_B2_SSIZE2 (rw) register accessor: an alias for `Reg<FCFG_B2_SSIZE2_SPEC>`"]
pub type FCFG_B2_SSIZE2 = crate::Reg<fcfg_b2_ssize2::FCFG_B2_SSIZE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize2;
#[doc = "FCFG_B2_SSIZE3 (rw) register accessor: an alias for `Reg<FCFG_B2_SSIZE3_SPEC>`"]
pub type FCFG_B2_SSIZE3 = crate::Reg<fcfg_b2_ssize3::FCFG_B2_SSIZE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize3;
#[doc = "FCFG_B3_SSIZE0 (rw) register accessor: an alias for `Reg<FCFG_B3_SSIZE0_SPEC>`"]
pub type FCFG_B3_SSIZE0 = crate::Reg<fcfg_b3_ssize0::FCFG_B3_SSIZE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize0;
#[doc = "FCFG_B3_SSIZE1 (rw) register accessor: an alias for `Reg<FCFG_B3_SSIZE1_SPEC>`"]
pub type FCFG_B3_SSIZE1 = crate::Reg<fcfg_b3_ssize1::FCFG_B3_SSIZE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize1;
#[doc = "FCFG_B3_SSIZE2 (rw) register accessor: an alias for `Reg<FCFG_B3_SSIZE2_SPEC>`"]
pub type FCFG_B3_SSIZE2 = crate::Reg<fcfg_b3_ssize2::FCFG_B3_SSIZE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize2;
#[doc = "FCFG_B3_SSIZE3 (rw) register accessor: an alias for `Reg<FCFG_B3_SSIZE3_SPEC>`"]
pub type FCFG_B3_SSIZE3 = crate::Reg<fcfg_b3_ssize3::FCFG_B3_SSIZE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize3;
#[doc = "FCFG_B4_SSIZE0 (rw) register accessor: an alias for `Reg<FCFG_B4_SSIZE0_SPEC>`"]
pub type FCFG_B4_SSIZE0 = crate::Reg<fcfg_b4_ssize0::FCFG_B4_SSIZE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize0;
#[doc = "FCFG_B4_SSIZE1 (rw) register accessor: an alias for `Reg<FCFG_B4_SSIZE1_SPEC>`"]
pub type FCFG_B4_SSIZE1 = crate::Reg<fcfg_b4_ssize1::FCFG_B4_SSIZE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize1;
#[doc = "FCFG_B4_SSIZE2 (rw) register accessor: an alias for `Reg<FCFG_B4_SSIZE2_SPEC>`"]
pub type FCFG_B4_SSIZE2 = crate::Reg<fcfg_b4_ssize2::FCFG_B4_SSIZE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize2;
#[doc = "FCFG_B4_SSIZE3 (rw) register accessor: an alias for `Reg<FCFG_B4_SSIZE3_SPEC>`"]
pub type FCFG_B4_SSIZE3 = crate::Reg<fcfg_b4_ssize3::FCFG_B4_SSIZE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize3;
#[doc = "FCFG_B5_SSIZE0 (rw) register accessor: an alias for `Reg<FCFG_B5_SSIZE0_SPEC>`"]
pub type FCFG_B5_SSIZE0 = crate::Reg<fcfg_b5_ssize0::FCFG_B5_SSIZE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize0;
#[doc = "FCFG_B5_SSIZE1 (rw) register accessor: an alias for `Reg<FCFG_B5_SSIZE1_SPEC>`"]
pub type FCFG_B5_SSIZE1 = crate::Reg<fcfg_b5_ssize1::FCFG_B5_SSIZE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize1;
#[doc = "FCFG_B5_SSIZE2 (rw) register accessor: an alias for `Reg<FCFG_B5_SSIZE2_SPEC>`"]
pub type FCFG_B5_SSIZE2 = crate::Reg<fcfg_b5_ssize2::FCFG_B5_SSIZE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize2;
#[doc = "FCFG_B5_SSIZE3 (rw) register accessor: an alias for `Reg<FCFG_B5_SSIZE3_SPEC>`"]
pub type FCFG_B5_SSIZE3 = crate::Reg<fcfg_b5_ssize3::FCFG_B5_SSIZE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize3;
#[doc = "FCFG_B6_SSIZE0 (rw) register accessor: an alias for `Reg<FCFG_B6_SSIZE0_SPEC>`"]
pub type FCFG_B6_SSIZE0 = crate::Reg<fcfg_b6_ssize0::FCFG_B6_SSIZE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize0;
#[doc = "FCFG_B6_SSIZE1 (rw) register accessor: an alias for `Reg<FCFG_B6_SSIZE1_SPEC>`"]
pub type FCFG_B6_SSIZE1 = crate::Reg<fcfg_b6_ssize1::FCFG_B6_SSIZE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize1;
#[doc = "FCFG_B6_SSIZE2 (rw) register accessor: an alias for `Reg<FCFG_B6_SSIZE2_SPEC>`"]
pub type FCFG_B6_SSIZE2 = crate::Reg<fcfg_b6_ssize2::FCFG_B6_SSIZE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize2;
#[doc = "FCFG_B6_SSIZE3 (rw) register accessor: an alias for `Reg<FCFG_B6_SSIZE3_SPEC>`"]
pub type FCFG_B6_SSIZE3 = crate::Reg<fcfg_b6_ssize3::FCFG_B6_SSIZE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize3;
#[doc = "FCFG_B7_SSIZE0 (rw) register accessor: an alias for `Reg<FCFG_B7_SSIZE0_SPEC>`"]
pub type FCFG_B7_SSIZE0 = crate::Reg<fcfg_b7_ssize0::FCFG_B7_SSIZE0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize0;
#[doc = "FCFG_B7_SSIZE1 (rw) register accessor: an alias for `Reg<FCFG_B7_SSIZE1_SPEC>`"]
pub type FCFG_B7_SSIZE1 = crate::Reg<fcfg_b7_ssize1::FCFG_B7_SSIZE1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize1;
#[doc = "FCFG_B7_SSIZE2 (rw) register accessor: an alias for `Reg<FCFG_B7_SSIZE2_SPEC>`"]
pub type FCFG_B7_SSIZE2 = crate::Reg<fcfg_b7_ssize2::FCFG_B7_SSIZE2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize2;
#[doc = "FCFG_B7_SSIZE3 (rw) register accessor: an alias for `Reg<FCFG_B7_SSIZE3_SPEC>`"]
pub type FCFG_B7_SSIZE3 = crate::Reg<fcfg_b7_ssize3::FCFG_B7_SSIZE3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize3;
