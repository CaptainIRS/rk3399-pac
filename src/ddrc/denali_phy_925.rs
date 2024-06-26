#[doc = "Register `DENALI_PHY_925` reader"]
pub type R = crate::R<DenaliPhy925Spec>;
#[doc = "Register `DENALI_PHY_925` writer"]
pub type W = crate::W<DenaliPhy925Spec>;
#[doc = "Field `PHY_PAD_FDBK_DRIVE2` reader - Controls drive settings (enslice/ boost) for gate feedback pads."]
pub type PhyPadFdbkDrive2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_FDBK_DRIVE2` writer - Controls drive settings (enslice/ boost) for gate feedback pads."]
pub type PhyPadFdbkDrive2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Controls drive settings (enslice/ boost) for gate feedback pads."]
    #[inline(always)]
    pub fn phy_pad_fdbk_drive2(&self) -> PhyPadFdbkDrive2R {
        PhyPadFdbkDrive2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Controls drive settings (enslice/ boost) for gate feedback pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_fdbk_drive2(&mut self) -> PhyPadFdbkDrive2W<DenaliPhy925Spec> {
        PhyPadFdbkDrive2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_925::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_925::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy925Spec;
impl crate::RegisterSpec for DenaliPhy925Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_925::R`](R) reader structure"]
impl crate::Readable for DenaliPhy925Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_925::W`](W) writer structure"]
impl crate::Writable for DenaliPhy925Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_925 to value 0xff"]
impl crate::Resettable for DenaliPhy925Spec {
    const RESET_VALUE: u32 = 0xff;
}
