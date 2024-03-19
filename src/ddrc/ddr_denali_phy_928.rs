#[doc = "Register `DDR_DENALI_PHY_928` reader"]
pub type R = crate::R<DdrDenaliPhy928Spec>;
#[doc = "Register `DDR_DENALI_PHY_928` writer"]
pub type W = crate::W<DdrDenaliPhy928Spec>;
#[doc = "Field `PHY_PAD_ADDR_DRIVE` reader - 0x0-0x1fffffff Controls drive settings for the address/control pads."]
pub type PhyPadAddrDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_ADDR_DRIVE` writer - 0x0-0x1fffffff Controls drive settings for the address/control pads."]
pub type PhyPadAddrDriveW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 0x0-0x1fffffff Controls drive settings for the address/control pads."]
    #[inline(always)]
    pub fn phy_pad_addr_drive(&self) -> PhyPadAddrDriveR {
        PhyPadAddrDriveR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 0x0-0x1fffffff Controls drive settings for the address/control pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_addr_drive(&mut self) -> PhyPadAddrDriveW<DdrDenaliPhy928Spec> {
        PhyPadAddrDriveW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_928::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_928::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy928Spec;
impl crate::RegisterSpec for DdrDenaliPhy928Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_928::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy928Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_928::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy928Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_928 to value 0x0f"]
impl crate::Resettable for DdrDenaliPhy928Spec {
    const RESET_VALUE: u32 = 0x0f;
}
