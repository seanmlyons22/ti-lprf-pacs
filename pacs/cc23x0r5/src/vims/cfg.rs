#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "0:0\\]
This bit is used to write protect flash 1T $amp; 2T waitstate registers, flash charge pump and bank trim registers, TRMVLID and ATTEST configuration registers. This register is sticky when written with value 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weprtrm {
    #[doc = "1: Allow"]
    Allow = 1,
    #[doc = "0: Block"]
    Restrict = 0,
}
impl From<Weprtrm> for bool {
    #[inline(always)]
    fn from(variant: Weprtrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WEPRTRM` reader - 0:0\\]
This bit is used to write protect flash 1T $amp; 2T waitstate registers, flash charge pump and bank trim registers, TRMVLID and ATTEST configuration registers. This register is sticky when written with value 0."]
pub type WeprtrmR = crate::BitReader<Weprtrm>;
impl WeprtrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Weprtrm {
        match self.bits {
            true => Weprtrm::Allow,
            false => Weprtrm::Restrict,
        }
    }
    #[doc = "Allow"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        *self == Weprtrm::Allow
    }
    #[doc = "Block"]
    #[inline(always)]
    pub fn is_restrict(&self) -> bool {
        *self == Weprtrm::Restrict
    }
}
#[doc = "Field `WEPRTRM` writer - 0:0\\]
This bit is used to write protect flash 1T $amp; 2T waitstate registers, flash charge pump and bank trim registers, TRMVLID and ATTEST configuration registers. This register is sticky when written with value 0."]
pub type WeprtrmW<'a, REG> = crate::BitWriter<'a, REG, Weprtrm>;
impl<'a, REG> WeprtrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow"]
    #[inline(always)]
    pub fn allow(self) -> &'a mut crate::W<REG> {
        self.variant(Weprtrm::Allow)
    }
    #[doc = "Block"]
    #[inline(always)]
    pub fn restrict(self) -> &'a mut crate::W<REG> {
        self.variant(Weprtrm::Restrict)
    }
}
#[doc = "Field `TRMVLID` reader - 1:1\\]
This bit indicates if flash charge pump and bank trim values are valid."]
pub type TrmvlidR = crate::BitReader;
#[doc = "Field `TRMVLID` writer - 1:1\\]
This bit indicates if flash charge pump and bank trim values are valid."]
pub type TrmvlidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTEST` reader - 2:2\\]
This bit is used to enable flash test mode."]
pub type AttestR = crate::BitReader;
#[doc = "Field `ATTEST` writer - 2:2\\]
This bit is used to enable flash test mode."]
pub type AttestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is used to write protect flash 1T $amp; 2T waitstate registers, flash charge pump and bank trim registers, TRMVLID and ATTEST configuration registers. This register is sticky when written with value 0."]
    #[inline(always)]
    pub fn weprtrm(&self) -> WeprtrmR {
        WeprtrmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit indicates if flash charge pump and bank trim values are valid."]
    #[inline(always)]
    pub fn trmvlid(&self) -> TrmvlidR {
        TrmvlidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is used to enable flash test mode."]
    #[inline(always)]
    pub fn attest(&self) -> AttestR {
        AttestR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit is used to write protect flash 1T $amp; 2T waitstate registers, flash charge pump and bank trim registers, TRMVLID and ATTEST configuration registers. This register is sticky when written with value 0."]
    #[inline(always)]
    #[must_use]
    pub fn weprtrm(&mut self) -> WeprtrmW<CfgSpec> {
        WeprtrmW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit indicates if flash charge pump and bank trim values are valid."]
    #[inline(always)]
    #[must_use]
    pub fn trmvlid(&mut self) -> TrmvlidW<CfgSpec> {
        TrmvlidW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is used to enable flash test mode."]
    #[inline(always)]
    #[must_use]
    pub fn attest(&mut self) -> AttestW<CfgSpec> {
        AttestW::new(self, 2)
    }
}
#[doc = "This register is used for flash configuration. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x01"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
