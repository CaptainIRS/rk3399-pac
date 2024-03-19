#[doc = "Register `DDR_DENALI_PHY_924` reader"]
pub type R = crate::R<DdrDenaliPhy924Spec>;
#[doc = "Register `DDR_DENALI_PHY_924` writer"]
pub type W = crate::W<DdrDenaliPhy924Spec>;
#[doc = "Field `PHY_PAD_FDBK_DRIVE` reader - Controls drive settings for gate feedback pads."]
pub type PhyPadFdbkDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_FDBK_DRIVE` writer - Controls drive settings for gate feedback pads."]
pub type PhyPadFdbkDriveW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - Controls drive settings for gate feedback pads."]
    #[inline(always)]
    pub fn phy_pad_fdbk_drive(&self) -> PhyPadFdbkDriveR {
        PhyPadFdbkDriveR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - Controls drive settings for gate feedback pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_fdbk_drive(&mut self) -> PhyPadFdbkDriveW<DdrDenaliPhy924Spec> {
        PhyPadFdbkDriveW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_924::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_924::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy924Spec;
impl crate::RegisterSpec for DdrDenaliPhy924Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_924::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy924Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_924::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy924Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_924 to value 0xff"]
impl crate::Resettable for DdrDenaliPhy924Spec {
    const RESET_VALUE: u32 = 0xff;
}
