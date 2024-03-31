#[doc = "Register `DENALI_PHY_933` reader"]
pub type R = crate::R<DenaliPhy933Spec>;
#[doc = "Register `DENALI_PHY_933` writer"]
pub type W = crate::W<DenaliPhy933Spec>;
#[doc = "Field `PHY_PAD_ADDR_TERM` reader - Controls term settings for the address/control pads."]
pub type PhyPadAddrTermR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_ADDR_TERM` writer - Controls term settings for the address/control pads."]
pub type PhyPadAddrTermW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Controls term settings for the address/control pads."]
    #[inline(always)]
    pub fn phy_pad_addr_term(&self) -> PhyPadAddrTermR {
        PhyPadAddrTermR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Controls term settings for the address/control pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_addr_term(&mut self) -> PhyPadAddrTermW<DenaliPhy933Spec> {
        PhyPadAddrTermW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_933::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_933::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy933Spec;
impl crate::RegisterSpec for DenaliPhy933Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_933::R`](R) reader structure"]
impl crate::Readable for DenaliPhy933Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_933::W`](W) writer structure"]
impl crate::Writable for DenaliPhy933Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_933 to value 0x4410"]
impl crate::Resettable for DenaliPhy933Spec {
    const RESET_VALUE: u32 = 0x4410;
}
