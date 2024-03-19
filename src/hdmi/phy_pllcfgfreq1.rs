#[doc = "Register `PHY_PLLCFGFREQ1` reader"]
pub type R = crate::R<PhyPllcfgfreq1Spec>;
#[doc = "Register `PHY_PLLCFGFREQ1` writer"]
pub type W = crate::W<PhyPllcfgfreq1Spec>;
#[doc = "Field `PLLCFGFREQ` reader - PLL Configuration Frequency (pllcfgfreq\\[15:8\\])."]
pub type PllcfgfreqR = crate::FieldReader;
#[doc = "Field `PLLCFGFREQ` writer - PLL Configuration Frequency (pllcfgfreq\\[15:8\\])."]
pub type PllcfgfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PLL Configuration Frequency (pllcfgfreq\\[15:8\\])."]
    #[inline(always)]
    pub fn pllcfgfreq(&self) -> PllcfgfreqR {
        PllcfgfreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLL Configuration Frequency (pllcfgfreq\\[15:8\\])."]
    #[inline(always)]
    #[must_use]
    pub fn pllcfgfreq(&mut self) -> PllcfgfreqW<PhyPllcfgfreq1Spec> {
        PllcfgfreqW::new(self, 0)
    }
}
#[doc = "PHY PLL Test Interface Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pllcfgfreq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pllcfgfreq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyPllcfgfreq1Spec;
impl crate::RegisterSpec for PhyPllcfgfreq1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_pllcfgfreq1::R`](R) reader structure"]
impl crate::Readable for PhyPllcfgfreq1Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_pllcfgfreq1::W`](W) writer structure"]
impl crate::Writable for PhyPllcfgfreq1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_PLLCFGFREQ1 to value 0x27"]
impl crate::Resettable for PhyPllcfgfreq1Spec {
    const RESET_VALUE: u8 = 0x27;
}
