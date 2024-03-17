#[doc = "Register `DENALI_CTL_07` reader"]
pub type R = crate::R<DenaliCtl07Spec>;
#[doc = "Register `DENALI_CTL_07` writer"]
pub type W = crate::W<DenaliCtl07Spec>;
#[doc = "Field `TINIT4_F0` reader - DRAM TINIT4 value for frequency copy 0 in cycles."]
pub type Tinit4F0R = crate::FieldReader<u32>;
#[doc = "Field `TINIT4_F0` writer - DRAM TINIT4 value for frequency copy 0 in cycles."]
pub type Tinit4F0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DRAM TINIT4 value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tinit4_f0(&self) -> Tinit4F0R {
        Tinit4F0R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DRAM TINIT4 value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tinit4_f0(&mut self) -> Tinit4F0W<DenaliCtl07Spec> {
        Tinit4F0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_07::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_07::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl07Spec;
impl crate::RegisterSpec for DenaliCtl07Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_07::R`](R) reader structure"]
impl crate::Readable for DenaliCtl07Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_07::W`](W) writer structure"]
impl crate::Writable for DenaliCtl07Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_07 to value 0"]
impl crate::Resettable for DenaliCtl07Spec {
    const RESET_VALUE: u32 = 0;
}
