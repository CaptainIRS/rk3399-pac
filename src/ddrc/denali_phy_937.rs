#[doc = "Register `DENALI_PHY_937` reader"]
pub type R = crate::R<DenaliPhy937Spec>;
#[doc = "Register `DENALI_PHY_937` writer"]
pub type W = crate::W<DenaliPhy937Spec>;
#[doc = "Field `PHY_PAD_RST_DRIVE` reader - 0x0-0x1fffffff Controls drive settings for reset_n pads."]
pub type PhyPadRstDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_RST_DRIVE` writer - 0x0-0x1fffffff Controls drive settings for reset_n pads."]
pub type PhyPadRstDriveW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 0x0-0x1fffffff Controls drive settings for reset_n pads."]
    #[inline(always)]
    pub fn phy_pad_rst_drive(&self) -> PhyPadRstDriveR {
        PhyPadRstDriveR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 0x0-0x1fffffff Controls drive settings for reset_n pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_rst_drive(&mut self) -> PhyPadRstDriveW<DenaliPhy937Spec> {
        PhyPadRstDriveW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_937::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_937::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy937Spec;
impl crate::RegisterSpec for DenaliPhy937Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_937::R`](R) reader structure"]
impl crate::Readable for DenaliPhy937Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_937::W`](W) writer structure"]
impl crate::Writable for DenaliPhy937Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_937 to value 0x0f"]
impl crate::Resettable for DenaliPhy937Spec {
    const RESET_VALUE: u32 = 0x0f;
}
