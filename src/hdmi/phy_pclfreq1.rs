#[doc = "Register `PHY_PCLFREQ1` reader"]
pub type R = crate::R<PhyPclfreq1Spec>;
#[doc = "Register `PHY_PCLFREQ1` writer"]
pub type W = crate::W<PhyPclfreq1Spec>;
#[doc = "Field `PCLK_FREQ` reader - Pixel Clock Frequency (pclk_freq\\[9:8\\])."]
pub type PclkFreqR = crate::FieldReader;
#[doc = "Field `PCLK_FREQ` writer - Pixel Clock Frequency (pclk_freq\\[9:8\\])."]
pub type PclkFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Pixel Clock Frequency (pclk_freq\\[9:8\\])."]
    #[inline(always)]
    pub fn pclk_freq(&self) -> PclkFreqR {
        PclkFreqR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pixel Clock Frequency (pclk_freq\\[9:8\\])."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_freq(&mut self) -> PclkFreqW<PhyPclfreq1Spec> {
        PclkFreqW::new(self, 0)
    }
}
#[doc = "Pixel Clock Frequency (pclk_freq\\[9:8\\]).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pclfreq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pclfreq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyPclfreq1Spec;
impl crate::RegisterSpec for PhyPclfreq1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_pclfreq1::R`](R) reader structure"]
impl crate::Readable for PhyPclfreq1Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_pclfreq1::W`](W) writer structure"]
impl crate::Writable for PhyPclfreq1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_PCLFREQ1 to value 0"]
impl crate::Resettable for PhyPclfreq1Spec {
    const RESET_VALUE: u8 = 0;
}
