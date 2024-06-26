#[doc = "Register `DENALI_CTL_47` reader"]
pub type R = crate::R<DenaliCtl47Spec>;
#[doc = "Register `DENALI_CTL_47` writer"]
pub type W = crate::W<DenaliCtl47Spec>;
#[doc = "Field `AREFRESH` writer - Initiate auto-refresh at the end of the current burst boundary. Set to 1 to trigger."]
pub type ArefreshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TREF_ENABLE` reader - Issue auto-refresh commands to the DRAMs at the interval defined in the TREF parameter. Set to 1 to enable."]
pub type TrefEnableR = crate::BitReader;
#[doc = "Field `TREF_ENABLE` writer - Issue auto-refresh commands to the DRAMs at the interval defined in the TREF parameter. Set to 1 to enable."]
pub type TrefEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Issue auto-refresh commands to the DRAMs at the interval defined in the TREF parameter. Set to 1 to enable."]
    #[inline(always)]
    pub fn tref_enable(&self) -> TrefEnableR {
        TrefEnableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate auto-refresh at the end of the current burst boundary. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn arefresh(&mut self) -> ArefreshW<DenaliCtl47Spec> {
        ArefreshW::new(self, 0)
    }
    #[doc = "Bit 16 - Issue auto-refresh commands to the DRAMs at the interval defined in the TREF parameter. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn tref_enable(&mut self) -> TrefEnableW<DenaliCtl47Spec> {
        TrefEnableW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl47Spec;
impl crate::RegisterSpec for DenaliCtl47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_47::R`](R) reader structure"]
impl crate::Readable for DenaliCtl47Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_47::W`](W) writer structure"]
impl crate::Writable for DenaliCtl47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_47 to value 0"]
impl crate::Resettable for DenaliCtl47Spec {
    const RESET_VALUE: u32 = 0;
}
