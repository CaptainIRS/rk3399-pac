#[doc = "Register `DDR_DENALI_PHY_805` reader"]
pub type R = crate::R<DdrDenaliPhy805Spec>;
#[doc = "Register `DDR_DENALI_PHY_805` writer"]
pub type W = crate::W<DdrDenaliPhy805Spec>;
#[doc = "Field `PHY_ADR_CALVL_DLY_STEP_2` reader - Sets the delay step size plus 1 during CA training for address slice 2."]
pub type PhyAdrCalvlDlyStep2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_DLY_STEP_2` writer - Sets the delay step size plus 1 during CA training for address slice 2."]
pub type PhyAdrCalvlDlyStep2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Sets the delay step size plus 1 during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_dly_step_2(&self) -> PhyAdrCalvlDlyStep2R {
        PhyAdrCalvlDlyStep2R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sets the delay step size plus 1 during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_dly_step_2(&mut self) -> PhyAdrCalvlDlyStep2W<DdrDenaliPhy805Spec> {
        PhyAdrCalvlDlyStep2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_805::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_805::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy805Spec;
impl crate::RegisterSpec for DdrDenaliPhy805Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_805::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy805Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_805::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy805Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_805 to value 0"]
impl crate::Resettable for DdrDenaliPhy805Spec {
    const RESET_VALUE: u32 = 0;
}
