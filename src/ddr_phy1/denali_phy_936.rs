#[doc = "Register `DENALI_PHY_936` reader"]
pub type R = crate::R<DenaliPhy936Spec>;
#[doc = "Register `DENALI_PHY_936` writer"]
pub type W = crate::W<DenaliPhy936Spec>;
#[doc = "Field `PHY_PAD_CKE_TERM` reader - Controls term settings for cke pads."]
pub type PhyPadCkeTermR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_CKE_TERM` writer - Controls term settings for cke pads."]
pub type PhyPadCkeTermW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Controls term settings for cke pads."]
    #[inline(always)]
    pub fn phy_pad_cke_term(&self) -> PhyPadCkeTermR {
        PhyPadCkeTermR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Controls term settings for cke pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_cke_term(&mut self) -> PhyPadCkeTermW<DenaliPhy936Spec> {
        PhyPadCkeTermW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_936::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_936::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy936Spec;
impl crate::RegisterSpec for DenaliPhy936Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_936::R`](R) reader structure"]
impl crate::Readable for DenaliPhy936Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_936::W`](W) writer structure"]
impl crate::Writable for DenaliPhy936Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_936 to value 0x4410"]
impl crate::Resettable for DenaliPhy936Spec {
    const RESET_VALUE: u32 = 0x4410;
}
