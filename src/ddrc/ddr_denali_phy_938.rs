#[doc = "Register `DDR_DENALI_PHY_938` reader"]
pub type R = crate::R<DdrDenaliPhy938Spec>;
#[doc = "Register `DDR_DENALI_PHY_938` writer"]
pub type W = crate::W<DdrDenaliPhy938Spec>;
#[doc = "Field `PHY_PAD_RST_TERM` reader - Controls term settings for reset_n pads."]
pub type PhyPadRstTermR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_RST_TERM` writer - Controls term settings for reset_n pads."]
pub type PhyPadRstTermW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Controls term settings for reset_n pads."]
    #[inline(always)]
    pub fn phy_pad_rst_term(&self) -> PhyPadRstTermR {
        PhyPadRstTermR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Controls term settings for reset_n pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rst_term(&mut self) -> PhyPadRstTermW<DdrDenaliPhy938Spec> {
        PhyPadRstTermW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_938::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_938::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy938Spec;
impl crate::RegisterSpec for DdrDenaliPhy938Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_938::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy938Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_938::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy938Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_938 to value 0x4410"]
impl crate::Resettable for DdrDenaliPhy938Spec {
    const RESET_VALUE: u32 = 0x4410;
}
