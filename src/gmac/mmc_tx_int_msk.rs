#[doc = "Register `MMC_TX_INT_MSK` reader"]
pub type R = crate::R<MmcTxIntMskSpec>;
#[doc = "Register `MMC_TX_INT_MSK` writer"]
pub type W = crate::W<MmcTxIntMskSpec>;
#[doc = "Field `INT0` reader - Setting this bit masks the interrupt when the txoctetcount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT0` writer - Setting this bit masks the interrupt when the txoctetcount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - Setting this bit masks the interrupt when the txframecount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT1` writer - Setting this bit masks the interrupt when the txframecount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT13` reader - Setting this bit masks the interrupt when the txunderflowerror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int13R = crate::BitReader;
#[doc = "Field `INT13` writer - Setting this bit masks the interrupt when the txunderflowerror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT19` reader - Setting this bit masks the interrupt when the txcarriererror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int19R = crate::BitReader;
#[doc = "Field `INT19` writer - Setting this bit masks the interrupt when the txcarriererror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT20` reader - Setting this bit masks the interrupt when the txoctetcount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int20R = crate::BitReader;
#[doc = "Field `INT20` writer - Setting this bit masks the interrupt when the txoctetcount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT21` reader - Setting this bit masks the interrupt when the txframecount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int21R = crate::BitReader;
#[doc = "Field `INT21` writer - Setting this bit masks the interrupt when the txframecount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int21W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting this bit masks the interrupt when the txoctetcount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit masks the interrupt when the txframecount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting this bit masks the interrupt when the txunderflowerror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int13(&self) -> Int13R {
        Int13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - Setting this bit masks the interrupt when the txcarriererror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int19(&self) -> Int19R {
        Int19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Setting this bit masks the interrupt when the txoctetcount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int20(&self) -> Int20R {
        Int20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Setting this bit masks the interrupt when the txframecount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int21(&self) -> Int21R {
        Int21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit masks the interrupt when the txoctetcount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<MmcTxIntMskSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - Setting this bit masks the interrupt when the txframecount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<MmcTxIntMskSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 13 - Setting this bit masks the interrupt when the txunderflowerror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> Int13W<MmcTxIntMskSpec> {
        Int13W::new(self, 13)
    }
    #[doc = "Bit 19 - Setting this bit masks the interrupt when the txcarriererror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int19(&mut self) -> Int19W<MmcTxIntMskSpec> {
        Int19W::new(self, 19)
    }
    #[doc = "Bit 20 - Setting this bit masks the interrupt when the txoctetcount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int20(&mut self) -> Int20W<MmcTxIntMskSpec> {
        Int20W::new(self, 20)
    }
    #[doc = "Bit 21 - Setting this bit masks the interrupt when the txframecount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> Int21W<MmcTxIntMskSpec> {
        Int21W::new(self, 21)
    }
}
#[doc = "MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_int_msk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_tx_int_msk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTxIntMskSpec;
impl crate::RegisterSpec for MmcTxIntMskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_tx_int_msk::R`](R) reader structure"]
impl crate::Readable for MmcTxIntMskSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_tx_int_msk::W`](W) writer structure"]
impl crate::Writable for MmcTxIntMskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_TX_INT_MSK to value 0"]
impl crate::Resettable for MmcTxIntMskSpec {
    const RESET_VALUE: u32 = 0;
}
