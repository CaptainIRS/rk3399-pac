#[doc = "Register `DENALI_PHY_900` reader"]
pub type R = crate::R<DenaliPhy900Spec>;
#[doc = "Register `DENALI_PHY_900` writer"]
pub type W = crate::W<DenaliPhy900Spec>;
#[doc = "Field `PHY_CSLVL_QTR` reader - Defines the CS training DLL quarter cycle delay value."]
pub type PhyCslvlQtrR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CSLVL_QTR` writer - Defines the CS training DLL quarter cycle delay value."]
pub type PhyCslvlQtrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CSLVL_CAPTURE_CNT` reader - Defines the number of samples to take at each GRP slave delay setting during CS training."]
pub type PhyCslvlCaptureCntR = crate::FieldReader;
#[doc = "Field `PHY_CSLVL_CAPTURE_CNT` writer - Defines the number of samples to take at each GRP slave delay setting during CS training."]
pub type PhyCslvlCaptureCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:10 - Defines the CS training DLL quarter cycle delay value."]
    #[inline(always)]
    pub fn phy_cslvl_qtr(&self) -> PhyCslvlQtrR {
        PhyCslvlQtrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - Defines the number of samples to take at each GRP slave delay setting during CS training."]
    #[inline(always)]
    pub fn phy_cslvl_capture_cnt(&self) -> PhyCslvlCaptureCntR {
        PhyCslvlCaptureCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Defines the CS training DLL quarter cycle delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_qtr(&mut self) -> PhyCslvlQtrW<DenaliPhy900Spec> {
        PhyCslvlQtrW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Defines the number of samples to take at each GRP slave delay setting during CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_capture_cnt(&mut self) -> PhyCslvlCaptureCntW<DenaliPhy900Spec> {
        PhyCslvlCaptureCntW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_900::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_900::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy900Spec;
impl crate::RegisterSpec for DenaliPhy900Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_900::R`](R) reader structure"]
impl crate::Readable for DenaliPhy900Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_900::W`](W) writer structure"]
impl crate::Writable for DenaliPhy900Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_900 to value 0"]
impl crate::Resettable for DenaliPhy900Spec {
    const RESET_VALUE: u32 = 0;
}
