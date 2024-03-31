#[doc = "Register `DENALI_CTL_16` reader"]
pub type R = crate::R<DenaliCtl16Spec>;
#[doc = "Register `DENALI_CTL_16` writer"]
pub type W = crate::W<DenaliCtl16Spec>;
#[doc = "Field `TINIT5_F2` reader - DRAM TINIT5 value for frequency copy 2 in cycles."]
pub type Tinit5F2R = crate::FieldReader<u32>;
#[doc = "Field `TINIT5_F2` writer - DRAM TINIT5 value for frequency copy 2 in cycles."]
pub type Tinit5F2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NO_AUTO_MRR_INIT` reader - Disable MRR commands during initialization. Set to 1 to disable."]
pub type NoAutoMrrInitR = crate::BitReader;
#[doc = "Field `NO_AUTO_MRR_INIT` writer - Disable MRR commands during initialization. Set to 1 to disable."]
pub type NoAutoMrrInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - DRAM TINIT5 value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tinit5_f2(&self) -> Tinit5F2R {
        Tinit5F2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Disable MRR commands during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn no_auto_mrr_init(&self) -> NoAutoMrrInitR {
        NoAutoMrrInitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - DRAM TINIT5 value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tinit5_f2(&mut self) -> Tinit5F2W<DenaliCtl16Spec> {
        Tinit5F2W::new(self, 0)
    }
    #[doc = "Bit 24 - Disable MRR commands during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn no_auto_mrr_init(&mut self) -> NoAutoMrrInitW<DenaliCtl16Spec> {
        NoAutoMrrInitW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl16Spec;
impl crate::RegisterSpec for DenaliCtl16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_16::R`](R) reader structure"]
impl crate::Readable for DenaliCtl16Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_16::W`](W) writer structure"]
impl crate::Writable for DenaliCtl16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_16 to value 0"]
impl crate::Resettable for DenaliCtl16Spec {
    const RESET_VALUE: u32 = 0;
}
