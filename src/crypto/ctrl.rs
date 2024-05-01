#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `AES_START` reader - Starts/initializes AES\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type AesStartR = crate::BitReader;
#[doc = "Field `AES_START` writer - Starts/initializes AES\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type AesStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDES_START` reader - Starts/initializes TDES\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type TdesStartR = crate::BitReader;
#[doc = "Field `TDES_START` writer - Starts/initializes TDES\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type TdesStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_START` reader - Starts/initializes Block Cipher\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type BlockStartR = crate::BitReader;
#[doc = "Field `BLOCK_START` writer - Starts/initializes Block Cipher\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type BlockStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH_START` reader - Starts/initializes HASH/PRNG/HMAC\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type HashStartR = crate::BitReader;
#[doc = "Field `HASH_START` writer - Starts/initializes HASH/PRNG/HMAC\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type HashStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_START` reader - Starts/initializes PKA\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type PkaStartR = crate::BitReader;
#[doc = "Field `PKA_START` writer - Starts/initializes PKA\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type PkaStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_FLUSH` reader - Software write 1 to start Flush Process. The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process. It must last for at least 20 cycles to clean\n\nregisters and FSM"]
pub type BlockFlushR = crate::BitReader;
#[doc = "Field `BLOCK_FLUSH` writer - Software write 1 to start Flush Process. The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process. It must last for at least 20 cycles to clean\n\nregisters and FSM"]
pub type BlockFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH_FLUSH` reader - Software write 1 to start Flush Process. The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process"]
pub type HashFlushR = crate::BitReader;
#[doc = "Field `HASH_FLUSH` writer - Software write 1 to start Flush Process. The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process"]
pub type HashFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_FLUSH` reader - Software write 1 to start Flush Process.The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process"]
pub type PkaFlushR = crate::BitReader;
#[doc = "Field `PKA_FLUSH` writer - Software write 1 to start Flush Process.The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process"]
pub type PkaFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_START` reader - Start TRNG\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type TrngStartR = crate::BitReader;
#[doc = "Field `TRNG_START` writer - Start TRNG\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type TrngStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG_FLUSH` reader - Flush TRNG\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type TrngFlushR = crate::BitReader;
#[doc = "Field `TRNG_FLUSH` writer - Flush TRNG\n\nSoftware write 1 to start. When finishes, the core will clear it."]
pub type TrngFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` reader - Write_Mask"]
pub type WriteMaskR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Starts/initializes AES\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    pub fn aes_start(&self) -> AesStartR {
        AesStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Starts/initializes TDES\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    pub fn tdes_start(&self) -> TdesStartR {
        TdesStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Starts/initializes Block Cipher\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    pub fn block_start(&self) -> BlockStartR {
        BlockStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Starts/initializes HASH/PRNG/HMAC\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    pub fn hash_start(&self) -> HashStartR {
        HashStartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Starts/initializes PKA\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    pub fn pka_start(&self) -> PkaStartR {
        PkaStartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software write 1 to start Flush Process. The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process. It must last for at least 20 cycles to clean\n\nregisters and FSM"]
    #[inline(always)]
    pub fn block_flush(&self) -> BlockFlushR {
        BlockFlushR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software write 1 to start Flush Process. The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process"]
    #[inline(always)]
    pub fn hash_flush(&self) -> HashFlushR {
        HashFlushR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software write 1 to start Flush Process.The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process"]
    #[inline(always)]
    pub fn pka_flush(&self) -> PkaFlushR {
        PkaFlushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start TRNG\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    pub fn trng_start(&self) -> TrngStartR {
        TrngStartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flush TRNG\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    pub fn trng_flush(&self) -> TrngFlushR {
        TrngFlushR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Write_Mask"]
    #[inline(always)]
    pub fn write_mask(&self) -> WriteMaskR {
        WriteMaskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Starts/initializes AES\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    #[must_use]
    pub fn aes_start(&mut self) -> AesStartW<CtrlSpec> {
        AesStartW::new(self, 0)
    }
    #[doc = "Bit 1 - Starts/initializes TDES\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    #[must_use]
    pub fn tdes_start(&mut self) -> TdesStartW<CtrlSpec> {
        TdesStartW::new(self, 1)
    }
    #[doc = "Bit 2 - Starts/initializes Block Cipher\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    #[must_use]
    pub fn block_start(&mut self) -> BlockStartW<CtrlSpec> {
        BlockStartW::new(self, 2)
    }
    #[doc = "Bit 3 - Starts/initializes HASH/PRNG/HMAC\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    #[must_use]
    pub fn hash_start(&mut self) -> HashStartW<CtrlSpec> {
        HashStartW::new(self, 3)
    }
    #[doc = "Bit 4 - Starts/initializes PKA\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    #[must_use]
    pub fn pka_start(&mut self) -> PkaStartW<CtrlSpec> {
        PkaStartW::new(self, 4)
    }
    #[doc = "Bit 5 - Software write 1 to start Flush Process. The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process. It must last for at least 20 cycles to clean\n\nregisters and FSM"]
    #[inline(always)]
    #[must_use]
    pub fn block_flush(&mut self) -> BlockFlushW<CtrlSpec> {
        BlockFlushW::new(self, 5)
    }
    #[doc = "Bit 6 - Software write 1 to start Flush Process. The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process"]
    #[inline(always)]
    #[must_use]
    pub fn hash_flush(&mut self) -> HashFlushW<CtrlSpec> {
        HashFlushW::new(self, 6)
    }
    #[doc = "Bit 7 - Software write 1 to start Flush Process.The process will clear\n\nBRFIFO, BTFIFO, and state machine. Then Software should write 0\n\nto end FLUSH Process"]
    #[inline(always)]
    #[must_use]
    pub fn pka_flush(&mut self) -> PkaFlushW<CtrlSpec> {
        PkaFlushW::new(self, 7)
    }
    #[doc = "Bit 8 - Start TRNG\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    #[must_use]
    pub fn trng_start(&mut self) -> TrngStartW<CtrlSpec> {
        TrngStartW::new(self, 8)
    }
    #[doc = "Bit 9 - Flush TRNG\n\nSoftware write 1 to start. When finishes, the core will clear it."]
    #[inline(always)]
    #[must_use]
    pub fn trng_flush(&mut self) -> TrngFlushW<CtrlSpec> {
        TrngFlushW::new(self, 9)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
