#[doc = "Register `DDR_DENALI_PHY_391` reader"]
pub type R = crate::R<DdrDenaliPhy391Spec>;
#[doc = "Register `DDR_DENALI_PHY_391` writer"]
pub type W = crate::W<DdrDenaliPhy391Spec>;
#[doc = "Field `PHY_DQS_TSEL_SELECT_3` reader - Operation type tsel select values for DQS signals for slice 3. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
pub type PhyDqsTselSelect3R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DQS_TSEL_SELECT_3` writer - Operation type tsel select values for DQS signals for slice 3. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
pub type PhyDqsTselSelect3W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PHY_TWO_CYC_PREAMBLE_3` reader - cycle preamble support for slice 3. Bit (0) controls the 2 cycle read preamble. Bit (1) controls the 2 cycle write preamble. Set each bit to 1 to enable."]
pub type PhyTwoCycPreamble3R = crate::FieldReader;
#[doc = "Field `PHY_TWO_CYC_PREAMBLE_3` writer - cycle preamble support for slice 3. Bit (0) controls the 2 cycle read preamble. Bit (1) controls the 2 cycle write preamble. Set each bit to 1 to enable."]
pub type PhyTwoCycPreamble3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - Operation type tsel select values for DQS signals for slice 3. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
    #[inline(always)]
    pub fn phy_dqs_tsel_select_3(&self) -> PhyDqsTselSelect3R {
        PhyDqsTselSelect3R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - cycle preamble support for slice 3. Bit (0) controls the 2 cycle read preamble. Bit (1) controls the 2 cycle write preamble. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_two_cyc_preamble_3(&self) -> PhyTwoCycPreamble3R {
        PhyTwoCycPreamble3R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Operation type tsel select values for DQS signals for slice 3. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_select_3(&mut self) -> PhyDqsTselSelect3W<DdrDenaliPhy391Spec> {
        PhyDqsTselSelect3W::new(self, 0)
    }
    #[doc = "Bits 24:25 - cycle preamble support for slice 3. Bit (0) controls the 2 cycle read preamble. Bit (1) controls the 2 cycle write preamble. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_two_cyc_preamble_3(&mut self) -> PhyTwoCycPreamble3W<DdrDenaliPhy391Spec> {
        PhyTwoCycPreamble3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_391::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_391::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy391Spec;
impl crate::RegisterSpec for DdrDenaliPhy391Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_391::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy391Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_391::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy391Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_391 to value 0"]
impl crate::Resettable for DdrDenaliPhy391Spec {
    const RESET_VALUE: u32 = 0;
}
