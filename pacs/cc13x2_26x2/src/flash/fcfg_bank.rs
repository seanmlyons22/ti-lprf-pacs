#[doc = "Register `FCFG_BANK` reader"]
pub type R = crate::R<FcfgBankSpec>;
#[doc = "Register `FCFG_BANK` writer"]
pub type W = crate::W<FcfgBankSpec>;
#[doc = "Field `MAIN_NUM_BANK` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type MainNumBankR = crate::FieldReader;
#[doc = "Field `MAIN_NUM_BANK` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type MainNumBankW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAIN_BANK_WIDTH` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type MainBankWidthR = crate::FieldReader<u16>;
#[doc = "Field `MAIN_BANK_WIDTH` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type MainBankWidthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EE_NUM_BANK` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type EeNumBankR = crate::FieldReader;
#[doc = "Field `EE_NUM_BANK` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type EeNumBankW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE_BANK_WIDTH` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type EeBankWidthR = crate::FieldReader<u16>;
#[doc = "Field `EE_BANK_WIDTH` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type EeBankWidthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_num_bank(&self) -> MainNumBankR {
        MainNumBankR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn main_bank_width(&self) -> MainBankWidthR {
        MainBankWidthR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_num_bank(&self) -> EeNumBankR {
        EeNumBankR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_bank_width(&self) -> EeBankWidthR {
        EeBankWidthR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn main_num_bank(&mut self) -> MainNumBankW<FcfgBankSpec> {
        MainNumBankW::new(self, 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn main_bank_width(&mut self) -> MainBankWidthW<FcfgBankSpec> {
        MainBankWidthW::new(self, 4)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ee_num_bank(&mut self) -> EeNumBankW<FcfgBankSpec> {
        EeNumBankW::new(self, 16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ee_bank_width(&mut self) -> EeBankWidthW<FcfgBankSpec> {
        EeBankWidthW::new(self, 20)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_bank::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_bank::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgBankSpec;
impl crate::RegisterSpec for FcfgBankSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_bank::R`](R) reader structure"]
impl crate::Readable for FcfgBankSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_bank::W`](W) writer structure"]
impl crate::Writable for FcfgBankSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_BANK to value 0x0801"]
impl crate::Resettable for FcfgBankSpec {
    const RESET_VALUE: u32 = 0x0801;
}
