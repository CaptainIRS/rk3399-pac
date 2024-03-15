#[doc = "Register `DMAC_FTR0` reader"]
pub type R = crate::R<DmacFtr0Spec>;
#[doc = "Field `DMAC_FTR0_BITS_15` reader - Indicates if the DMA channel thread was attempting to execute an undefined instruction: 0 = defined instruction 1 = undefined instruction. This fault is a precise abort"]
pub type DmacFtr0Bits15R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_14` reader - Indicates if the DMA channel thread was attempting to execute an instruction operand that was not valid for the configuration of the DMAC: 0 = valid operand 1 = invalid operand. This fault is a precise abort."]
pub type DmacFtr0Bits14R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_12` reader - Indicates if the DMA channel thread attempts to execute DMAWFE or DMASEV with inappropriate security permissions: 0 = a DMA channel thread in the Non-secure state is not violating the security permissions 1 = a DMA channel thread in the Non-secure state attempted to execute either: DMAWFE to wait for a secure event DMASEV to create a secure event or secure interrupt. This fault is a precise abort."]
pub type DmacFtr0Bits12R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_11` reader - Indicates if a DMA channel thread, in the Non-secure state, attempts to execute DMAWFP, DMALDP, DMASTP, or DMAFLUSHP with inappropriate security permissions: 0 = a DMA channel thread in the Non-secure state is not violating the security permissions 1 = a DMA channel thread in the Non-secure state attempted to execute either: o DMAWFP to wait for a secure peripheral o DMALDP or DMASTP to notify a secure peripheral o DMAFLUSHP to flush a secure peripheral. This fault is a precise abort."]
pub type DmacFtr0Bits11R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_10` reader - Indicates if a DMA channel thread, in the Non-secure state, attempts to program the CCRn Register to perform a secure read or secure write: 0 = a DMA channel thread in the Non-secure state is not violating the security permissions 1 = a DMA channel thread in the Non-secure state attempted to perform a secure read or secure write. This fault is a precise abort"]
pub type DmacFtr0Bits10R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_8` reader - Indicates if the MFIFO prevented the DMA channel thread from executing DMALD or DMAST. Depending on the instruction: DMALD 0 = MFIFO contains sufficient space 1 = MFIFO is too small to hold the data that DMALD requires. DMAST 0 = MFIFO contains sufficient data 1 = MFIFO is too small to store the data to enable DMAST to complete. This fault is an imprecise abort"]
pub type DmacFtr0Bits8R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_7` reader - Indicates if the MFIFO did not contain the data to enable the DMAC to perform the DMAST: 0 = MFIFO contains all the data to enable the DMAST to complete 1 = previous DMALDs have not put enough data in the MFIFO to enable the DMAST to complete. This fault is a precise abort."]
pub type DmacFtr0Bits7R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_5` reader - Indicates the AXI response that the DMAC receives on the RRESP bus, after the DMA channel thread performs an instruction fetch: 0 = OKAY response 1 = EXOKAY, SLVERR, or DECERR response. This fault is a precise abort."]
pub type DmacFtr0Bits5R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_4` reader - Indicates the AXI response that the DMAC receives on the BRESP bus, after the DMA channel thread performs a data write: 0 = OKAY response 1 = EXOKAY, SLVERR, or DECERR response. This fault is an imprecise abort."]
pub type DmacFtr0Bits4R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_3` reader - Indicates the AXI response that the DMAC receives on the RRESP bus, after the DMA channel thread performs a data read: 0 = OKAY response 1 = EXOKAY, SLVERR, or DECERR response. This fault is an imprecise abort"]
pub type DmacFtr0Bits3R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_1` reader - If the DMA channel aborts, this bit indicates if the erroneous instruction was read from the system memory or from the debug interface: 0 = instruction that generated an abort was read from system memory 1 = instruction that generated an abort was read from the debug interface. This fault is an imprecise abort but the bit is only valid when a precise abort occurs."]
pub type DmacFtr0Bits1R = crate::BitReader;
#[doc = "Field `DMAC_FTR0_BITS_0` reader - Indicates if the DMA channel has locked-up because of resource starvation: 0 = DMA channel has adequate resources 1 = DMA channel has locked-up because of insufficient resources. This fault is an imprecise abort"]
pub type DmacFtr0Bits0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if the DMA channel thread was attempting to execute an undefined instruction: 0 = defined instruction 1 = undefined instruction. This fault is a precise abort"]
    #[inline(always)]
    pub fn dmac_ftr0_bits_15(&self) -> DmacFtr0Bits15R {
        DmacFtr0Bits15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if the DMA channel thread was attempting to execute an instruction operand that was not valid for the configuration of the DMAC: 0 = valid operand 1 = invalid operand. This fault is a precise abort."]
    #[inline(always)]
    pub fn dmac_ftr0_bits_14(&self) -> DmacFtr0Bits14R {
        DmacFtr0Bits14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if the DMA channel thread attempts to execute DMAWFE or DMASEV with inappropriate security permissions: 0 = a DMA channel thread in the Non-secure state is not violating the security permissions 1 = a DMA channel thread in the Non-secure state attempted to execute either: DMAWFE to wait for a secure event DMASEV to create a secure event or secure interrupt. This fault is a precise abort."]
    #[inline(always)]
    pub fn dmac_ftr0_bits_12(&self) -> DmacFtr0Bits12R {
        DmacFtr0Bits12R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates if a DMA channel thread, in the Non-secure state, attempts to execute DMAWFP, DMALDP, DMASTP, or DMAFLUSHP with inappropriate security permissions: 0 = a DMA channel thread in the Non-secure state is not violating the security permissions 1 = a DMA channel thread in the Non-secure state attempted to execute either: o DMAWFP to wait for a secure peripheral o DMALDP or DMASTP to notify a secure peripheral o DMAFLUSHP to flush a secure peripheral. This fault is a precise abort."]
    #[inline(always)]
    pub fn dmac_ftr0_bits_11(&self) -> DmacFtr0Bits11R {
        DmacFtr0Bits11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates if a DMA channel thread, in the Non-secure state, attempts to program the CCRn Register to perform a secure read or secure write: 0 = a DMA channel thread in the Non-secure state is not violating the security permissions 1 = a DMA channel thread in the Non-secure state attempted to perform a secure read or secure write. This fault is a precise abort"]
    #[inline(always)]
    pub fn dmac_ftr0_bits_10(&self) -> DmacFtr0Bits10R {
        DmacFtr0Bits10R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates if the MFIFO prevented the DMA channel thread from executing DMALD or DMAST. Depending on the instruction: DMALD 0 = MFIFO contains sufficient space 1 = MFIFO is too small to hold the data that DMALD requires. DMAST 0 = MFIFO contains sufficient data 1 = MFIFO is too small to store the data to enable DMAST to complete. This fault is an imprecise abort"]
    #[inline(always)]
    pub fn dmac_ftr0_bits_8(&self) -> DmacFtr0Bits8R {
        DmacFtr0Bits8R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates if the MFIFO did not contain the data to enable the DMAC to perform the DMAST: 0 = MFIFO contains all the data to enable the DMAST to complete 1 = previous DMALDs have not put enough data in the MFIFO to enable the DMAST to complete. This fault is a precise abort."]
    #[inline(always)]
    pub fn dmac_ftr0_bits_7(&self) -> DmacFtr0Bits7R {
        DmacFtr0Bits7R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates the AXI response that the DMAC receives on the RRESP bus, after the DMA channel thread performs an instruction fetch: 0 = OKAY response 1 = EXOKAY, SLVERR, or DECERR response. This fault is a precise abort."]
    #[inline(always)]
    pub fn dmac_ftr0_bits_5(&self) -> DmacFtr0Bits5R {
        DmacFtr0Bits5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates the AXI response that the DMAC receives on the BRESP bus, after the DMA channel thread performs a data write: 0 = OKAY response 1 = EXOKAY, SLVERR, or DECERR response. This fault is an imprecise abort."]
    #[inline(always)]
    pub fn dmac_ftr0_bits_4(&self) -> DmacFtr0Bits4R {
        DmacFtr0Bits4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates the AXI response that the DMAC receives on the RRESP bus, after the DMA channel thread performs a data read: 0 = OKAY response 1 = EXOKAY, SLVERR, or DECERR response. This fault is an imprecise abort"]
    #[inline(always)]
    pub fn dmac_ftr0_bits_3(&self) -> DmacFtr0Bits3R {
        DmacFtr0Bits3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - If the DMA channel aborts, this bit indicates if the erroneous instruction was read from the system memory or from the debug interface: 0 = instruction that generated an abort was read from system memory 1 = instruction that generated an abort was read from the debug interface. This fault is an imprecise abort but the bit is only valid when a precise abort occurs."]
    #[inline(always)]
    pub fn dmac_ftr0_bits_1(&self) -> DmacFtr0Bits1R {
        DmacFtr0Bits1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates if the DMA channel has locked-up because of resource starvation: 0 = DMA channel has adequate resources 1 = DMA channel has locked-up because of insufficient resources. This fault is an imprecise abort"]
    #[inline(always)]
    pub fn dmac_ftr0_bits_0(&self) -> DmacFtr0Bits0R {
        DmacFtr0Bits0R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Fault Type DMA Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_ftr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacFtr0Spec;
impl crate::RegisterSpec for DmacFtr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_ftr0::R`](R) reader structure"]
impl crate::Readable for DmacFtr0Spec {}
#[doc = "`reset()` method sets DMAC_FTR0 to value 0"]
impl crate::Resettable for DmacFtr0Spec {
    const RESET_VALUE: u32 = 0;
}
