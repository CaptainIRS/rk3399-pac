#[doc = "Register `DENALI_PHY_943` reader"]
pub type R = crate::R<DenaliPhy943Spec>;
#[doc = "Register `DENALI_PHY_943` writer"]
pub type W = crate::W<DenaliPhy943Spec>;
#[doc = "Field `PHY_TST_CLK_PAD_CTRL2` reader - PHY test clock pad additional controls."]
pub type PhyTstClkPadCtrl2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_TST_CLK_PAD_CTRL2` writer - PHY test clock pad additional controls."]
pub type PhyTstClkPadCtrl2W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - PHY test clock pad additional controls."]
    #[inline(always)]
    pub fn phy_tst_clk_pad_ctrl2(&self) -> PhyTstClkPadCtrl2R {
        PhyTstClkPadCtrl2R::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - PHY test clock pad additional controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tst_clk_pad_ctrl2(&mut self) -> PhyTstClkPadCtrl2W<DenaliPhy943Spec> {
        PhyTstClkPadCtrl2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_943::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_943::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy943Spec;
impl crate::RegisterSpec for DenaliPhy943Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_943::R`](R) reader structure"]
impl crate::Readable for DenaliPhy943Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_943::W`](W) writer structure"]
impl crate::Writable for DenaliPhy943Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_943 to value 0"]
impl crate::Resettable for DenaliPhy943Spec {
    const RESET_VALUE: u32 = 0;
}
