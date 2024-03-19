#[doc = "Register `PHY_PCLFREQ0` reader"]
pub type R = crate::R<PhyPclfreq0Spec>;
#[doc = "Register `PHY_PCLFREQ0` writer"]
pub type W = crate::W<PhyPclfreq0Spec>;
#[doc = "Field `PCLK_FREQ` reader - Pixel Clock Frequency (pclk_freq\\[7:0\\])."]
pub type PclkFreqR = crate::FieldReader;
#[doc = "Field `PCLK_FREQ` writer - Pixel Clock Frequency (pclk_freq\\[7:0\\])."]
pub type PclkFreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pixel Clock Frequency (pclk_freq\\[7:0\\])."]
    #[inline(always)]
    pub fn pclk_freq(&self) -> PclkFreqR {
        PclkFreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pixel Clock Frequency (pclk_freq\\[7:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_freq(&mut self) -> PclkFreqW<PhyPclfreq0Spec> {
        PclkFreqW::new(self, 0)
    }
}
#[doc = "PHY Test Interface Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pclfreq0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pclfreq0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyPclfreq0Spec;
impl crate::RegisterSpec for PhyPclfreq0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_pclfreq0::R`](R) reader structure"]
impl crate::Readable for PhyPclfreq0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_pclfreq0::W`](W) writer structure"]
impl crate::Writable for PhyPclfreq0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_PCLFREQ0 to value 0x32"]
impl crate::Resettable for PhyPclfreq0Spec {
    const RESET_VALUE: u8 = 0x32;
}
