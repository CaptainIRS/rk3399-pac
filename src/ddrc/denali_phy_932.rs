#[doc = "Register `DENALI_PHY_932` reader"]
pub type R = crate::R<DenaliPhy932Spec>;
#[doc = "Register `DENALI_PHY_932` writer"]
pub type W = crate::W<DenaliPhy932Spec>;
#[doc = "Field `PHY_PAD_DQS_TERM` reader - Controls term settings for dqs pads."]
pub type PhyPadDqsTermR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_DQS_TERM` writer - Controls term settings for dqs pads."]
pub type PhyPadDqsTermW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Controls term settings for dqs pads."]
    #[inline(always)]
    pub fn phy_pad_dqs_term(&self) -> PhyPadDqsTermR {
        PhyPadDqsTermR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Controls term settings for dqs pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_dqs_term(&mut self) -> PhyPadDqsTermW<DenaliPhy932Spec> {
        PhyPadDqsTermW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_932::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_932::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy932Spec;
impl crate::RegisterSpec for DenaliPhy932Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_932::R`](R) reader structure"]
impl crate::Readable for DenaliPhy932Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_932::W`](W) writer structure"]
impl crate::Writable for DenaliPhy932Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_932 to value 0x4410"]
impl crate::Resettable for DenaliPhy932Spec {
    const RESET_VALUE: u32 = 0x4410;
}
