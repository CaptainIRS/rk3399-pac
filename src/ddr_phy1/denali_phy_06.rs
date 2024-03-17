#[doc = "Register `DENALI_PHY_06` reader"]
pub type R = crate::R<DenaliPhy06Spec>;
#[doc = "Register `DENALI_PHY_06` writer"]
pub type W = crate::W<DenaliPhy06Spec>;
#[doc = "Field `PHY_DQ_TSEL_SELECT_0` reader - Operation type tsel select values for DQ/DM signals for slice 0. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
pub type PhyDqTselSelect0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DQ_TSEL_SELECT_0` writer - Operation type tsel select values for DQ/DM signals for slice 0. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
pub type PhyDqTselSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PHY_DQS_TSEL_ENABLE_0` reader - Operation type tsel enables for DQS signals for slice 0. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqsTselEnable0R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_ENABLE_0` writer - Operation type tsel enables for DQS signals for slice 0. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqsTselEnable0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:23 - Operation type tsel select values for DQ/DM signals for slice 0. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
    #[inline(always)]
    pub fn phy_dq_tsel_select_0(&self) -> PhyDqTselSelect0R {
        PhyDqTselSelect0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:26 - Operation type tsel enables for DQS signals for slice 0. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_dqs_tsel_enable_0(&self) -> PhyDqsTselEnable0R {
        PhyDqsTselEnable0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Operation type tsel select values for DQ/DM signals for slice 0. Bits (3:0) are tsel_sel values during read cycles. Bits (7:4) are tsel_sel values during write cycles. Bits (11:8) are tsel_sel values during idle cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_select_0(&mut self) -> PhyDqTselSelect0W<DenaliPhy06Spec> {
        PhyDqTselSelect0W::new(self, 0)
    }
    #[doc = "Bits 24:26 - Operation type tsel enables for DQS signals for slice 0. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_enable_0(&mut self) -> PhyDqsTselEnable0W<DenaliPhy06Spec> {
        PhyDqsTselEnable0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_06::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_06::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy06Spec;
impl crate::RegisterSpec for DenaliPhy06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_06::R`](R) reader structure"]
impl crate::Readable for DenaliPhy06Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_06::W`](W) writer structure"]
impl crate::Writable for DenaliPhy06Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_06 to value 0"]
impl crate::Resettable for DenaliPhy06Spec {
    const RESET_VALUE: u32 = 0;
}
