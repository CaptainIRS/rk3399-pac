#[doc = "Register `MMC_RX_INTR` reader"]
pub type R = crate::R<MmcRxIntrSpec>;
#[doc = "Register `MMC_RX_INTR` writer"]
pub type W = crate::W<MmcRxIntrSpec>;
#[doc = "Field `INT0` reader - The bit is set when the rxframecount_gb counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT1` reader - The bit is set when the rxoctetcount_gb counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT2` reader - The bit is set when the rxoctetcount_g counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int2R = crate::BitReader;
#[doc = "Field `INT4` reader - The bit is set when the rxmulticastframes_g counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int4R = crate::BitReader;
#[doc = "Field `INT5` reader - The bit is set when the rxcrcerror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
pub type Int5R = crate::BitReader;
#[doc = "Field `INT5` writer - The bit is set when the rxcrcerror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
pub type Int5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT18` reader - The bit is set when the rxlengtherror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Int18R = crate::BitReader;
#[doc = "Field `INT21` reader - The bit is set when the rxfifooverflow counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
pub type Int21R = crate::BitReader;
#[doc = "Field `INT21` writer - The bit is set when the rxfifooverflow counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
pub type Int21W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is set when the rxframecount_gb counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is set when the rxoctetcount_gb counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is set when the rxoctetcount_g counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is set when the rxmulticastframes_g counter reaches half\n\nthe maximum value, and also when it reaches the maximum\n\nvalue."]
    #[inline(always)]
    pub fn int4(&self) -> Int4R {
        Int4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is set when the rxcrcerror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is set when the rxlengtherror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int18(&self) -> Int18R {
        Int18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is set when the rxfifooverflow counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int21(&self) -> Int21R {
        Int21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - The bit is set when the rxcrcerror counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> Int5W<MmcRxIntrSpec> {
        Int5W::new(self, 5)
    }
    #[doc = "Bit 21 - The bit is set when the rxfifooverflow counter reaches half the\n\nmaximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> Int21W<MmcRxIntrSpec> {
        Int21W::new(self, 21)
    }
}
#[doc = "MMC Receive Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rx_intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxIntrSpec;
impl crate::RegisterSpec for MmcRxIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rx_intr::R`](R) reader structure"]
impl crate::Readable for MmcRxIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rx_intr::W`](W) writer structure"]
impl crate::Writable for MmcRxIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RX_INTR to value 0"]
impl crate::Resettable for MmcRxIntrSpec {
    const RESET_VALUE: u32 = 0;
}
