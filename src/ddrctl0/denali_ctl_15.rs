#[doc = "Register `DENALI_CTL_15` reader"]
pub type R = crate::R<DenaliCtl15Spec>;
#[doc = "Register `DENALI_CTL_15` writer"]
pub type W = crate::W<DenaliCtl15Spec>;
#[doc = "Field `TINIT4_F2` reader - DRAM TINIT4 value for frequency copy 2 in cycles."]
pub type Tinit4F2R = crate::FieldReader<u32>;
#[doc = "Field `TINIT4_F2` writer - DRAM TINIT4 value for frequency copy 2 in cycles."]
pub type Tinit4F2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DRAM TINIT4 value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tinit4_f2(&self) -> Tinit4F2R {
        Tinit4F2R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DRAM TINIT4 value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tinit4_f2(&mut self) -> Tinit4F2W<DenaliCtl15Spec> {
        Tinit4F2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl15Spec;
impl crate::RegisterSpec for DenaliCtl15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_15::R`](R) reader structure"]
impl crate::Readable for DenaliCtl15Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_15::W`](W) writer structure"]
impl crate::Writable for DenaliCtl15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_15 to value 0"]
impl crate::Resettable for DenaliCtl15Spec {
    const RESET_VALUE: u32 = 0;
}
