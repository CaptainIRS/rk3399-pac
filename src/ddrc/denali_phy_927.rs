#[doc = "Register `DENALI_PHY_927` reader"]
pub type R = crate::R<DenaliPhy927Spec>;
#[doc = "Register `DENALI_PHY_927` writer"]
pub type W = crate::W<DenaliPhy927Spec>;
#[doc = "Field `PHY_PAD_DQS_DRIVE` reader - Controls drive settings for dqs pads."]
pub type PhyPadDqsDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_DQS_DRIVE` writer - Controls drive settings for dqs pads."]
pub type PhyPadDqsDriveW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - Controls drive settings for dqs pads."]
    #[inline(always)]
    pub fn phy_pad_dqs_drive(&self) -> PhyPadDqsDriveR {
        PhyPadDqsDriveR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - Controls drive settings for dqs pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_dqs_drive(&mut self) -> PhyPadDqsDriveW<DenaliPhy927Spec> {
        PhyPadDqsDriveW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_927::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_927::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy927Spec;
impl crate::RegisterSpec for DenaliPhy927Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_927::R`](R) reader structure"]
impl crate::Readable for DenaliPhy927Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_927::W`](W) writer structure"]
impl crate::Writable for DenaliPhy927Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_927 to value 0"]
impl crate::Resettable for DenaliPhy927Spec {
    const RESET_VALUE: u32 = 0;
}
