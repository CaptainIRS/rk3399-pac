#[doc = "Register `PHY_PLLCFGFREQ0` reader"]
pub type R = crate::R<PhyPllcfgfreq0Spec>;
#[doc = "Register `PHY_PLLCFGFREQ0` writer"]
pub type W = crate::W<PhyPllcfgfreq0Spec>;
#[doc = "Field `PLLCFGFREQ` reader - PLL Configuration Frequency (pllcfgfreq\\[7:0\\])."]
pub type PllcfgfreqR = crate::FieldReader;
#[doc = "Field `PLLCFGFREQ` writer - PLL Configuration Frequency (pllcfgfreq\\[7:0\\])."]
pub type PllcfgfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PLL Configuration Frequency (pllcfgfreq\\[7:0\\])."]
    #[inline(always)]
    pub fn pllcfgfreq(&self) -> PllcfgfreqR {
        PllcfgfreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLL Configuration Frequency (pllcfgfreq\\[7:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn pllcfgfreq(&mut self) -> PllcfgfreqW<PhyPllcfgfreq0Spec> {
        PllcfgfreqW::new(self, 0)
    }
}
#[doc = "PLL Configuration Frequency (pllcfgfreq\\[7:0\\]).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pllcfgfreq0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pllcfgfreq0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyPllcfgfreq0Spec;
impl crate::RegisterSpec for PhyPllcfgfreq0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_pllcfgfreq0::R`](R) reader structure"]
impl crate::Readable for PhyPllcfgfreq0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_pllcfgfreq0::W`](W) writer structure"]
impl crate::Writable for PhyPllcfgfreq0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_PLLCFGFREQ0 to value 0x20"]
impl crate::Resettable for PhyPllcfgfreq0Spec {
    const RESET_VALUE: u8 = 0x20;
}
