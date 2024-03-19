#[doc = "Register `PHY_PLLCFGFREQ2` reader"]
pub type R = crate::R<PhyPllcfgfreq2Spec>;
#[doc = "Register `PHY_PLLCFGFREQ2` writer"]
pub type W = crate::W<PhyPllcfgfreq2Spec>;
#[doc = "Field `PLLCFGFREQ` reader - PLL Configuration Frequency (pllcfgfreq\\[23:16\\])."]
pub type PllcfgfreqR = crate::FieldReader;
#[doc = "Field `PLLCFGFREQ` writer - PLL Configuration Frequency (pllcfgfreq\\[23:16\\])."]
pub type PllcfgfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PLL Configuration Frequency (pllcfgfreq\\[23:16\\])."]
    #[inline(always)]
    pub fn pllcfgfreq(&self) -> PllcfgfreqR {
        PllcfgfreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLL Configuration Frequency (pllcfgfreq\\[23:16\\])."]
    #[inline(always)]
    #[must_use]
    pub fn pllcfgfreq(&mut self) -> PllcfgfreqW<PhyPllcfgfreq2Spec> {
        PllcfgfreqW::new(self, 0)
    }
}
#[doc = "PHY PLL Test Interface Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pllcfgfreq2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pllcfgfreq2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyPllcfgfreq2Spec;
impl crate::RegisterSpec for PhyPllcfgfreq2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_pllcfgfreq2::R`](R) reader structure"]
impl crate::Readable for PhyPllcfgfreq2Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_pllcfgfreq2::W`](W) writer structure"]
impl crate::Writable for PhyPllcfgfreq2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_PLLCFGFREQ2 to value 0"]
impl crate::Resettable for PhyPllcfgfreq2Spec {
    const RESET_VALUE: u8 = 0;
}
