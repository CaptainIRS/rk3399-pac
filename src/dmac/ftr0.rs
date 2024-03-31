#[doc = "Register `FTR0` reader"]
pub type R = crate::R<Ftr0Spec>;
#[doc = "Field `FTR0_BITS_15` reader - Indicates if the DMA channel thread was attempting to execute an\n\nundefined instruction:\n\n0 = defined instruction\n\n1 = undefined instruction.\n\nThis fault is a precise abort"]
pub type Ftr0Bits15R = crate::BitReader;
#[doc = "Field `FTR0_BITS_14` reader - Indicates if the DMA channel thread was attempting to execute an\n\ninstruction operand that was not\n\nvalid for the configuration of the DMAC:\n\n0 = valid operand\n\n1 = invalid operand.\n\nThis fault is a precise abort."]
pub type Ftr0Bits14R = crate::BitReader;
#[doc = "Field `FTR0_BITS_12` reader - Indicates if the DMA channel thread attempts to execute DMAWFE\n\nor DMASEV with inappropriate security permissions:\n\n0 = a DMA channel thread in the Non-secure state is not violating\n\nthe security permissions\n\n1 = a DMA channel thread in the Non-secure state attempted to\n\nexecute either:\n\nDMAWFE to wait for a secure event\n\nDMASEV to create a secure event or secure interrupt.\n\nThis fault is a precise abort."]
pub type Ftr0Bits12R = crate::BitReader;
#[doc = "Field `FTR0_BITS_11` reader - Indicates if a DMA channel thread, in the Non-secure state,\n\nattempts to execute DMAWFP, DMALDP,\n\nDMASTP, or DMAFLUSHP with inappropriate security permissions:\n\n0 = a DMA channel thread in the Non-secure state is not violating\n\nthe security permissions\n\n1 = a DMA channel thread in the Non-secure state attempted to\n\nexecute either:\n\no DMAWFP to wait for a secure peripheral\n\no DMALDP or DMASTP to notify a secure peripheral\n\no DMAFLUSHP to flush a secure peripheral.\n\nThis fault is a precise abort."]
pub type Ftr0Bits11R = crate::BitReader;
#[doc = "Field `FTR0_BITS_10` reader - Indicates if a DMA channel thread, in the Non-secure state,\n\nattempts to program the CCRn Register\n\nto perform a secure read or secure write:\n\n0 = a DMA channel thread in the Non-secure state is not violating\n\nthe security permissions\n\n1 = a DMA channel thread in the Non-secure state attempted to\n\nperform a secure read or secure write.\n\nThis fault is a precise abort"]
pub type Ftr0Bits10R = crate::BitReader;
#[doc = "Field `FTR0_BITS_8` reader - Indicates if the MFIFO prevented the DMA channel thread from\n\nexecuting DMALD or DMAST. Depending on the instruction:\n\nDMALD 0 = MFIFO contains sufficient space\n\n1 = MFIFO is too small to hold the data that DMALD requires.\n\nDMAST 0 = MFIFO contains sufficient data\n\n1 = MFIFO is too small to store the data to enable DMAST to\n\ncomplete.\n\nThis fault is an imprecise abort"]
pub type Ftr0Bits8R = crate::BitReader;
#[doc = "Field `FTR0_BITS_7` reader - Indicates if the MFIFO did not contain the data to enable the DMAC\n\nto perform the DMAST:\n\n0 = MFIFO contains all the data to enable the DMAST to complete\n\n1 = previous DMALDs have not put enough data in the MFIFO to\n\nenable the DMAST to complete.\n\nThis fault is a precise abort."]
pub type Ftr0Bits7R = crate::BitReader;
#[doc = "Field `FTR0_BITS_5` reader - Indicates the AXI response that the DMAC receives on the RRESP\n\nbus, after the DMA channel\n\nthread performs an instruction fetch:\n\n0 = OKAY response\n\n1 = EXOKAY, SLVERR, or DECERR response.\n\nThis fault is a precise abort."]
pub type Ftr0Bits5R = crate::BitReader;
#[doc = "Field `FTR0_BITS_4` reader - Indicates the AXI response that the DMAC receives on the BRESP\n\nbus, after the DMA channel\n\nthread performs a data write:\n\n0 = OKAY response\n\n1 = EXOKAY, SLVERR, or DECERR response.\n\nThis fault is an imprecise abort."]
pub type Ftr0Bits4R = crate::BitReader;
#[doc = "Field `FTR0_BITS_3` reader - Indicates the AXI response that the DMAC receives on the RRESP\n\nbus, after the DMA channel\n\nthread performs a data read:\n\n0 = OKAY response\n\n1 = EXOKAY, SLVERR, or DECERR response.\n\nThis fault is an imprecise abort"]
pub type Ftr0Bits3R = crate::BitReader;
#[doc = "Field `FTR0_BITS_1` reader - If the DMA channel aborts, this bit indicates if the erroneous\n\ninstruction was read from the system\n\nmemory or from the debug interface:\n\n0 = instruction that generated an abort was read from system\n\nmemory\n\n1 = instruction that generated an abort was read from the debug\n\ninterface.\n\nThis fault is an imprecise abort but the bit is only valid when a\n\nprecise abort occurs."]
pub type Ftr0Bits1R = crate::BitReader;
#[doc = "Field `FTR0_BITS_0` reader - Indicates if the DMA channel has locked-up because of resource\n\nstarvation:\n\n0 = DMA channel has adequate resources\n\n1 = DMA channel has locked-up because of insufficient resources.\n\nThis fault is an imprecise abort"]
pub type Ftr0Bits0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if the DMA channel thread was attempting to execute an\n\nundefined instruction:\n\n0 = defined instruction\n\n1 = undefined instruction.\n\nThis fault is a precise abort"]
    #[inline(always)]
    pub fn ftr0_bits_15(&self) -> Ftr0Bits15R {
        Ftr0Bits15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if the DMA channel thread was attempting to execute an\n\ninstruction operand that was not\n\nvalid for the configuration of the DMAC:\n\n0 = valid operand\n\n1 = invalid operand.\n\nThis fault is a precise abort."]
    #[inline(always)]
    pub fn ftr0_bits_14(&self) -> Ftr0Bits14R {
        Ftr0Bits14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if the DMA channel thread attempts to execute DMAWFE\n\nor DMASEV with inappropriate security permissions:\n\n0 = a DMA channel thread in the Non-secure state is not violating\n\nthe security permissions\n\n1 = a DMA channel thread in the Non-secure state attempted to\n\nexecute either:\n\nDMAWFE to wait for a secure event\n\nDMASEV to create a secure event or secure interrupt.\n\nThis fault is a precise abort."]
    #[inline(always)]
    pub fn ftr0_bits_12(&self) -> Ftr0Bits12R {
        Ftr0Bits12R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates if a DMA channel thread, in the Non-secure state,\n\nattempts to execute DMAWFP, DMALDP,\n\nDMASTP, or DMAFLUSHP with inappropriate security permissions:\n\n0 = a DMA channel thread in the Non-secure state is not violating\n\nthe security permissions\n\n1 = a DMA channel thread in the Non-secure state attempted to\n\nexecute either:\n\no DMAWFP to wait for a secure peripheral\n\no DMALDP or DMASTP to notify a secure peripheral\n\no DMAFLUSHP to flush a secure peripheral.\n\nThis fault is a precise abort."]
    #[inline(always)]
    pub fn ftr0_bits_11(&self) -> Ftr0Bits11R {
        Ftr0Bits11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates if a DMA channel thread, in the Non-secure state,\n\nattempts to program the CCRn Register\n\nto perform a secure read or secure write:\n\n0 = a DMA channel thread in the Non-secure state is not violating\n\nthe security permissions\n\n1 = a DMA channel thread in the Non-secure state attempted to\n\nperform a secure read or secure write.\n\nThis fault is a precise abort"]
    #[inline(always)]
    pub fn ftr0_bits_10(&self) -> Ftr0Bits10R {
        Ftr0Bits10R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates if the MFIFO prevented the DMA channel thread from\n\nexecuting DMALD or DMAST. Depending on the instruction:\n\nDMALD 0 = MFIFO contains sufficient space\n\n1 = MFIFO is too small to hold the data that DMALD requires.\n\nDMAST 0 = MFIFO contains sufficient data\n\n1 = MFIFO is too small to store the data to enable DMAST to\n\ncomplete.\n\nThis fault is an imprecise abort"]
    #[inline(always)]
    pub fn ftr0_bits_8(&self) -> Ftr0Bits8R {
        Ftr0Bits8R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates if the MFIFO did not contain the data to enable the DMAC\n\nto perform the DMAST:\n\n0 = MFIFO contains all the data to enable the DMAST to complete\n\n1 = previous DMALDs have not put enough data in the MFIFO to\n\nenable the DMAST to complete.\n\nThis fault is a precise abort."]
    #[inline(always)]
    pub fn ftr0_bits_7(&self) -> Ftr0Bits7R {
        Ftr0Bits7R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates the AXI response that the DMAC receives on the RRESP\n\nbus, after the DMA channel\n\nthread performs an instruction fetch:\n\n0 = OKAY response\n\n1 = EXOKAY, SLVERR, or DECERR response.\n\nThis fault is a precise abort."]
    #[inline(always)]
    pub fn ftr0_bits_5(&self) -> Ftr0Bits5R {
        Ftr0Bits5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates the AXI response that the DMAC receives on the BRESP\n\nbus, after the DMA channel\n\nthread performs a data write:\n\n0 = OKAY response\n\n1 = EXOKAY, SLVERR, or DECERR response.\n\nThis fault is an imprecise abort."]
    #[inline(always)]
    pub fn ftr0_bits_4(&self) -> Ftr0Bits4R {
        Ftr0Bits4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates the AXI response that the DMAC receives on the RRESP\n\nbus, after the DMA channel\n\nthread performs a data read:\n\n0 = OKAY response\n\n1 = EXOKAY, SLVERR, or DECERR response.\n\nThis fault is an imprecise abort"]
    #[inline(always)]
    pub fn ftr0_bits_3(&self) -> Ftr0Bits3R {
        Ftr0Bits3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - If the DMA channel aborts, this bit indicates if the erroneous\n\ninstruction was read from the system\n\nmemory or from the debug interface:\n\n0 = instruction that generated an abort was read from system\n\nmemory\n\n1 = instruction that generated an abort was read from the debug\n\ninterface.\n\nThis fault is an imprecise abort but the bit is only valid when a\n\nprecise abort occurs."]
    #[inline(always)]
    pub fn ftr0_bits_1(&self) -> Ftr0Bits1R {
        Ftr0Bits1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates if the DMA channel has locked-up because of resource\n\nstarvation:\n\n0 = DMA channel has adequate resources\n\n1 = DMA channel has locked-up because of insufficient resources.\n\nThis fault is an imprecise abort"]
    #[inline(always)]
    pub fn ftr0_bits_0(&self) -> Ftr0Bits0R {
        Ftr0Bits0R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Fault Type DMA Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ftr0Spec;
impl crate::RegisterSpec for Ftr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftr0::R`](R) reader structure"]
impl crate::Readable for Ftr0Spec {}
#[doc = "`reset()` method sets FTR0 to value 0"]
impl crate::Resettable for Ftr0Spec {
    const RESET_VALUE: u32 = 0;
}
