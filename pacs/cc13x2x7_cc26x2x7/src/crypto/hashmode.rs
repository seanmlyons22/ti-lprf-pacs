#[doc = "Register `HASHMODE` reader"]
pub type R = crate::R<HashmodeSpec>;
#[doc = "Register `HASHMODE` writer"]
pub type W = crate::W<HashmodeSpec>;
#[doc = "Field `NEW_HASH` reader - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
pub type NewHashR = crate::BitReader;
#[doc = "Field `NEW_HASH` writer - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
pub type NewHashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 2:1\\]
Write 0s and ignore on reading"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 2:1\\]
Write 0s and ignore on reading"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SHA256_MODE` reader - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
pub type Sha256ModeR = crate::BitReader;
#[doc = "Field `SHA256_MODE` writer - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
pub type Sha256ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA224_MODE` reader - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
pub type Sha224ModeR = crate::BitReader;
#[doc = "Field `SHA224_MODE` writer - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
pub type Sha224ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA512_MODE` reader - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
pub type Sha512ModeR = crate::BitReader;
#[doc = "Field `SHA512_MODE` writer - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
pub type Sha512ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA384_MODE` reader - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
pub type Sha384ModeR = crate::BitReader;
#[doc = "Field `SHA384_MODE` writer - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
pub type Sha384ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 31:7\\]
Write 0s and ignore on reading"]
pub type Reserved7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Write 0s and ignore on reading"]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    pub fn new_hash(&self) -> NewHashR {
        NewHashR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
    #[inline(always)]
    pub fn sha256_mode(&self) -> Sha256ModeR {
        Sha256ModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
    #[inline(always)]
    pub fn sha224_mode(&self) -> Sha224ModeR {
        Sha224ModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
    #[inline(always)]
    pub fn sha512_mode(&self) -> Sha512ModeR {
        Sha512ModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
    #[inline(always)]
    pub fn sha384_mode(&self) -> Sha384ModeR {
        Sha384ModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set to 1, it indicates that the hash engine must start processing a new hash session. The \\[HASHDIGESTn.* \\]
registers will automatically be loaded with the initial hash algorithm constants of the selected hash algorithm. When this bit is 0 while the hash processing is started, the initial hash algorithm constants are not loaded in the HASHDIGESTn registers. The hash engine will start processing with the digest that is currently in its internal HASHDIGESTn registers. This bit is automatically cleared when hash processing is started."]
    #[inline(always)]
    #[must_use]
    pub fn new_hash(&mut self) -> NewHashW<HashmodeSpec> {
        NewHashW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<HashmodeSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
The host must write this bit with 1 prior to processing a SHA 256 session."]
    #[inline(always)]
    #[must_use]
    pub fn sha256_mode(&mut self) -> Sha256ModeW<HashmodeSpec> {
        Sha256ModeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
The host must write this bit with 1 prior to processing a SHA 224 session."]
    #[inline(always)]
    #[must_use]
    pub fn sha224_mode(&mut self) -> Sha224ModeW<HashmodeSpec> {
        Sha224ModeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
The host must write this bit with 1 prior to processing a SHA 512 session."]
    #[inline(always)]
    #[must_use]
    pub fn sha512_mode(&mut self) -> Sha512ModeW<HashmodeSpec> {
        Sha512ModeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
The host must write this bit with 1 prior to processing a SHA 384 session."]
    #[inline(always)]
    #[must_use]
    pub fn sha384_mode(&mut self) -> Sha384ModeW<HashmodeSpec> {
        Sha384ModeW::new(self, 6)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Write 0s and ignore on reading"]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<HashmodeSpec> {
        Reserved7W::new(self, 7)
    }
}
#[doc = "HASH Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashmodeSpec;
impl crate::RegisterSpec for HashmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashmode::R`](R) reader structure"]
impl crate::Readable for HashmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`hashmode::W`](W) writer structure"]
impl crate::Writable for HashmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASHMODE to value 0"]
impl crate::Resettable for HashmodeSpec {
    const RESET_VALUE: u32 = 0;
}
