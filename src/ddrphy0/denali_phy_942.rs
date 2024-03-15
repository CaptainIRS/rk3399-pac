#[doc = "Register `DENALI_PHY_942` reader"]
pub type R = crate::R<DenaliPhy942Spec>;
#[doc = "Register `DENALI_PHY_942` writer"]
pub type W = crate::W<DenaliPhy942Spec>;
#[doc = "Field `PHY_TST_CLK_PAD_CTRL` reader - PHY test clock pad controls."]
pub type PhyTstClkPadCtrlR = crate::FieldReader<u32>;
#[doc = "Field `PHY_TST_CLK_PAD_CTRL` writer - PHY test clock pad controls."]
pub type PhyTstClkPadCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PHY test clock pad controls."]
    #[inline(always)]
    pub fn phy_tst_clk_pad_ctrl(&self) -> PhyTstClkPadCtrlR {
        PhyTstClkPadCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PHY test clock pad controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tst_clk_pad_ctrl(&mut self) -> PhyTstClkPadCtrlW<DenaliPhy942Spec> {
        PhyTstClkPadCtrlW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_942::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_942::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy942Spec;
impl crate::RegisterSpec for DenaliPhy942Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_942::R`](R) reader structure"]
impl crate::Readable for DenaliPhy942Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_942::W`](W) writer structure"]
impl crate::Writable for DenaliPhy942Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_942 to value 0"]
impl crate::Resettable for DenaliPhy942Spec {
    const RESET_VALUE: u32 = 0;
}
