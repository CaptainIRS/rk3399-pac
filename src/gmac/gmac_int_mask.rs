#[doc = "Register `GMAC_INT_MASK` reader"]
pub type R = crate::R<GmacIntMaskSpec>;
#[doc = "Register `GMAC_INT_MASK` writer"]
pub type W = crate::W<GmacIntMaskSpec>;
#[doc = "Field `RIM` reader - RGMII Interrupt Mask This bit when set, will disable the assertion of the interrupt signal due to the setting of RGMII Interrupt Status bit in Register GMAC_INT_STATUS."]
pub type RimR = crate::BitReader;
#[doc = "Field `RIM` writer - RGMII Interrupt Mask This bit when set, will disable the assertion of the interrupt signal due to the setting of RGMII Interrupt Status bit in Register GMAC_INT_STATUS."]
pub type RimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIM` reader - PMT Interrupt Mask This bit when set, will disable the assertion of the interrupt signal due to the setting of PMT Interrupt Status bit in Register GMAC_INT_STATUS."]
pub type PimR = crate::BitReader;
#[doc = "Field `PIM` writer - PMT Interrupt Mask This bit when set, will disable the assertion of the interrupt signal due to the setting of PMT Interrupt Status bit in Register GMAC_INT_STATUS."]
pub type PimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RGMII Interrupt Mask This bit when set, will disable the assertion of the interrupt signal due to the setting of RGMII Interrupt Status bit in Register GMAC_INT_STATUS."]
    #[inline(always)]
    pub fn rim(&self) -> RimR {
        RimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask This bit when set, will disable the assertion of the interrupt signal due to the setting of PMT Interrupt Status bit in Register GMAC_INT_STATUS."]
    #[inline(always)]
    pub fn pim(&self) -> PimR {
        PimR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RGMII Interrupt Mask This bit when set, will disable the assertion of the interrupt signal due to the setting of RGMII Interrupt Status bit in Register GMAC_INT_STATUS."]
    #[inline(always)]
    #[must_use]
    pub fn rim(&mut self) -> RimW<GmacIntMaskSpec> {
        RimW::new(self, 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask This bit when set, will disable the assertion of the interrupt signal due to the setting of PMT Interrupt Status bit in Register GMAC_INT_STATUS."]
    #[inline(always)]
    #[must_use]
    pub fn pim(&mut self) -> PimW<GmacIntMaskSpec> {
        PimW::new(self, 3)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_int_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_int_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacIntMaskSpec;
impl crate::RegisterSpec for GmacIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_int_mask::R`](R) reader structure"]
impl crate::Readable for GmacIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_int_mask::W`](W) writer structure"]
impl crate::Writable for GmacIntMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_INT_MASK to value 0"]
impl crate::Resettable for GmacIntMaskSpec {
    const RESET_VALUE: u32 = 0;
}
