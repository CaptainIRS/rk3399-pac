#[doc = "Register `DENALI_PHY_549` reader"]
pub type R = crate::R<DenaliPhy549Spec>;
#[doc = "Register `DENALI_PHY_549` writer"]
pub type W = crate::W<DenaliPhy549Spec>;
#[doc = "Field `PHY_ADR_CALVL_DLY_STEP_0` reader - Sets the delay step size plus 1 during CA training for address slice 0."]
pub type PhyAdrCalvlDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_DLY_STEP_0` writer - Sets the delay step size plus 1 during CA training for address slice 0."]
pub type PhyAdrCalvlDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Sets the delay step size plus 1 during CA training for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_dly_step_0(&self) -> PhyAdrCalvlDlyStep0R {
        PhyAdrCalvlDlyStep0R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sets the delay step size plus 1 during CA training for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_dly_step_0(&mut self) -> PhyAdrCalvlDlyStep0W<DenaliPhy549Spec> {
        PhyAdrCalvlDlyStep0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_549::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_549::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy549Spec;
impl crate::RegisterSpec for DenaliPhy549Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_549::R`](R) reader structure"]
impl crate::Readable for DenaliPhy549Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_549::W`](W) writer structure"]
impl crate::Writable for DenaliPhy549Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_549 to value 0"]
impl crate::Resettable for DenaliPhy549Spec {
    const RESET_VALUE: u32 = 0;
}
