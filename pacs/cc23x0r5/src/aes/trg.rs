#[doc = "Register `TRG` reader"]
pub type R = crate::R<TrgSpec>;
#[doc = "Register `TRG` writer"]
pub type W = crate::W<TrgSpec>;
#[doc = "1:0\\]
Electronic Codebook (ECB) Operation Write an enumerated value to this field when STA.STATE = IDLE to manually trigger an ECB encryption. If condition is not met, the trigger is ignored. Non-enumerated values are ignored. Enumerated value indicates source of ECB operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ecbop {
    #[doc = "3: TXT = ECB(KEY, TXT XOR BUF)"]
    Txtxbuf = 3,
    #[doc = "2: TXT = ECB(KEY,BUF)"]
    Buf = 2,
    #[doc = "1: TXT = ECB(KEY,TXT)"]
    Txt = 1,
}
impl From<Ecbop> for u8 {
    #[inline(always)]
    fn from(variant: Ecbop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ecbop {
    type Ux = u8;
}
impl crate::IsEnum for Ecbop {}
#[doc = "Field `ECBOP` writer - 1:0\\]
Electronic Codebook (ECB) Operation Write an enumerated value to this field when STA.STATE = IDLE to manually trigger an ECB encryption. If condition is not met, the trigger is ignored. Non-enumerated values are ignored. Enumerated value indicates source of ECB operation"]
pub type EcbopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ecbop>;
impl<'a, REG> EcbopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXT = ECB(KEY, TXT XOR BUF)"]
    #[inline(always)]
    pub fn txtxbuf(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbop::Txtxbuf)
    }
    #[doc = "TXT = ECB(KEY,BUF)"]
    #[inline(always)]
    pub fn buf(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbop::Buf)
    }
    #[doc = "TXT = ECB(KEY,TXT)"]
    #[inline(always)]
    pub fn txt(self) -> &'a mut crate::W<REG> {
        self.variant(Ecbop::Txt)
    }
}
#[doc = "Field `DMACHB` writer - 2:2\\]
Write 1 to this field to manually trigger channel B request."]
pub type DmachbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACHA` writer - 3:3\\]
Write 1 to this field to manually trigger channel A request."]
pub type DmachaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` writer - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Electronic Codebook (ECB) Operation Write an enumerated value to this field when STA.STATE = IDLE to manually trigger an ECB encryption. If condition is not met, the trigger is ignored. Non-enumerated values are ignored. Enumerated value indicates source of ECB operation"]
    #[inline(always)]
    #[must_use]
    pub fn ecbop(&mut self) -> EcbopW<TrgSpec> {
        EcbopW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to this field to manually trigger channel B request."]
    #[inline(always)]
    #[must_use]
    pub fn dmachb(&mut self) -> DmachbW<TrgSpec> {
        DmachbW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to this field to manually trigger channel A request."]
    #[inline(always)]
    #[must_use]
    pub fn dmacha(&mut self) -> DmachaW<TrgSpec> {
        DmachaW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<TrgSpec> {
        Reserved2W::new(self, 4)
    }
}
#[doc = "Trigger Use this register to manually trigger operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrgSpec;
impl crate::RegisterSpec for TrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trg::R`](R) reader structure"]
impl crate::Readable for TrgSpec {}
#[doc = "`write(|w| ..)` method takes [`trg::W`](W) writer structure"]
impl crate::Writable for TrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRG to value 0"]
impl crate::Resettable for TrgSpec {
    const RESET_VALUE: u32 = 0;
}
