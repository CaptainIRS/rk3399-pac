#[doc = "Register `DENALI_PHY_940` reader"]
pub type R = crate::R<DenaliPhy940Spec>;
#[doc = "Register `DENALI_PHY_940` writer"]
pub type W = crate::W<DenaliPhy940Spec>;
#[doc = "Field `PHY_PAD_CS_TERM` reader - Controls term settings for cs pads."]
pub type PhyPadCsTermR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_CS_TERM` writer - Controls term settings for cs pads."]
pub type PhyPadCsTermW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Controls term settings for cs pads."]
    #[inline(always)]
    pub fn phy_pad_cs_term(&self) -> PhyPadCsTermR {
        PhyPadCsTermR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Controls term settings for cs pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_cs_term(&mut self) -> PhyPadCsTermW<DenaliPhy940Spec> {
        PhyPadCsTermW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_940::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_940::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy940Spec;
impl crate::RegisterSpec for DenaliPhy940Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_940::R`](R) reader structure"]
impl crate::Readable for DenaliPhy940Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_940::W`](W) writer structure"]
impl crate::Writable for DenaliPhy940Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_940 to value 0x4410"]
impl crate::Resettable for DenaliPhy940Spec {
    const RESET_VALUE: u32 = 0x4410;
}
