#[doc = "Register `DENALI_PHY_929` reader"]
pub type R = crate::R<DenaliPhy929Spec>;
#[doc = "Register `DENALI_PHY_929` writer"]
pub type W = crate::W<DenaliPhy929Spec>;
#[doc = "Field `PHY_PAD_CLK_DRIVE` reader - 0x0-0x7fffffff Controls drive settings for clock pads."]
pub type PhyPadClkDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_CLK_DRIVE` writer - 0x0-0x7fffffff Controls drive settings for clock pads."]
pub type PhyPadClkDriveW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - 0x0-0x7fffffff Controls drive settings for clock pads."]
    #[inline(always)]
    pub fn phy_pad_clk_drive(&self) -> PhyPadClkDriveR {
        PhyPadClkDriveR::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - 0x0-0x7fffffff Controls drive settings for clock pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_clk_drive(&mut self) -> PhyPadClkDriveW<DenaliPhy929Spec> {
        PhyPadClkDriveW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_929::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_929::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy929Spec;
impl crate::RegisterSpec for DenaliPhy929Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_929::R`](R) reader structure"]
impl crate::Readable for DenaliPhy929Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_929::W`](W) writer structure"]
impl crate::Writable for DenaliPhy929Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_929 to value 0x0f"]
impl crate::Resettable for DenaliPhy929Spec {
    const RESET_VALUE: u32 = 0x0f;
}
