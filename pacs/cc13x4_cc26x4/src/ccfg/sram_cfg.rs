#[doc = "Register `SRAM_CFG` reader"]
pub type R = crate::R<SramCfgSpec>;
#[doc = "Register `SRAM_CFG` writer"]
pub type W = crate::W<SramCfgSpec>;
#[doc = "Field `PARITY_DIS` reader - 0:0\\]
Value will be inverted and then written to PRCM:MCUSRAMCFG.PARITY_EN by ROM boot FW"]
pub type ParityDisR = crate::BitReader;
#[doc = "Field `PARITY_DIS` writer - 0:0\\]
Value will be inverted and then written to PRCM:MCUSRAMCFG.PARITY_EN by ROM boot FW"]
pub type ParityDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SEL` reader - 31:8\\]
Value will be written to SRAM_MMR:MEM_CTL.MEM_SEL by ROM boot FW"]
pub type MemSelR = crate::FieldReader<u32>;
#[doc = "Field `MEM_SEL` writer - 31:8\\]
Value will be written to SRAM_MMR:MEM_CTL.MEM_SEL by ROM boot FW"]
pub type MemSelW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Value will be inverted and then written to PRCM:MCUSRAMCFG.PARITY_EN by ROM boot FW"]
    #[inline(always)]
    pub fn parity_dis(&self) -> ParityDisR {
        ParityDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Value will be written to SRAM_MMR:MEM_CTL.MEM_SEL by ROM boot FW"]
    #[inline(always)]
    pub fn mem_sel(&self) -> MemSelR {
        MemSelR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Value will be inverted and then written to PRCM:MCUSRAMCFG.PARITY_EN by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn parity_dis(&mut self) -> ParityDisW<SramCfgSpec> {
        ParityDisW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Value will be written to SRAM_MMR:MEM_CTL.MEM_SEL by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn mem_sel(&mut self) -> MemSelW<SramCfgSpec> {
        MemSelW::new(self, 8)
    }
}
#[doc = "Configuration register for MCU SRAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramCfgSpec;
impl crate::RegisterSpec for SramCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cfg::R`](R) reader structure"]
impl crate::Readable for SramCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_cfg::W`](W) writer structure"]
impl crate::Writable for SramCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_CFG to value 0xffff_ffff"]
impl crate::Resettable for SramCfgSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
