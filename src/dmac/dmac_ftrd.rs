#[doc = "Register `DMAC_FTRD` reader"]
pub type R = crate::R<DmacFtrdSpec>;
#[doc = "Register `DMAC_FTRD` writer"]
pub type W = crate::W<DmacFtrdSpec>;
#[doc = "Field `DMAC_FTRD_BITS_9` reader - Indicates if the DMA manager was attempting to execute an\n\nundefined instruction:\n\n0 = defined instruction\n\n1 = undefined instruction."]
pub type DmacFtrdBits9R = crate::BitReader;
#[doc = "Field `DMAC_FTRD_BITS_9` writer - Indicates if the DMA manager was attempting to execute an\n\nundefined instruction:\n\n0 = defined instruction\n\n1 = undefined instruction."]
pub type DmacFtrdBits9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC_FTRD_BITS_8` reader - Indicates if the DMA manager was attempting to execute an\n\ninstruction operand that was not valid for\n\nthe configuration of the DMAC:\n\n0 = valid operand\n\n1 = invalid operand."]
pub type DmacFtrdBits8R = crate::BitReader;
#[doc = "Field `DMAC_FTRD_BITS_6` reader - Indicates if the DMA manager was attempting to execute DMAGO\n\nwith inappropriate security permissions:\n\n0 = DMA manager has appropriate security to execute DMAGO\n\n1 = DMA manager thread in the Non-secure state attempted to\n\nexecute DMAGO to create a DMA channel operating in the Secure\n\nstate."]
pub type DmacFtrdBits6R = crate::BitReader;
#[doc = "Field `DMAC_FTRD_BITS_5` reader - Indicates if the DMA manager was attempting to execute DMAWFE\n\nor DMASEV with inappropriate security permissions:\n\n0 = DMA manager has appropriate security to execute DMAWFE or\n\nDMASEV\n\n1 = a DMA manager thread in the Non-secure state attempted to\n\nexecute either:\n\nDMAWFE to wait for a secure event\n\nDMASEV to create a secure event or secure interrupt"]
pub type DmacFtrdBits5R = crate::BitReader;
#[doc = "Field `DMAC_FTRD_BITS_3` reader - Indicates the AXI response that the DMAC receives on the RRESP\n\nbus, after the DMA manager\n\nperforms an instruction fetch:\n\n0 = OKAY response\n\n1 = EXOKAY, SLVERR, or DECERR response"]
pub type DmacFtrdBits3R = crate::BitReader;
#[doc = "Field `DMAC_FTRD_BITS_1` reader - If the DMA manager aborts, this bit indicates if the erroneous\n\ninstruction was read from the system\n\nmemory or from the debug interface:\n\n0 = instruction that generated an abort was read from system\n\nmemory\n\n1 = instruction that generated an abort was read from the debug\n\ninterface."]
pub type DmacFtrdBits1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if the DMA manager was attempting to execute an\n\nundefined instruction:\n\n0 = defined instruction\n\n1 = undefined instruction."]
    #[inline(always)]
    pub fn dmac_ftrd_bits_9(&self) -> DmacFtrdBits9R {
        DmacFtrdBits9R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if the DMA manager was attempting to execute an\n\ninstruction operand that was not valid for\n\nthe configuration of the DMAC:\n\n0 = valid operand\n\n1 = invalid operand."]
    #[inline(always)]
    pub fn dmac_ftrd_bits_8(&self) -> DmacFtrdBits8R {
        DmacFtrdBits8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates if the DMA manager was attempting to execute DMAGO\n\nwith inappropriate security permissions:\n\n0 = DMA manager has appropriate security to execute DMAGO\n\n1 = DMA manager thread in the Non-secure state attempted to\n\nexecute DMAGO to create a DMA channel operating in the Secure\n\nstate."]
    #[inline(always)]
    pub fn dmac_ftrd_bits_6(&self) -> DmacFtrdBits6R {
        DmacFtrdBits6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if the DMA manager was attempting to execute DMAWFE\n\nor DMASEV with inappropriate security permissions:\n\n0 = DMA manager has appropriate security to execute DMAWFE or\n\nDMASEV\n\n1 = a DMA manager thread in the Non-secure state attempted to\n\nexecute either:\n\nDMAWFE to wait for a secure event\n\nDMASEV to create a secure event or secure interrupt"]
    #[inline(always)]
    pub fn dmac_ftrd_bits_5(&self) -> DmacFtrdBits5R {
        DmacFtrdBits5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates the AXI response that the DMAC receives on the RRESP\n\nbus, after the DMA manager\n\nperforms an instruction fetch:\n\n0 = OKAY response\n\n1 = EXOKAY, SLVERR, or DECERR response"]
    #[inline(always)]
    pub fn dmac_ftrd_bits_3(&self) -> DmacFtrdBits3R {
        DmacFtrdBits3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - If the DMA manager aborts, this bit indicates if the erroneous\n\ninstruction was read from the system\n\nmemory or from the debug interface:\n\n0 = instruction that generated an abort was read from system\n\nmemory\n\n1 = instruction that generated an abort was read from the debug\n\ninterface."]
    #[inline(always)]
    pub fn dmac_ftrd_bits_1(&self) -> DmacFtrdBits1R {
        DmacFtrdBits1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if the DMA manager was attempting to execute an\n\nundefined instruction:\n\n0 = defined instruction\n\n1 = undefined instruction."]
    #[inline(always)]
    #[must_use]
    pub fn dmac_ftrd_bits_9(&mut self) -> DmacFtrdBits9W<DmacFtrdSpec> {
        DmacFtrdBits9W::new(self, 0)
    }
}
#[doc = "Fault Type DMA Manager Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ftrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_ftrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacFtrdSpec;
impl crate::RegisterSpec for DmacFtrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ftrd::R`](R) reader structure"]
impl crate::Readable for DmacFtrdSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac_ftrd::W`](W) writer structure"]
impl crate::Writable for DmacFtrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_FTRD to value 0"]
impl crate::Resettable for DmacFtrdSpec {
    const RESET_VALUE: u32 = 0;
}
