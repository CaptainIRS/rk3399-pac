#[doc = "Register `MMC_RX_INT_MSK` reader"]
pub type R = crate::R<MmcRxIntMskSpec>;
#[doc = "Register `MMC_RX_INT_MSK` writer"]
pub type W = crate::W<MmcRxIntMskSpec>;
#[doc = "Field `INT0` reader - Setting this bit masks the interrupt when the rxframecount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT0` writer - Setting this bit masks the interrupt when the rxframecount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - Setting this bit masks the interrupt when the rxoctetcount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT1` writer - Setting this bit masks the interrupt when the rxoctetcount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2` reader - Setting this bit masks the interrupt when the rxoctetcount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int2R = crate::BitReader;
#[doc = "Field `INT2` writer - Setting this bit masks the interrupt when the rxoctetcount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT4` reader - Setting this bit masks the interrupt when the\n\nrxmulticastframes_g counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
pub type Int4R = crate::BitReader;
#[doc = "Field `INT4` writer - Setting this bit masks the interrupt when the\n\nrxmulticastframes_g counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
pub type Int4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT5` reader - Setting this bit masks the interrupt when the rxcrcerror counter\n\nreaches half the maximum value, and also when it reaches the\n\nmaximum value."]
pub type Int5R = crate::BitReader;
#[doc = "Field `INT5` writer - Setting this bit masks the interrupt when the rxcrcerror counter\n\nreaches half the maximum value, and also when it reaches the\n\nmaximum value."]
pub type Int5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT18` reader - Setting this bit masks the interrupt when the rxlengtherror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int18R = crate::BitReader;
#[doc = "Field `INT18` writer - Setting this bit masks the interrupt when the rxlengtherror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT21` reader - Setting this bit masks the interrupt when the rxfifooverflow\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int21R = crate::BitReader;
#[doc = "Field `INT21` writer - Setting this bit masks the interrupt when the rxfifooverflow\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
pub type Int21W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting this bit masks the interrupt when the rxframecount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit masks the interrupt when the rxoctetcount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit masks the interrupt when the rxoctetcount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit masks the interrupt when the\n\nrxmulticastframes_g counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
    #[inline(always)]
    pub fn int4(&self) -> Int4R {
        Int4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting this bit masks the interrupt when the rxcrcerror counter\n\nreaches half the maximum value, and also when it reaches the\n\nmaximum value."]
    #[inline(always)]
    pub fn int5(&self) -> Int5R {
        Int5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 18 - Setting this bit masks the interrupt when the rxlengtherror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int18(&self) -> Int18R {
        Int18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Setting this bit masks the interrupt when the rxfifooverflow\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    pub fn int21(&self) -> Int21R {
        Int21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit masks the interrupt when the rxframecount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<MmcRxIntMskSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - Setting this bit masks the interrupt when the rxoctetcount_gb\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<MmcRxIntMskSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 2 - Setting this bit masks the interrupt when the rxoctetcount_g\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<MmcRxIntMskSpec> {
        Int2W::new(self, 2)
    }
    #[doc = "Bit 4 - Setting this bit masks the interrupt when the\n\nrxmulticastframes_g counter reaches half the maximum value,\n\nand also when it reaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> Int4W<MmcRxIntMskSpec> {
        Int4W::new(self, 4)
    }
    #[doc = "Bit 5 - Setting this bit masks the interrupt when the rxcrcerror counter\n\nreaches half the maximum value, and also when it reaches the\n\nmaximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> Int5W<MmcRxIntMskSpec> {
        Int5W::new(self, 5)
    }
    #[doc = "Bit 18 - Setting this bit masks the interrupt when the rxlengtherror\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int18(&mut self) -> Int18W<MmcRxIntMskSpec> {
        Int18W::new(self, 18)
    }
    #[doc = "Bit 21 - Setting this bit masks the interrupt when the rxfifooverflow\n\ncounter reaches half the maximum value, and also when it\n\nreaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> Int21W<MmcRxIntMskSpec> {
        Int21W::new(self, 21)
    }
}
#[doc = "MMC Receive Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_int_msk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rx_int_msk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxIntMskSpec;
impl crate::RegisterSpec for MmcRxIntMskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rx_int_msk::R`](R) reader structure"]
impl crate::Readable for MmcRxIntMskSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rx_int_msk::W`](W) writer structure"]
impl crate::Writable for MmcRxIntMskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RX_INT_MSK to value 0"]
impl crate::Resettable for MmcRxIntMskSpec {
    const RESET_VALUE: u32 = 0;
}
