#[doc = "Register `DENALI_PHY_926` reader"]
pub type R = crate::R<DenaliPhy926Spec>;
#[doc = "Register `DENALI_PHY_926` writer"]
pub type W = crate::W<DenaliPhy926Spec>;
#[doc = "Field `PHY_PAD_DATA_DRIVE` reader - Controls drive settings for data pads."]
pub type PhyPadDataDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_DATA_DRIVE` writer - Controls drive settings for data pads."]
pub type PhyPadDataDriveW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - Controls drive settings for data pads."]
    #[inline(always)]
    pub fn phy_pad_data_drive(&self) -> PhyPadDataDriveR {
        PhyPadDataDriveR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Controls drive settings for data pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_data_drive(&mut self) -> PhyPadDataDriveW<DenaliPhy926Spec> {
        PhyPadDataDriveW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_926::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_926::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy926Spec;
impl crate::RegisterSpec for DenaliPhy926Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_926::R`](R) reader structure"]
impl crate::Readable for DenaliPhy926Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_926::W`](W) writer structure"]
impl crate::Writable for DenaliPhy926Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_926 to value 0"]
impl crate::Resettable for DenaliPhy926Spec {
    const RESET_VALUE: u32 = 0;
}
