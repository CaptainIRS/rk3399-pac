#[doc = "Register `MMC_TX_INTR` reader"]
pub type R = crate::R<MmcTxIntrSpec>;
#[doc = "Field `INT0` reader - The bit is set when the txoctetcount_gb counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT1` reader - The bit is set when the txframecount_gb counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT13` reader - The bit is set when the txunderflowerror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int13R = crate::BitReader;
#[doc = "Field `INT19` reader - The bit is set when the txcarriererror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int19R = crate::BitReader;
#[doc = "Field `INT20` reader - The bit is set when the txoctetcount_g counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int20R = crate::BitReader;
#[doc = "Field `INT21` reader - The bit is set when the txframecount_g counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int21R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is set when the txoctetcount_gb counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is set when the txframecount_gb counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit is set when the txunderflowerror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is set when the txcarriererror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int19(&self) -> Int19R {
        Int19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The bit is set when the txoctetcount_g counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int20(&self) -> Int20R {
        Int20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is set when the txframecount_g counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int21(&self) -> Int21R {
        Int21R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "MMC Transmit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_intr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTxIntrSpec;
impl crate::RegisterSpec for MmcTxIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_tx_intr::R`](R) reader structure"]
impl crate::Readable for MmcTxIntrSpec {}
#[doc = "`reset()` method sets MMC_TX_INTR to value 0"]
impl crate::Resettable for MmcTxIntrSpec {
    const RESET_VALUE: u32 = 0;
}
